import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { requestPermission } from '@tauri-apps/plugin-notification';

const ICONS = {
  sit: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"></path><path d="M12 6v6l4 2"></path></svg>`,
  water: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.32 0L12 2.69z"></path></svg>`,
  eye: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>`,
  work: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line></svg>`,
  pause: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><rect x="6" y="4" width="4" height="16"></rect><rect x="14" y="4" width="4" height="16"></rect></svg>`,
  play: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg>`,
  reset: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"></polyline><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path></svg>`,
  plus: `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>`,
  trash: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>`,
  bell: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path><path d="M13.73 21a2 2 0 0 1-3.46 0"></path></svg>`,
  volume: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"></path></svg>`
};

const DEFAULT_TASKS = [
  { id: 'sit', title: '久坐提醒', desc: '该起来活动了，走动一下吧~', interval: 45, enabled: true, icon: 'sit' },
  { id: 'water', title: '喝水提醒', desc: '该喝口水了，保持水分充足~', interval: 60, enabled: true, icon: 'water' },
  { id: 'eye', title: '护眼提醒', desc: '让眼睛休息一下，看看远处~', interval: 20, enabled: true, icon: 'eye' }
];

let settings = {
  tasks: [...DEFAULT_TASKS],
  soundEnabled: true,
  autoStart: false,
  lockScreenEnabled: false,
  lockDuration: 10,
};

let countdowns = {};
let stats = {
  sitBreaks: 0,
  waterCups: 0,
  workMinutes: 0,
};
let isPaused = false;
let isSystemLocked = false;
let workStartTime = Date.now();
let activePopup = null;
let lockScreenState = {
  active: false,
  remaining: 0,
  task: null,
  unlockProgress: 0,
  unlockTimer: null,
  waitingConfirm: false,
}; 

async function init() {
  const urlParams = new URLSearchParams(window.location.search);
  if (urlParams.get('mode') === 'lock_slave') {
    const task = {
      title: urlParams.get('title') || '休息时间',
      desc: urlParams.get('desc') || '让眼睛休息一下',
      icon: urlParams.get('icon') || 'eye',
      id: 'slave_lock'
    };
    const duration = parseInt(urlParams.get('duration') || '10');
    
    settings.lockDuration = duration;
    
    lockScreenState = {
      active: true,
      remaining: duration,
      task: task,
      unlockProgress: 0,
      unlockTimer: null,
      waitingConfirm: false,
    };
    
    renderFullUI();
    
    // 隐藏从属屏幕的解锁按钮
    setTimeout(() => {
      const btn = document.querySelector('.unlock-btn');
      if (btn) btn.style.display = 'none';
    }, 0);

    const lockInterval = setInterval(() => {
      lockScreenState.remaining--;
      updateLockScreenTimer();
      if (lockScreenState.remaining <= 0) {
        clearInterval(lockInterval);
      }
    }, 1000);

    return;
  }

  await loadSettings();
  
  try {
    settings.autoStart = await isEnabled();
  } catch (e) {
    console.error('Failed to check autostart status', e);
  }

  try {
    await requestPermission();
  } catch (e) {
    console.error('Failed to request notification permission', e);
  }
  
  settings.tasks.forEach(task => {
    if (countdowns[task.id] === undefined) {
      countdowns[task.id] = task.interval * 60;
    }
  });

  renderFullUI(); 
  setInterval(tick, 1000);
  
  listen('show-window', () => {
    invoke('show_main_window');
  });

  listen('reset-all-tasks', () => {
    resetAll();
  });

  listen('toggle-pause', () => {
    togglePause();
  });

  listen('system-locked', () => {
    isSystemLocked = true;
  });

  listen('system-unlocked', () => {
    isSystemLocked = false;
  });
}

async function loadSettings() {
  try {
    const saved = await invoke('load_settings');
    if (saved) {
      const parsed = JSON.parse(saved);
      settings = { ...settings, ...parsed };
    }
  } catch (e) {
    console.log('Using default settings');
  }
  
  const savedStats = localStorage.getItem('reminder_stats');
  if (savedStats) {
    const parsed = JSON.parse(savedStats);
    if (parsed.date === new Date().toDateString()) {
      stats = parsed.stats;
    }
  }
}

async function saveSettings() {
  await invoke('save_settings', { settings: JSON.stringify(settings) });
}

function saveStats() {
  localStorage.setItem('reminder_stats', JSON.stringify({
    date: new Date().toDateString(),
    stats: stats,
  }));
}

function tick() {
  if (isPaused || isSystemLocked || lockScreenState.active) return;
  stats.workMinutes = Math.floor((Date.now() - workStartTime) / 60000);
  
  settings.tasks.forEach(task => {
    if (task.enabled && countdowns[task.id] > 0) {
      countdowns[task.id]--;
      if (countdowns[task.id] === 0) {
        countdowns[task.id] = task.interval * 60; 
        triggerNotification(task);
      }
    }
  });
  updateLiveValues(); 
}

async function triggerNotification(task) {
  if (settings.soundEnabled) {
    invoke('play_notification_sound').catch(() => {});
  }
  invoke('show_notification', { title: task.title, body: task.desc }).catch(console.error);
  
  if (settings.lockScreenEnabled) {
    await startLockScreen(task);
  } else {
    activePopup = { ...task };
    renderFullUI();
  }
}

async function startLockScreen(task) {
  lockScreenState = {
    active: true,
    remaining: settings.lockDuration,
    task: { ...task },
    unlockProgress: 0,
    unlockTimer: null,
    waitingConfirm: false,
  };
  
  try {
    await invoke('show_main_window');
    await invoke('enter_lock_mode', {
      task: {
        title: task.title,
        desc: task.desc,
        duration: settings.lockDuration,
        icon: task.icon
      }
    });
  } catch (e) {
    console.error('Failed to enter lock mode', e);
  }
  
  renderFullUI();
  
  const lockInterval = setInterval(() => {
    if (!lockScreenState.active) {
      clearInterval(lockInterval);
      return;
    }
    
    lockScreenState.remaining--;
    updateLockScreenTimer();
    
    if (lockScreenState.remaining <= 0) {
      clearInterval(lockInterval);
      showLockConfirm();
    }
  }, 1000);
}

function showLockConfirm() {
  lockScreenState.waitingConfirm = true;
  renderFullUI();
}

async function endLockScreen() {
  lockScreenState.active = false;
  lockScreenState.waitingConfirm = false;
  
  const id = lockScreenState.task?.id;
  if (id === 'sit') stats.sitBreaks++;
  if (id === 'water') stats.waterCups++;
  saveStats();
  
  try {
    await invoke('exit_lock_mode');
    await invoke('hide_main_window');
  } catch (e) {
    console.error('Failed to exit lock mode', e);
  }
  
  renderFullUI();
}

function updateLockScreenTimer() {
  const secondsEl = document.querySelector('.lock-seconds');
  const progressEl = document.querySelector('.lock-timer-ring .progress');
  
  if (secondsEl) {
    secondsEl.textContent = lockScreenState.remaining;
  }
  
  if (progressEl) {
    const total = settings.lockDuration;
    const offset = 565 * (1 - lockScreenState.remaining / total);
    progressEl.style.strokeDashoffset = offset;
  }
}

function startUnlockPress() {
  if (lockScreenState.unlockTimer) return;
  
  lockScreenState.unlockProgress = 0;
  const btn = document.querySelector('.unlock-btn');
  const progressBar = document.querySelector('.unlock-progress');
  
  if (btn) btn.classList.add('pressing');
  
  lockScreenState.unlockTimer = setInterval(() => {
    lockScreenState.unlockProgress += 100 / 30;
    
    if (progressBar) {
      progressBar.style.width = `${lockScreenState.unlockProgress}%`;
    }
    
    if (lockScreenState.unlockProgress >= 100) {
      cancelUnlockPress();
      endLockScreen();
    }
  }, 100);
}

function cancelUnlockPress() {
  if (lockScreenState.unlockTimer) {
    clearInterval(lockScreenState.unlockTimer);
    lockScreenState.unlockTimer = null;
  }
  
  lockScreenState.unlockProgress = 0;
  
  const btn = document.querySelector('.unlock-btn');
  const progressBar = document.querySelector('.unlock-progress');
  
  if (btn) btn.classList.remove('pressing');
  if (progressBar) progressBar.style.width = '0';
}

function dismissNotification() {
  if (!activePopup) return;
  
  // 点击“我知道了”仅记录统计数据，不再负责计时重置（重置已在触发时提前完成）
  const id = activePopup.id;
  if (id === 'sit') stats.sitBreaks++;
  if (id === 'water') stats.waterCups++;
  
  activePopup = null;
  saveStats();
  renderFullUI();
}

function addTask() {
  const id = 'task_' + Date.now();
  settings.tasks.push({
    id: id, title: '新提醒', desc: '又是充满活力的一天，记得休息哦~',
    interval: 30, enabled: true, icon: 'bell'
  });
  countdowns[id] = 30 * 60;
  saveSettings();
  renderFullUI();
}

function removeTask(id) {
  settings.tasks = settings.tasks.filter(t => t.id !== id);
  delete countdowns[id];
  saveSettings();
  renderFullUI();
}

function updateTask(id, updates) {
  const task = settings.tasks.find(t => t.id === id);
  if (task) {
    Object.assign(task, updates);
    if (updates.interval !== undefined) {
      countdowns[id] = task.interval * 60;
    }
    saveSettings();
  }
}

function togglePause() {
  isPaused = !isPaused;
  invoke('update_pause_menu', { paused: isPaused }).catch(() => {});
  renderFullUI();
}

function resetAll() {
  settings.tasks.forEach(task => {
    countdowns[task.id] = task.interval * 60;
  });
  isPaused = false;
  renderFullUI();
}

function formatTime(seconds) {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
}

function updateTrayTooltip() {
  let lines = ['健康提醒助手'];
  if (isPaused) {
    lines.push('(已暂停)');
  } else {
    settings.tasks.forEach(t => {
      if (t.enabled) {
        lines.push(`${t.title}：${formatTime(countdowns[t.id])}`);
      }
    });
  }
  invoke('update_tray_tooltip', { tooltip: lines.join('\n') }).catch(() => {});
}

function updateLiveValues() {
  const statsElements = document.querySelectorAll('.status-item .value');
  if (statsElements[0]) statsElements[0].innerText = stats.sitBreaks;
  if (statsElements[1]) statsElements[1].innerText = stats.waterCups;
  if (statsElements[2]) statsElements[2].innerText = stats.workMinutes;

  let nextTask = null;
  let minTime = Infinity;
  settings.tasks.forEach(t => {
    if (t.enabled && countdowns[t.id] < minTime) {
      minTime = countdowns[t.id];
      nextTask = t;
    }
  });

  const timerText = document.querySelector('.time-text');
  if (timerText) {
    const timeStr = nextTask ? formatTime(countdowns[nextTask.id]) : '--:--';
    timerText.querySelector('.minutes').innerText = timeStr.split(':')[0];
    timerText.querySelector('.seconds').innerText = ':' + timeStr.split(':')[1];
  }

  const timerLabel = document.querySelector('.timer-label');
  if (timerLabel) {
    timerLabel.innerText = (nextTask ? nextTask.title : '无活动任务') + (isPaused ? ' (已暂停)' : '');
  }

  const mainRing = document.querySelector('.timer-ring .progress');
  if (mainRing && nextTask) {
    const total = nextTask.interval * 60;
    const offset = 502 * (1 - countdowns[nextTask.id] / total);
    mainRing.style.strokeDashoffset = offset;
  }

  settings.tasks.forEach(task => {
    const card = document.querySelector(`.reminder-card[data-id="${task.id}"]`);
    if (card) {
      const current = countdowns[task.id] || 0;
      const total = task.interval * 60;
      const offset = 113 * (1 - current / total);
      card.querySelector('.progress-mini .progress').style.strokeDashoffset = offset;
      const timeDisplay = card.querySelector('.time-remaining');
      if (timeDisplay) timeDisplay.innerText = `(${formatTime(current)})`;
    }
  });

  updateTrayTooltip();
}

function renderFullUI() {
  const app = document.getElementById('app');
  app.innerHTML = `
    <div class="header">
      <h1>健康提醒助手</h1>
      <p>关爱健康，从每一次提醒开始</p>
    </div>

    <div class="status-bar">
      <div class="status-item"><div class="icon">${ICONS.sit}</div><div class="value">${stats.sitBreaks}</div><div class="label">休息次数</div></div>
      <div class="status-item"><div class="icon">${ICONS.water}</div><div class="value">${stats.waterCups}</div><div class="label">喝水次数</div></div>
      <div class="status-item"><div class="icon">${ICONS.work}</div><div class="value">${stats.workMinutes}</div><div class="label">工作分钟</div></div>
    </div>

    <div class="timer-display">
      <div class="timer-ring">
        <svg width="180" height="180" viewBox="0 0 180 180"><circle class="bg" cx="90" cy="90" r="80" /><circle class="progress" cx="90" cy="90" r="80" stroke-dasharray="502" stroke-dashoffset="502" /></svg>
        <div class="time-text"><div class="minutes">00</div><div class="seconds">:00</div></div>
      </div>
      <div class="timer-label">正在加载...</div>
    </div>

    <div class="reminder-cards">
      ${settings.tasks.map(task => `
        <div class="reminder-card" data-id="${task.id}">
          <div class="progress-mini" style="cursor:pointer;" title="点击重置" data-reset-id="${task.id}">
            <svg width="40" height="40" viewBox="0 0 40 40"><circle class="bg" cx="20" cy="20" r="18" /><circle class="progress" cx="20" cy="20" r="18" stroke-dasharray="113" stroke-dashoffset="113" /></svg>
            <div style="position:absolute; top:50%; left:50%; transform:translate(-50%,-50%); color:var(--primary); pointer-events:none;">${ICONS[task.icon] || ICONS.bell}</div>
          </div>
          <div class="info">
            <div class="title" contenteditable="true" data-id="${task.id}">${task.title}</div>
            <div class="interval-controls">
              <div class="input-group">
                <input type="number" class="interval-input" value="${task.interval}" data-id="${task.id}" min="1" max="1440">
                <span style="font-size:0.8rem; color:var(--text-muted)">分钟 <span class="time-remaining"></span></span>
              </div>
              <div class="presets">
                <button class="preset-btn" data-id="${task.id}" data-val="15">15m</button><button class="preset-btn" data-id="${task.id}" data-val="30">30m</button>
                <button class="preset-btn" data-id="${task.id}" data-val="45">45m</button><button class="preset-btn" data-id="${task.id}" data-val="60">60m</button>
              </div>
            </div>
          </div>
          <div style="display:flex; flex-direction:column; align-items:flex-end; gap:8px;">
            <div class="toggle ${task.enabled ? 'active' : ''}" data-toggle-id="${task.id}"></div>
            ${!['sit', 'water', 'eye'].includes(task.id) ? `<div class="remove-btn" data-id="${task.id}" style="color:var(--danger); cursor:pointer;">${ICONS.trash}</div>` : ''}
          </div>
        </div>
      `).join('')}
    </div>

    <button class="add-task-btn" id="addTaskBtn">${ICONS.plus} 添加自定义提醒</button>

    <div class="quick-actions">
      <button class="btn btn-primary" id="pauseBtn">${isPaused ? ICONS.play : ICONS.pause} ${isPaused ? '继续' : '暂停'}</button>
      <button class="btn btn-secondary" id="resetBtn">${ICONS.reset} 全部重置</button>
    </div>

    <div class="settings-section">
      <h3>系统设置</h3>
      <div class="setting-row">
        <div class="setting-info">
          <label>强制休息锁屏</label>
          <span class="setting-desc">提醒时锁定屏幕，确保真正休息</span>
        </div>
        <div class="toggle ${settings.lockScreenEnabled ? 'active' : ''}" id="lockToggle"></div>
      </div>
      <div class="setting-row" style="${settings.lockScreenEnabled ? '' : 'opacity:0.5; pointer-events:none;'}">
        <label>锁屏时长</label>
        <div class="duration-select">
          <button class="duration-btn ${settings.lockDuration === 10 ? 'active' : ''}" data-duration="10">10秒</button>
          <button class="duration-btn ${settings.lockDuration === 20 ? 'active' : ''}" data-duration="20">20秒</button>
          <button class="duration-btn ${settings.lockDuration === 30 ? 'active' : ''}" data-duration="30">30秒</button>
        </div>
      </div>
      <div class="setting-row">
        <label>提示音</label>
        <div style="display:flex; gap:12px; align-items:center;">
          <button class="preset-btn" id="testSoundBtn" style="padding:4px 8px; display:flex; gap:4px; align-items:center;">${ICONS.volume} 测试</button>
          <div class="toggle ${settings.soundEnabled ? 'active' : ''}" id="soundToggle"></div>
        </div>
      </div>
      <div class="setting-row">
        <label>开机自启动</label>
        <div class="toggle ${settings.autoStart ? 'active' : ''}" id="startToggle"></div>
      </div>
    </div>

    <div class="notification-popup ${activePopup ? 'show' : ''}">
      <div class="notification-content">
        <div class="emoji">${activePopup ? (ICONS[activePopup.icon] || ICONS.bell) : ''}</div>
        <h2>${activePopup ? activePopup.title : ''}</h2>
        <p>${activePopup ? activePopup.desc : ''}</p>
        <button class="btn btn-primary" id="dismissBtn">我知道了</button>
      </div>
    </div>

    <div class="lock-screen ${lockScreenState.active ? 'show' : ''}">
      <div class="lock-screen-particles">
        ${Array.from({length: 20}, (_, i) => `<div class="particle" style="left:${Math.random()*100}%; top:${Math.random()*100}%; animation-delay:${Math.random()*6}s;"></div>`).join('')}
      </div>
      <div class="lock-screen-content">
        <div class="lock-timer-ring">
          <svg width="200" height="200" viewBox="0 0 200 200">
            <defs>
              <linearGradient id="lockGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#007aff"/>
                <stop offset="100%" style="stop-color:#34c759"/>
              </linearGradient>
            </defs>
            <circle class="bg" cx="100" cy="100" r="90" />
            <circle class="progress" cx="100" cy="100" r="90" stroke-dasharray="565" stroke-dashoffset="0" />
          </svg>
          <div class="center-content">
            <div class="lock-icon">${lockScreenState.task ? (ICONS[lockScreenState.task.icon] || ICONS.bell) : ICONS.eye}</div>
            <div class="lock-seconds">${lockScreenState.waitingConfirm ? '✓' : lockScreenState.remaining}</div>
            <div class="lock-unit">${lockScreenState.waitingConfirm ? '完成' : '秒'}</div>
          </div>
        </div>
        <div class="lock-title">${lockScreenState.waitingConfirm ? '休息时间到！' : (lockScreenState.task?.title || '休息时间')}</div>
        <div class="lock-message">${lockScreenState.waitingConfirm ? '您完成休息了吗？点击下方按钮确认~' : (lockScreenState.task?.desc || '让身体和眼睛休息一下吧~')}</div>
        ${lockScreenState.waitingConfirm ? `
        <button class="confirm-btn" id="confirmBtn">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
          已完成休息
        </button>
        ` : `
        <button class="unlock-btn" id="unlockBtn">
          <div class="unlock-progress"></div>
          <div class="unlock-text">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 9.9-1"/></svg>
            长按 3 秒紧急解锁
          </div>
        </button>
        `}
      </div>
    </div>

    <div class="footer">健康办公助手 v1.4.9 · 愿你每天都有好身体</div>
  `;

  bindEvents();
  updateLiveValues();
}

function bindEvents() {
  document.querySelectorAll('.toggle').forEach(el => {
    el.addEventListener('click', async (e) => {
      if (el.dataset.toggleId) {
        const task = settings.tasks.find(t => t.id === el.dataset.toggleId);
        if (task) {
          task.enabled = !task.enabled;
          el.classList.toggle('active', task.enabled);
          saveSettings();
          updateLiveValues();
        }
      } else if (el.id === 'soundToggle') {
        settings.soundEnabled = !settings.soundEnabled;
        el.classList.toggle('active', settings.soundEnabled);
        saveSettings();
      } else if (el.id === 'startToggle') {
        try {
          const newState = !settings.autoStart;
          if (newState) {
            await enable();
          } else {
            await disable();
          }
          settings.autoStart = newState;
          el.classList.toggle('active', settings.autoStart);
          saveSettings();
        } catch (err) {
          console.error('Failed to toggle autostart', err);
        }
      } else if (el.id === 'lockToggle') {
        settings.lockScreenEnabled = !settings.lockScreenEnabled;
        el.classList.toggle('active', settings.lockScreenEnabled);
        saveSettings();
        renderFullUI();
      }
    });
  });

  document.querySelectorAll('.duration-btn').forEach(el => {
    el.addEventListener('click', () => {
      settings.lockDuration = parseInt(el.dataset.duration);
      saveSettings();
      renderFullUI();
    });
  });

  document.querySelectorAll('.interval-input').forEach(el => {
    el.addEventListener('input', (e) => {
      const val = parseInt(e.target.value);
      if (val > 0) {
        updateTask(el.dataset.id, { interval: val });
        updateLiveValues();
      }
    });
  });

  document.querySelectorAll('.preset-btn:not(#testSoundBtn)').forEach(el => {
    el.addEventListener('click', () => {
      const val = parseInt(el.dataset.val);
      updateTask(el.dataset.id, { interval: val });
      const input = document.querySelector(`.interval-input[data-id="${el.dataset.id}"]`);
      if (input) input.value = val;
      updateLiveValues();
    });
  });

  document.querySelectorAll('.title[contenteditable="true"]').forEach(el => {
    el.addEventListener('blur', (e) => {
      updateTask(el.dataset.id, { title: e.target.innerText });
      updateLiveValues();
    });
  });

  document.querySelectorAll('.progress-mini[data-reset-id]').forEach(el => {
    el.addEventListener('click', () => {
      const id = el.dataset.resetId;
      const task = settings.tasks.find(t => t.id === id);
      if (task) {
        countdowns[id] = task.interval * 60;
        updateLiveValues();
      }
    });
  });

  document.querySelectorAll('.remove-btn').forEach(el => {
    el.addEventListener('click', () => removeTask(el.dataset.id));
  });

  document.getElementById('addTaskBtn').onclick = addTask;
  document.getElementById('pauseBtn').onclick = togglePause;
  document.getElementById('resetBtn').onclick = resetAll;
  document.getElementById('dismissBtn').onclick = dismissNotification;
  
  document.getElementById('testSoundBtn').onclick = () => {
    invoke('play_notification_sound').catch(e => console.error('Sound invoke failed:', e));
  };

  const unlockBtn = document.getElementById('unlockBtn');
  if (unlockBtn) {
    unlockBtn.addEventListener('mousedown', startUnlockPress);
    unlockBtn.addEventListener('mouseup', cancelUnlockPress);
    unlockBtn.addEventListener('mouseleave', cancelUnlockPress);
    unlockBtn.addEventListener('touchstart', (e) => {
      e.preventDefault();
      startUnlockPress();
    });
    unlockBtn.addEventListener('touchend', cancelUnlockPress);
    unlockBtn.addEventListener('touchcancel', cancelUnlockPress);
  }

  const confirmBtn = document.getElementById('confirmBtn');
  if (confirmBtn) {
    confirmBtn.addEventListener('click', endLockScreen);
  }
}

window.triggerNotification = triggerNotification;
window.settings = settings;

init();
