# Health Reminder Modification Plan (Issue #4)

## Overview
This plan addresses the issues and suggestions from GitHub Issue #4, focusing on fixing the lock screen behavior on Windows 10, improving user experience with auto-unlocking, and adding features like pre-notifications and strict mode.

## Phase 1: Core Fixes & UX Improvements

### 1. Fix Lock Screen Minimization (Windows 10 Bug)
**Goal:** Ensure the lock screen remains full-screen and top-most, preventing accidental minimization.
- **File:** `src-tauri/src/lib.rs`
- **Changes:**
    - Implement a "Watchdog" mechanism within the existing timer thread.
    - When `lock_screen_active` is true, periodically (e.g., every second) force the lock screen windows (main and slaves) to focus and stay on top.
    - Re-apply `set_always_on_top(true)` and `set_focus()` if the window loses focus.

### 2. Auto-Unlock After Timer Ends
**Goal:** Remove the need for a manual confirmation click after the break timer finishes.
- **File:** `src/main.js`
- **Changes:**
    - Add `autoUnlock: true` to the default `settings` object.
    - Add a toggle switch in the "System Settings" section of the UI for "Auto Unlock".
    - Update `startLockScreen` logic: When the countdown reaches zero (`remaining <= 0`), check `settings.autoUnlock`.
        - If `true`: Call `endLockScreen()` immediately.
        - If `false`: Call `showLockConfirm()` (existing behavior).

## Phase 2: Feature Enhancements

### 3. Pre-Lock Notification
**Goal:** Warn the user before the screen locks to allow them to prepare or finish immediate tasks.
- **File:** `src/main.js`
- **Changes:**
    - Add `preNotificationSeconds: 30` to `settings`.
    - Add a configuration input in settings for "Pre-notification Time".
    - In the timer logic (frontend or backend event listener), check if `remaining === preNotificationSeconds`.
    - If matched, trigger a system notification: "Screen will lock in 30 seconds."

### 4. Strict Mode
**Goal:** Prevent users from skipping breaks easily.
- **File:** `src/main.js`, `src/styles.css` (if needed)
- **Changes:**
    - Add `strictMode: false` to `settings`.
    - Add a "Strict Mode" toggle in settings with a warning description.
    - When rendering the lock screen (`renderFullUI` -> `lock-screen` section):
        - If `strictMode` is enabled, hide the `.unlock-btn` (Emergency Unlock).
    - *Safety Net:* Consider a keyboard combination (e.g., specific key sequence) as a failsafe if users get stuck, but keep it hidden from the UI.

### 5. Reminder Snooze (Postponement)
**Goal:** Allow users to delay a break if they are busy.
- **File:** `src/main.js`, `src-tauri/src/lib.rs` (new command)
- **Changes:**
    - **Frontend:** Add a "Snooze 5 min" button to the pre-notification pop-up or main interface when a task is imminent.
    - **Backend:** Add a `snooze_task(task_id, minutes)` command to add time to the task's `reset_time`.

## Implementation Order
1.  **Backend Fix:** Implement Lock Screen Watchdog in `src-tauri/src/lib.rs`.
2.  **Frontend UX:** Implement Auto-Unlock logic and settings in `src/main.js`.
3.  **Frontend Feature:** Implement Pre-notification logic.
4.  **Frontend Feature:** Implement Strict Mode.

## Verification
- Test lock screen behavior on Windows 10/11 (ensure no minimization).
- Verify auto-unlock transitions smoothly back to the desktop.
- Verify notifications appear at the correct time before locking.
- Verify strict mode hides the unlock button.
