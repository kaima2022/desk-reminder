use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, TrayIcon},
    Manager, WindowEvent, State, Emitter, WebviewWindowBuilder, WebviewUrl,
};
use tauri_plugin_notification::NotificationExt;
use url::form_urlencoded;

struct TrayState(Mutex<Option<TrayIcon>>);
struct LockState(Mutex<Vec<String>>);

#[derive(serde::Deserialize, serde::Serialize)]
struct LockTaskArgs {
    title: String,
    desc: String,
    duration: i32,
    icon: String,
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
fn update_tray_tooltip(state: State<TrayState>, tooltip: String) {
    if let Some(tray) = state.0.lock().unwrap().as_ref() {
        let _ = tray.set_tooltip(Some(&tooltip));
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
            update_tray_tooltip,
            enter_lock_mode,
            exit_lock_mode,
        ])
        .manage(TrayState(Mutex::new(None)))
        .manage(LockState(Mutex::new(Vec::new())))
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let reset = MenuItem::with_id(app, "reset", "重置所有任务", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &reset, &quit])?;
            
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