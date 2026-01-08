use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::thread;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, TrayIcon},
    Manager, WindowEvent, State, Emitter, WebviewWindowBuilder, WebviewUrl, AppHandle,
};
use tauri_plugin_notification::NotificationExt;
use url::form_urlencoded;

// ============= 跨平台空闲检测 =============

/// 获取系统空闲时间（秒）
/// Windows: 使用 GetLastInputInfo
/// macOS: 使用 CGEventSourceSecondsSinceLastEventType
/// Linux: 使用 X11 screensaver extension
fn get_idle_seconds() -> u64 {
    #[cfg(target_os = "windows")]
    {
        get_idle_seconds_windows()
    }

    #[cfg(target_os = "macos")]
    {
        get_idle_seconds_macos()
    }

    #[cfg(target_os = "linux")]
    {
        get_idle_seconds_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        0 // 不支持的平台返回 0
    }
}

#[cfg(target_os = "windows")]
fn get_idle_seconds_windows() -> u64 {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    use windows::Win32::System::SystemInformation::GetTickCount;

    unsafe {
        let mut lii = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        if GetLastInputInfo(&mut lii).as_bool() {
            let current_tick = GetTickCount();
            let idle_ms = current_tick.wrapping_sub(lii.dwTime);
            (idle_ms / 1000) as u64
        } else {
            0
        }
    }
}

#[cfg(target_os = "macos")]
fn get_idle_seconds_macos() -> u64 {
    use std::process::Command;

    // 使用 ioreg 命令获取空闲时间（更可靠的方式）
    let output = Command::new("ioreg")
        .args(["-c", "IOHIDSystem"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // 查找 HIDIdleTime 字段
        for line in stdout.lines() {
            if line.contains("HIDIdleTime") {
                // 格式: "HIDIdleTime" = 1234567890
                if let Some(value) = line.split('=').nth(1) {
                    if let Ok(ns) = value.trim().parse::<u64>() {
                        return ns / 1_000_000_000; // 纳秒转秒
                    }
                }
            }
        }
    }
    0
}

#[cfg(target_os = "linux")]
fn get_idle_seconds_linux() -> u64 {
    use x11::xlib::{XOpenDisplay, XCloseDisplay, XDefaultRootWindow};
    use x11::xss::{XScreenSaverQueryInfo, XScreenSaverAllocInfo};
    use std::ptr;

    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            return 0;
        }

        let info = XScreenSaverAllocInfo();
        if info.is_null() {
            XCloseDisplay(display);
            return 0;
        }

        let root = XDefaultRootWindow(display);
        let result = XScreenSaverQueryInfo(display, root, info);

        let idle_ms = if result != 0 {
            (*info).idle
        } else {
            0
        };

        x11::xlib::XFree(info as *mut _);
        XCloseDisplay(display);

        (idle_ms / 1000) as u64
    }
}

struct TrayState(Mutex<Option<TrayIcon>>);
struct LockState(Mutex<Vec<String>>);
struct PauseMenuState(Mutex<Option<MenuItem<tauri::Wry>>>);

// ============= 后端定时器系统 =============

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct TaskConfig {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub interval: u64,  // 分钟
    pub enabled: bool,
    pub icon: String,
    #[serde(default)]
    pub auto_reset_on_idle: bool,  // 空闲时自动重置
}

#[derive(Clone, Debug)]
struct TaskTimer {
    config: TaskConfig,
    reset_time: Instant,
    triggered: bool,  // 本轮是否已触发
    disabled_at: Option<Instant>,  // 禁用时的时间点，用于计算暂停时长
}

struct TimerState {
    tasks: HashMap<String, TaskTimer>,
    paused: bool,
    pause_start: Option<Instant>,
    system_locked: bool,
    lock_screen_active: bool,
    lock_screen_start: Option<Instant>,  // 锁屏开始时间，用于补偿
    // 空闲检测相关
    idle_threshold_seconds: u64,  // 空闲阈值（秒），默认 300 秒 = 5 分钟
    is_idle: bool,  // 当前是否处于空闲状态
    idle_start: Option<Instant>,  // 进入空闲状态的时间点
}

impl TimerState {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            paused: false,
            pause_start: None,
            system_locked: false,
            lock_screen_active: false,
            lock_screen_start: None,
            idle_threshold_seconds: 300,  // 默认 5 分钟
            is_idle: false,
            idle_start: None,
        }
    }
}

static TIMER_STATE: std::sync::OnceLock<Mutex<TimerState>> = std::sync::OnceLock::new();

fn get_timer_state() -> &'static Mutex<TimerState> {
    TIMER_STATE.get_or_init(|| Mutex::new(TimerState::new()))
}

#[cfg(target_os = "windows")]
static SYSTEM_LOCKED: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "windows")]
fn start_session_monitor(app_handle: tauri::AppHandle) {
    use windows::Win32::System::RemoteDesktop::{
        WTSRegisterSessionNotification, NOTIFY_FOR_THIS_SESSION,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DispatchMessageW, GetMessageW, RegisterClassW,
        TranslateMessage, CS_HREDRAW, CS_VREDRAW, MSG, WINDOW_EX_STYLE, WNDCLASSW, WS_OVERLAPPED,
        WM_WTSSESSION_CHANGE,
    };
    use windows::Win32::Foundation::HWND;
    use windows::core::{PCWSTR, w};

    const WTS_SESSION_LOCK: u32 = 0x7;
    const WTS_SESSION_UNLOCK: u32 = 0x8;

    std::thread::spawn(move || {
        unsafe {
            let class_name = w!("DeskReminderSessionMonitor");

            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(session_wnd_proc),
                hInstance: std::mem::zeroed(),
                lpszClassName: class_name,
                ..std::mem::zeroed()
            };

            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                PCWSTR::null(),
                WS_OVERLAPPED,
                0, 0, 0, 0,
                HWND::default(),
                None,
                None,
                None,
            ).unwrap_or(HWND::default());

            if hwnd.0 != std::ptr::null_mut() {
                let _ = WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION);

                let mut msg = MSG::default();
                while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
                    if msg.message == WM_WTSSESSION_CHANGE {
                        let wparam = msg.wParam.0 as u32;
                        if wparam == WTS_SESSION_LOCK {
                            SYSTEM_LOCKED.store(true, Ordering::SeqCst);
                            let _ = app_handle.emit("system-locked", ());
                        } else if wparam == WTS_SESSION_UNLOCK {
                            SYSTEM_LOCKED.store(false, Ordering::SeqCst);
                            let _ = app_handle.emit("system-unlocked", ());
                        }
                    }
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    });
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn session_wnd_proc(
    hwnd: windows::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows::Win32::Foundation::WPARAM,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LockTaskArgs {
    title: String,
    desc: String,
    duration: i32,
    icon: String,
}

// ============= 定时器命令 =============

#[derive(Clone, serde::Serialize)]
struct CountdownInfo {
    id: String,
    remaining: u64,  // 剩余秒数
    total: u64,      // 总秒数
    enabled: bool,
}

#[derive(Clone, serde::Serialize)]
struct TaskTriggeredPayload {
    id: String,
    title: String,
    desc: String,
    icon: String,
}

#[tauri::command]
fn sync_tasks(tasks: Vec<TaskConfig>) {
    let mut state = get_timer_state().lock().unwrap();
    let now = Instant::now();

    // 保留现有任务的计时状态，只更新配置
    let mut new_tasks: HashMap<String, TaskTimer> = HashMap::new();

    for task in tasks {
        if let Some(existing) = state.tasks.get(&task.id) {
            // 任务已存在
            let interval_changed = existing.config.interval != task.interval;
            let was_disabled = !existing.config.enabled;
            let is_now_enabled = task.enabled;
            let was_enabled = existing.config.enabled;
            let is_now_disabled = !task.enabled;

            if interval_changed {
                // interval 变了，重置计时
                new_tasks.insert(task.id.clone(), TaskTimer {
                    config: task,
                    reset_time: now,
                    triggered: false,
                    disabled_at: None,
                });
            } else if was_disabled && is_now_enabled {
                // 从禁用变为启用，补偿禁用期间的时间
                let mut new_reset_time = existing.reset_time;
                if let Some(disabled_at) = existing.disabled_at {
                    let disabled_duration = now.duration_since(disabled_at);
                    new_reset_time += disabled_duration;
                }
                new_tasks.insert(task.id.clone(), TaskTimer {
                    config: task,
                    reset_time: new_reset_time,
                    triggered: existing.triggered,
                    disabled_at: None,
                });
            } else if was_enabled && is_now_disabled {
                // 从启用变为禁用，记录禁用时间点
                new_tasks.insert(task.id.clone(), TaskTimer {
                    config: task,
                    reset_time: existing.reset_time,
                    triggered: existing.triggered,
                    disabled_at: Some(now),
                });
            } else {
                // 状态没变，保留
                new_tasks.insert(task.id.clone(), TaskTimer {
                    config: task,
                    reset_time: existing.reset_time,
                    triggered: existing.triggered,
                    disabled_at: existing.disabled_at,
                });
            }
        } else {
            // 新任务
            new_tasks.insert(task.id.clone(), TaskTimer {
                config: task.clone(),
                reset_time: now,
                triggered: false,
                disabled_at: if task.enabled { None } else { Some(now) },
            });
        }
    }

    state.tasks = new_tasks;
}

#[tauri::command]
fn timer_pause() {
    let mut state = get_timer_state().lock().unwrap();
    if !state.paused {
        state.paused = true;
        state.pause_start = Some(Instant::now());
    }
}

#[tauri::command]
fn timer_resume() {
    let mut state = get_timer_state().lock().unwrap();
    if state.paused {
        if let Some(pause_start) = state.pause_start {
            let pause_duration = pause_start.elapsed();
            // 补偿暂停时间
            for timer in state.tasks.values_mut() {
                timer.reset_time += pause_duration;
            }
        }
        state.paused = false;
        state.pause_start = None;
    }
}

#[tauri::command]
fn timer_reset_task(task_id: String) {
    let mut state = get_timer_state().lock().unwrap();
    let now = Instant::now();
    if let Some(timer) = state.tasks.get_mut(&task_id) {
        timer.reset_time = now;
        timer.triggered = false;
        // 如果任务禁用，也更新 disabled_at
        if timer.disabled_at.is_some() {
            timer.disabled_at = Some(now);
        }
    }
}

#[tauri::command]
fn timer_reset_all() {
    let mut state = get_timer_state().lock().unwrap();
    let now = Instant::now();
    for timer in state.tasks.values_mut() {
        timer.reset_time = now;
        timer.triggered = false;
        // 如果任务禁用，也更新 disabled_at
        if timer.disabled_at.is_some() {
            timer.disabled_at = Some(now);
        }
    }
}

#[tauri::command]
fn timer_snooze_task(task_id: String, minutes: u64) {
    let mut state = get_timer_state().lock().unwrap();
    if let Some(timer) = state.tasks.get_mut(&task_id) {
        // 增加 reset_time，相当于推迟触发
        timer.reset_time += Duration::from_secs(minutes * 60);
        // 如果当前已经触发（triggered=true），需要重置状态以便重新计时？
        // 如果是在锁屏界面点击 snooze，说明任务已经 triggered 并且正在展示
        // 我们只需调整 reset_time，主循环会自动计算剩余时间
        // 但需要把 triggered 设为 false，否则主循环会认为它还在触发状态
        timer.triggered = false;
    }
}

#[tauri::command]
fn get_countdowns() -> Vec<CountdownInfo> {
    let state = get_timer_state().lock().unwrap();
    let now = Instant::now();

    state.tasks.values().map(|timer| {
        let total_secs = timer.config.interval * 60;

        // 如果任务被禁用，使用禁用时间点计算 elapsed，这样时间就"冻结"了
        let effective_now = if let Some(disabled_at) = timer.disabled_at {
            disabled_at
        } else {
            now
        };

        let elapsed = effective_now.saturating_duration_since(timer.reset_time).as_secs();
        let remaining = if elapsed >= total_secs { 0 } else { total_secs - elapsed };

        CountdownInfo {
            id: timer.config.id.clone(),
            remaining,
            total: total_secs,
            enabled: timer.config.enabled,
        }
    }).collect()
}

#[tauri::command]
fn timer_set_system_locked(locked: bool) {
    let mut state = get_timer_state().lock().unwrap();
    let now = Instant::now();

    if locked && !state.system_locked {
        // 刚锁屏，记录暂停时间
        state.system_locked = true;
        if state.pause_start.is_none() {
            state.pause_start = Some(now);
        }
    } else if !locked && state.system_locked {
        // 解锁
        let pause_duration = state.pause_start.map(|s| s.elapsed());

        for timer in state.tasks.values_mut() {
            if timer.config.auto_reset_on_idle {
                // 勾选了"空闲重置"，直接重置为初始值
                timer.reset_time = now;
                timer.triggered = false;
            } else if let Some(duration) = pause_duration {
                // 没有勾选，补偿暂停时间
                timer.reset_time += duration;
            }
        }

        state.system_locked = false;
        if !state.paused {
            state.pause_start = None;
        }
    }
}

#[tauri::command]
fn timer_set_lock_screen_active(active: bool) {
    let mut state = get_timer_state().lock().unwrap();
    if active && !state.lock_screen_active {
        // 刚进入锁屏模式，记录开始时间
        state.lock_screen_active = true;
        state.lock_screen_start = Some(Instant::now());
    } else if !active && state.lock_screen_active {
        // 退出锁屏模式，补偿锁屏期间的时间
        if let Some(lock_start) = state.lock_screen_start {
            let lock_duration = lock_start.elapsed();
            for timer in state.tasks.values_mut() {
                timer.reset_time += lock_duration;
            }
        }
        state.lock_screen_active = false;
        state.lock_screen_start = None;
    }
}

#[tauri::command]
fn set_idle_threshold(seconds: u64) {
    let mut state = get_timer_state().lock().unwrap();
    state.idle_threshold_seconds = seconds;
}

#[tauri::command]
fn get_idle_threshold() -> u64 {
    let state = get_timer_state().lock().unwrap();
    state.idle_threshold_seconds
}

#[derive(Clone, serde::Serialize)]
struct IdleStatus {
    is_idle: bool,
    idle_seconds: u64,
    threshold: u64,
}

fn start_timer_thread(app_handle: AppHandle) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));

            let mut tasks_to_trigger: Vec<TaskTriggeredPayload> = Vec::new();
            let mut idle_status_changed = false;
            let mut current_idle_status = IdleStatus {
                is_idle: false,
                idle_seconds: 0,
                threshold: 300,
            };

            {
                let mut state = get_timer_state().lock().unwrap();

                // 如果暂停、系统锁屏或锁屏模式激活，跳过检查
                if state.paused || state.system_locked || state.lock_screen_active {
                    continue;
                }

                let now = Instant::now();
                let idle_seconds = get_idle_seconds();
                let threshold = state.idle_threshold_seconds;
                let was_idle = state.is_idle;
                let is_now_idle = idle_seconds >= threshold;

                // 检测空闲状态变化
                if is_now_idle && !was_idle {
                    // 刚进入空闲状态
                    state.is_idle = true;
                    state.idle_start = Some(now);
                    idle_status_changed = true;

                    // 重置所有勾选了「空闲重置」的任务
                    for timer in state.tasks.values_mut() {
                        if timer.config.auto_reset_on_idle && timer.config.enabled {
                            timer.reset_time = now;
                            timer.triggered = false;
                        }
                    }
                } else if !is_now_idle && was_idle {
                    // 刚从空闲状态恢复
                    state.is_idle = false;

                    // 重新开始倒计时（从头开始）
                    for timer in state.tasks.values_mut() {
                        if timer.config.auto_reset_on_idle && timer.config.enabled {
                            timer.reset_time = now;
                            timer.triggered = false;
                        }
                    }

                    state.idle_start = None;
                    idle_status_changed = true;
                }

                current_idle_status = IdleStatus {
                    is_idle: state.is_idle,
                    idle_seconds,
                    threshold,
                };

                // 如果处于空闲状态，不检查任务触发（计时暂停）
                if state.is_idle {
                    // 空闲时不触发任何任务，但仍然发送倒计时更新
                } else {
                    // 正常检查任务触发
                    for timer in state.tasks.values_mut() {
                        if !timer.config.enabled || timer.triggered {
                            continue;
                        }

                        let elapsed = now.saturating_duration_since(timer.reset_time).as_secs();
                        let total_secs = timer.config.interval * 60;

                        if elapsed >= total_secs {
                            // 触发提醒
                            tasks_to_trigger.push(TaskTriggeredPayload {
                                id: timer.config.id.clone(),
                                title: timer.config.title.clone(),
                                desc: timer.config.desc.clone(),
                                icon: timer.config.icon.clone(),
                            });

                            // 重置计时，开始下一轮
                            timer.reset_time = now;
                            timer.triggered = false;
                        }
                    }
                }
            }

            // 发送触发事件到前端
            for task in tasks_to_trigger {
                let _ = app_handle.emit("task-triggered", task);
            }

            // 发送空闲状态更新（只在状态变化时发送，或每 5 秒发送一次状态）
            if idle_status_changed {
                let _ = app_handle.emit("idle-status-changed", current_idle_status.clone());
            }

            // 发送倒计时更新
            let countdowns = get_countdowns();
            let _ = app_handle.emit("countdown-update", countdowns);

            // ============= 锁屏看门狗 (Watchdog) =============
            // 确保锁屏窗口始终置顶且聚焦，防止被最小化
            let is_locked = get_timer_state().lock().unwrap().lock_screen_active;
            if is_locked {
                // 主窗口
                if let Some(window) = app_handle.get_webview_window("main") {
                    if !window.is_visible().unwrap_or(false) {
                        let _ = window.show();
                    }
                    if !window.is_focused().unwrap_or(false) {
                        let _ = window.set_focus();
                    }
                    let _ = window.set_always_on_top(true);
                }

                // 从属窗口
                let lock_state = app_handle.state::<LockState>();
                // 获取锁并克隆列表，尽快释放锁
                let slave_windows: Vec<String> = {
                    let guard = lock_state.0.lock().unwrap();
                    guard.clone()
                };
                
                for label in slave_windows {
                    if let Some(window) = app_handle.get_webview_window(&label) {
                        if !window.is_visible().unwrap_or(false) {
                            let _ = window.show();
                        }
                        if !window.is_focused().unwrap_or(false) {
                            let _ = window.set_focus();
                        }
                        let _ = window.set_always_on_top(true);
                    }
                }
            }
        }
    });
}

fn get_settings_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("desk-reminder").join("settings.json")
}

#[tauri::command]
fn load_settings() -> String {
    let path = get_settings_path();
    fs::read_to_string(path).unwrap_or_default()
}

#[tauri::command]
fn save_settings(settings: String) -> Result<(), String> {
    let path = get_settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, settings).map_err(|e| e.to_string())
}

#[tauri::command]
fn play_notification_sound() {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        use std::os::windows::process::CommandExt;
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-WindowStyle", "Hidden",
                "-ExecutionPolicy", "Bypass",
                "-Command",
                "Add-Type -AssemblyName System.Sound; [System.Media.SystemSounds]::Beep.Play();"
            ])
            .creation_flags(0x08000000)
            .output();
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("afplay")
            .args(["/System/Library/Sounds/Glass.aiff"])
            .output();
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        // 尝试多种 Linux 系统声音命令
        if Command::new("paplay")
            .args(["/usr/share/sounds/alsa/Front_Left.wav"])
            .output().is_ok() {
            return;
        }

        if Command::new("aplay")
            .args(["/usr/share/sounds/alsa/Front_Left.wav"])
            .output().is_ok() {
            return;
        }

        // 最后尝试系统提示音
        let _ = Command::new("echo").args(["\u{0007}"]).output();
    }
}

#[tauri::command]
fn show_notification(app: tauri::AppHandle, title: String, body: String) {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .unwrap();
}

#[tauri::command]
fn show_main_window(window: tauri::Window) {
    let _ = window.show();
    let _ = window.set_focus();
}

#[tauri::command]
fn hide_main_window(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
fn update_tray_tooltip(state: State<TrayState>, tooltip: String) {
    if let Some(tray) = state.0.lock().unwrap().as_ref() {
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}

#[tauri::command]
fn update_pause_menu(state: State<PauseMenuState>, paused: bool) {
    if let Some(menu_item) = state.0.lock().unwrap().as_ref() {
        let text = if paused { "继续" } else { "暂停" };
        let _ = menu_item.set_text(text);
    }
}

#[tauri::command]
async fn enter_lock_mode(app: tauri::AppHandle, window: tauri::Window, state: State<'_, LockState>, task: Option<LockTaskArgs>) -> Result<(), String> {
    let _ = window.set_fullscreen(true);
    let _ = window.set_always_on_top(true);
    let _ = window.set_closable(false);
    let _ = window.set_minimizable(false);
    let _ = window.set_focus();

    let monitors = window.available_monitors().unwrap_or_default();
    let current_monitor = window.current_monitor().unwrap_or(None);
    
    let mut created_windows = Vec::new();
    
    for (i, m) in monitors.iter().enumerate() {
        if let Some(ref cm) = current_monitor {
             // Basic position check to assume it's the same monitor
             if m.position().x == cm.position().x && m.position().y == cm.position().y {
                 continue;
             }
        }

        let label = format!("lock-slave-{}", i);
        
        let mut url_str = String::from("index.html?mode=lock_slave");
        if let Some(ref t) = task {
             let encoded: String = form_urlencoded::Serializer::new(String::new())
                .append_pair("title", &t.title)
                .append_pair("desc", &t.desc)
                .append_pair("duration", &t.duration.to_string())
                .append_pair("icon", &t.icon)
                .finish();
             url_str = format!("index.html?mode=lock_slave&{}", encoded);
        }

        if let Ok(slave) = WebviewWindowBuilder::new(&app, &label, WebviewUrl::App(PathBuf::from(url_str)))
            .title("Lock Screen")
            .always_on_top(true)
            .closable(false)
            .minimizable(false)
            .decorations(false)
            .resizable(false)
            .skip_taskbar(true)
            .visible(false)
            .build() {
                
            let _ = slave.set_position(m.position().clone());
            let _ = slave.set_size(tauri::Size::Physical(m.size().clone()));
            let _ = slave.show();
            let _ = slave.set_focus();
            let _ = slave.set_fullscreen(true);
            created_windows.push(label);
        }
    }
    
    let mut state_guard = state.0.lock().unwrap();
    state_guard.extend(created_windows);

    Ok(())
}

#[tauri::command]
fn exit_lock_mode(app: tauri::AppHandle, window: tauri::Window, state: State<LockState>) {
    let _ = window.set_fullscreen(false);
    let _ = window.set_always_on_top(false);
    let _ = window.set_closable(true);
    let _ = window.set_minimizable(true);

    let mut state_guard = state.0.lock().unwrap();
    for label in state_guard.iter() {
        if let Some(w) = app.get_webview_window(label) {
            let _ = w.close();
        }
    }
    state_guard.clear();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--silent"])
        ))
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            play_notification_sound,
            show_notification,
            show_main_window,
            hide_main_window,
            update_tray_tooltip,
            update_pause_menu,
            enter_lock_mode,
            exit_lock_mode,
            sync_tasks,
            timer_pause,
            timer_resume,
            timer_reset_task,
            timer_reset_all,
            timer_snooze_task,
            get_countdowns,
            timer_set_system_locked,
            timer_set_lock_screen_active,
            set_idle_threshold,
            get_idle_threshold,
        ])
        .manage(TrayState(Mutex::new(None)))
        .manage(LockState(Mutex::new(Vec::new())))
        .manage(PauseMenuState(Mutex::new(None)))
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let reset = MenuItem::with_id(app, "reset", "重置所有任务", true, None::<&str>)?;
            let pause = MenuItem::with_id(app, "pause", "暂停", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &pause, &reset, &quit])?;
            
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("健康提醒助手")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "reset" => {
                            let _ = app.emit("reset-all-tasks", ());
                        }
                        "pause" => {
                            let _ = app.emit("toggle-pause", ());
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
            *app.state::<TrayState>().0.lock().unwrap() = Some(tray);
            *app.state::<PauseMenuState>().0.lock().unwrap() = Some(pause);

            // 启动后端定时器线程
            start_timer_thread(app.handle().clone());

            #[cfg(target_os = "windows")]
            start_session_monitor(app.handle().clone());
            
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // If the window is a lock slave, just close it (don't prevent close)
                // The label check: main window has label "main" (default).
                // Slave windows have "lock-slave-X".
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}