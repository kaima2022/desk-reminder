/**
 * English Language Pack (US English)
 */
export default {
  // App info
  app: {
    title: 'Health Reminder',
    subtitle: 'Care for your health, one reminder at a time',
    footer: 'Health Reminder v1.5.4 · Wishing you good health every day',
    trayTooltip: 'Health Reminder',
  },

  // Default tasks
  tasks: {
    sit: {
      title: 'Stand Up Reminder',
      desc: 'Time to get up and stretch your legs~',
    },
    water: {
      title: 'Drink Water Reminder',
      desc: 'Time for a drink, stay hydrated~',
    },
    eye: {
      title: 'Eye Rest Reminder',
      desc: 'Give your eyes a break, look at something far away~',
    },
    newTask: {
      title: 'New Reminder',
      desc: 'Another energetic day, remember to take breaks~',
    },
  },

  // Statistics
  stats: {
    sitBreaks: 'Breaks Taken',
    waterCups: 'Drinks Taken',
    workMinutes: 'Work Minutes',
  },

  // Time units
  time: {
    minutes: 'min',
    seconds: 'sec',
    times: 'times',
  },

  // Buttons
  buttons: {
    pause: 'Pause',
    resume: 'Resume',
    resetAll: 'Reset All',
    gotIt: 'Got It',
    snooze: 'Snooze {minutes} min',
    addTask: 'Add Custom Reminder',
    checkUpdate: 'Check Update',
    updateNow: 'Update Now',
    checking: 'Checking...',
    test: 'Test',
    confirmRest: 'Rest Completed',
  },

  // Lock screen
  lockScreen: {
    emergencyUnlock: 'Hold 3 sec to unlock',
    restTime: 'Rest Time',
    restMessage: 'Take a break for your body and eyes~',
    timeUp: 'Rest time is up!',
    confirmMessage: 'Have you finished your rest? Click the button to confirm~',
    snoozeLimit: 'Snooze limit reached',
    strictDisabled: 'Snooze disabled in strict mode',
    snoozeDuring: 'Snoozed {time}',
  },

  // Settings
  settings: {
    title: 'System Settings',
    lockScreen: 'Force Rest Lock Screen',
    lockScreenDesc: 'Lock screen when reminder triggers to ensure real rest',
    strictMode: 'Strict Mode',
    strictModeDesc: 'Hides the emergency unlock button on lock screen, use with caution',
    advanced: 'Advanced Settings',
    autoUnlock: 'Auto Unlock After Countdown',
    autoUnlockDesc: 'Automatically exit lock screen when rest ends, no confirmation needed',
    resetOnIdle: 'Reset Tasks When Idle',
    resetOnIdleDesc: 'Automatically reset timers when user is away from computer',
    allowStrictSnooze: 'Allow Snooze in Strict Mode',
    allowStrictSnoozeDesc: 'When enabled, snooze is allowed even in strict mode',
    idleThreshold: 'Idle Detection Threshold',
    idleThresholdDesc: 'Considered idle after this duration of inactivity',
    idleThresholdDescIdle: 'Considered idle after this duration of inactivity (Currently Idle)',
    maxSnoozeCount: 'Max Snooze Count',
    maxSnoozeCountDesc: 'Maximum consecutive snoozes allowed after task triggers',
    sound: 'Notification Sound',
    autoStart: 'Start on Boot',
    version: 'Version Update',
    currentVersion: 'Current version v1.5.4',
    newVersion: 'Current version v1.5.4 (New version v{version} available)',
    language: 'Language',
  },

  // Task card
  taskCard: {
    preNotify: 'Pre-notify',
    allowSnooze: 'Allow snooze',
    lockDuration: 'Lock duration',
    clickToReset: 'Click to reset',
    settings: 'Settings',
    resetTask: 'Reset this task',
  },

  // Status
  status: {
    paused: 'Paused',
    idle: 'Idle',
    loading: 'Loading...',
    noActiveTask: 'No Active Task',
    snoozed: 'Snoozed',
  },

  // Notifications
  notification: {
    preNotifyTitle: 'Upcoming: {title}',
    preNotifyBody: 'Reminder will trigger in {seconds} seconds, get ready.',
  },

  // Update
  update: {
    newVersion: 'New version v{version} available',
    updating: 'Updating...',
    upToDate: 'Already up to date!',
    checkFailed: 'Update check failed: {error}',
    networkError: 'Network error, please try again later',
  },

  // Tray menu
  tray: {
    quit: 'Quit',
    show: 'Show Main Window',
    reset: 'Reset All Tasks',
    pause: 'Pause',
    resume: 'Resume',
  },
};
