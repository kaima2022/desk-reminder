# Health Reminder

[中文](./README.md) | **English**

> **Minimalist, Precise, Efficient. A health guardian app tailored for modern office workers.**

[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Version](https://img.shields.io/badge/Version-1.5.5-orange?style=flat-square)](https://github.com/)

In today's fast-paced digital era, a healthy body is the cornerstone of high productivity. **Health Reminder** is a high-performance desktop application built with Rust and Tauri, designed to help you take breaks from sitting, stay hydrated, and rest your eyes through intelligent task scheduling and multi-dimensional reminders.

---

## Showcase

### Main Interface (Dashboard) & Custom Task Scheduling (Tasks)

<p align="center">
  <img src="./docs/screenshots/展示1.png" alt="Dashboard" width="30%">

  <img src="./docs/screenshots/展示2.png" alt="Tasks" width="30%">

  <img src="./docs/screenshots/展示3.png" alt="Tasks" width="30%">
</p>

### Pre-notifications & Popups (Notifications)

<p align="center">
  <img src="./docs/screenshots/预告.png" alt="In-app notification" width="40%">

  <img src="./docs/screenshots/弹窗提醒.png" alt="System notification" width="40%">
</p>

### System Tray

<p align="center">
  <img width="176" height="184" alt="Tray icon" src="https://github.com/user-attachments/assets/f7b40cf9-bf2e-4459-84f8-72ee05a4a549" />
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <img alt="Tray menu" src="docs\screenshots\托盘菜单.png" />
</p>



### Multi-Monitor Lock Screen (Supports Strict Mode)

![Lock Screen](./docs/screenshots/锁屏.png)



### Settings Interface (Simple & Advanced Settings)



<p align="center">
  <img src="./docs/screenshots/设置1.png" alt="Dashboard" width="30%">
	&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <img src="./docs/screenshots/设置2.png" alt="Tasks" width="30%">

</p>



---

## Why Do You Need It?

### 1. It Actually Makes You Rest
- **Forced Lock Screen**: When it's time to rest, the screen locks automatically (supports multiple monitors), forcing you to put down your mouse and get moving.
- **Strict Mode**: If you can't resist clicking "skip", try enabling strict mode - it hides the unlock button until rest time is over.
- **Auto Unlock**: When rest time ends, the screen unlocks automatically without any extra clicks, seamlessly returning you to work.

### 2. Thoughtful Without Being Intrusive
- **Advance Notice**: A popup appears before locking (default 5 seconds), giving you time to save your work without being suddenly interrupted.
- **Temporary Snooze**: In a meeting or handling something urgent? Click "Snooze" to work a few more minutes. You can set a maximum snooze count to prevent endless procrastination.
- **Smart Idle Detection**: If you step away from your computer to get water, it automatically resets the sitting timer, so you won't be reminded to rest right after sitting back down.

### 3. Simple and Easy to Use
- **Parallel Multi-tasking**: Sitting, drinking water, eye rest - enable whatever reminders you want, they don't conflict.
- **One-click Settings**: All settings are right on the card, click the gear icon to change times, no need to dig through complex menus.
- **Lightweight and Silent**: It quietly sits in the tray, using minimal resources, barely noticeable.

---

## Technical Architecture

This product pursues minimal memory usage and startup speed:

- **Backend**: [Rust](https://www.rust-lang.org/) (Tauri 2.0) - Provides secure, high-performance system-level capabilities.
- **Frontend**: [Vite](https://vitejs.dev/) + Vanilla TypeScript - Minimalist rendering logic ensuring zero-latency UI response.
- **Communication**: Tauri IPC - High-speed async frontend-backend communication protocol.
- **Styles**: CSS Variables (Modern Design System) - Silky smooth Apple-style UI.

---

## Download & Installation

### Option 1. Installer Package

You can directly go to [GitHub Releases](https://github.com/kaima2022/desk-reminder/releases) to download the latest installer for your system. Supports Windows (.exe), macOS (.dmg), and Linux (.deb, .AppImage).

### Option 2. Windows Scoop Install or Update

```powershell
# Add bucket (first time only)
scoop bucket add health-reminder https://github.com/kaima2022/desk-reminder

# Install
scoop install health-reminder

# Update
scoop update health-reminder
```

<img width="1025" height="245" alt="image" src="https://github.com/user-attachments/assets/11282b88-4665-4374-9de8-54bdcec27e7f" />


---

## Build from Source

If you wish to compile and run this project from source, ensure you have the Rust toolchain and Node.js environment installed.

### 1. Development Mode
```bash
# Install dependencies
npm install

# Start development server and application
npm run tauri dev
```

### 2. Build Installer
```bash
# Build production version for current system
npm run tauri build
```

---

## Roadmap

- [ ] Statistics View: View weekly/monthly health achievement rates.
- [ ] More System Sounds: Support custom upload of reminder sounds.
- [ ] Focus Mode Integration: Smart silent mode when computer is in fullscreen work or gaming.

## Version History

> ### Multiple Update Methods: Auto Check || Manual Check || Installer Update || Scoop Update
> <img width="505" height="192" alt="image" src="https://github.com/user-attachments/assets/31e4d792-6800-4540-bead-58024aa3ba08" />
> <img width="377" height="184" alt="image" src="https://github.com/user-attachments/assets/e54b30f4-97c8-4561-9c69-9373a15137ea" />

> #### Scoop update command
> scoop update health-reminder

### v1.5.5 (2026-01-13)
- **macOS Fix**: Fixed an issue where clicking the Dock icon would not restore the main window.

### v1.5.4 (2026-01-10)
- **Internationalization (i18n)**: Added Chinese/English language switching, quick toggle in the top-right corner
- **Multi-language Tray Menu**: Tray right-click menu supports real-time language switching
- **Multi-screen Lock Screen Language Sync**: Fixed issue where secondary screen lock screen displayed different language than main screen
- **English README**: Added [README.en.md](./README.en.md) English documentation

### v1.5.3 (2026-01-09)
- **Lock Screen Keep-alive & Fix**: Fixed issue where forced lock screen window could be minimized or lose focus on Windows 10 (Watchdog mechanism).
- **Snooze Feature**: Added "Snooze" function with customizable snooze duration (default 5 minutes), correctly displaying snooze countdown.
- **Auto Exit Lock Screen**: Added "Auto unlock when countdown ends" option, no manual confirmation needed to exit lock screen.
- **Strict Mode**: **Added strict mode - in strict mode, you must rest properly, no exit allowed. Take care of your body!**
- **Strict Mode Enhancement**: Added "Allow snooze in strict mode" option; snooze button hidden by default in strict mode, supports setting maximum snooze count.
- **Pre-notification**: Added pre-notification before task triggers (default 5 seconds before), configurable per task.
- **UI/UX Upgrades**:
    - **Advanced Settings Collapse**: Less frequently used system settings are now in a collapsible "Advanced Settings" panel for a cleaner interface.
    - **Card Settings Optimization**: Restructured task card footer settings bar with Grid layout, collapsed by default, expandable with one click.
    - **Right-click Menu Enhancement**: Tray right-click menu now includes "Reset Single Task" submenu.
- **Performance Optimization**: Reduced memory usage during long-term operation.

### v1.5.2 (2026-01-04)
- **Cross-platform Idle Detection**: Added system idle detection supporting Windows, macOS, and Linux.
- **Auto Reset on Idle**: Automatically resets enabled task countdowns when user is inactive beyond threshold.
- **Configurable Idle Threshold**: Supports customizing idle detection threshold (1-60 minutes) in settings.
- **Per-task Control**: Each task can independently enable/disable idle reset functionality.
- **UI Optimization**: Unified task card footer area styling for better visual consistency.

### v1.5.1 (2025-12-27)
- **Backend Timer Refactor**: Moved timing logic from frontend JavaScript to Rust backend, solving timer throttling issues caused by macOS App Nap and window minimization.
- **Cross-platform Precise Timing**: Uses system-level thread timer, unaffected by WebView throttling, consistent behavior across Windows/macOS/Linux.
- **Guaranteed On-time Reminders**: Reminders trigger on time even when app is in background or minimized.

### v1.5.0 (2025-12-26)
- **Auto Update System**: Automatic new version detection on startup, manual check in settings, version status display, one-click update installation.
- **Secure Signature Verification**: Uses asymmetric encryption signature mechanism, GitHub Actions auto-builds, signs and generates update files.
- **Improved User Experience**: Toast messages replace popups, auto-dismiss after 3 seconds, smooth non-interrupting operation.



### v1.4.9 (2025-12-25)
- **Timer Stops on Lock**: Pauses all task timers during lock screen to prevent missing reminders while resting.
- **Tray Pause Function**: Right-click menu adds "Pause/Resume" option for quick timer control.

### v1.4.8 (2025-12-24)
- **Multi-monitor Lock Screen**: Lock screen covers all monitors, preventing work on secondary screens.
- **Manual Confirmation**: Manual click required to confirm rest completion after lock screen countdown ends.
- **Auto Minimize**: Software automatically minimizes to tray after lock screen ends.
- **Scoop Installation Support**: Windows users can install and update via Scoop package manager.

### v1.4.7 (2025-12-23)
- **Forced Rest Lock Screen**: Added lock screen feature, fullscreen lock when reminder triggers, ensuring real rest.
- **Configurable Lock Duration**: Supports 10s / 20s / 30s lock screen duration options.
- **Emergency Unlock**: Long press 3 seconds for emergency unlock, preventing delays for urgent tasks.
- **Auto Popup**: Window automatically pops up during lock screen, even when minimized to tray.

### v1.4.6 (2025-12-22)
- **Dual Icon Fix**: Fixed issue showing two icons.
- **Tray Hover Tooltip**: Mouse hover shows remaining time for all tasks.
- **Right-click Menu Enhancement**: Right-click menu adds "Reset All Task Times" function.
- **Version Display Fix**: Fixed software version display issue.

### v1.4.4 (2025-12-21)
- **Package Optimization**: Fixed Linux Debian package naming convention issues, bundled complete runtime dependency declarations.
- **Logic Fix**: Implemented non-blocking timing, next round starts immediately after reminder triggers without waiting for user click.
- **Feature Implementation**: Fully completed "System Settings" module including real auto-start control and sound test functionality.
- **CI/CD Enhancement**: Optimized GitHub Actions scripts, supports macOS Apple Silicon native builds.

### v1.3.0 (2025-12-21)
- **UI Rewrite**: Fully adopted Apple design language, restructured dashboard and task cards.
- **Notification System**: Fixed Tauri 2.0 system notification failure, integrated native Rust notification plugin.
- **Input Optimization**: Refactored rendering engine for partial refresh, solved focus loss issue when modifying minutes.

### v1.0.0 (2025-12-20)
- Initial release with basic sitting, drinking water, and eye rest countdown features.

---

## License

This project is licensed under the **MIT License**. You are free to use, modify, and distribute.

---
© 2025 Health Reminder. Wishing you good health every day.
