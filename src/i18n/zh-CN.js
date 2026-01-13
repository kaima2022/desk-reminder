/**
 * 中文语言包 (简体中文)
 */
export default {
  // 应用信息
  app: {
    title: '健康提醒助手',
    subtitle: '关爱健康，从每一次提醒开始',
    footer: '健康办公助手 v1.5.5 · 愿你每天都有好身体',
    trayTooltip: '健康提醒助手',
  },

  // 默认任务
  tasks: {
    sit: {
      title: '久坐提醒',
      desc: '该起来活动了，走动一下吧~',
    },
    water: {
      title: '喝水提醒',
      desc: '该喝口水了，保持水分充足~',
    },
    eye: {
      title: '护眼提醒',
      desc: '让眼睛休息一下，看看远处~',
    },
    newTask: {
      title: '新提醒',
      desc: '又是充满活力的一天，记得休息哦~',
    },
  },

  // 统计
  stats: {
    sitBreaks: '休息次数',
    waterCups: '喝水次数',
    workMinutes: '工作分钟',
  },

  // 时间单位
  time: {
    minutes: '分钟',
    seconds: '秒',
    times: '次',
  },

  // 按钮
  buttons: {
    pause: '暂停',
    resume: '继续',
    resetAll: '全部重置',
    gotIt: '我知道了',
    snooze: '推迟 {minutes} 分钟',
    addTask: '添加自定义提醒',
    checkUpdate: '检查更新',
    updateNow: '立即更新',
    checking: '检查中...',
    test: '测试',
    confirmRest: '已完成休息',
  },

  // 锁屏
  lockScreen: {
    emergencyUnlock: '长按 3 秒紧急解锁',
    restTime: '休息时间',
    restMessage: '让身体和眼睛休息一下吧~',
    timeUp: '休息时间到！',
    confirmMessage: '您完成休息了吗？点击下方按钮确认~',
    snoozeLimit: '已达推迟上限',
    strictDisabled: '严格模式已禁用推迟',
    snoozeDuring: '推迟中 {time}',
  },

  // 设置
  settings: {
    title: '系统设置',
    lockScreen: '强制休息锁屏',
    lockScreenDesc: '提醒时锁定屏幕，确保真正休息',
    strictMode: '严格模式',
    strictModeDesc: '开启后锁屏界面将隐藏"紧急解锁"按钮，请谨慎开启',
    advanced: '高级设置',
    autoUnlock: '倒计时结束自动解锁',
    autoUnlockDesc: '休息结束后自动退出锁屏，无需手动确认',
    resetOnIdle: '空闲时重置任务',
    resetOnIdleDesc: '当用户离开电脑（空闲）时自动重置计时',
    allowStrictSnooze: '严格模式允许推迟',
    allowStrictSnoozeDesc: '开启后，即使在严格模式下也允许使用推迟功能',
    idleThreshold: '空闲检测阈值',
    idleThresholdDesc: '超过此时间无操作视为空闲',
    idleThresholdDescIdle: '超过此时间无操作视为空闲 (当前空闲中)',
    maxSnoozeCount: '最大推迟次数',
    maxSnoozeCountDesc: '任务触发后允许连续推迟的次数',
    sound: '提示音',
    autoStart: '开机自启动',
    version: '版本更新',
    currentVersion: '当前版本 v1.5.5',
    newVersion: '当前版本 v1.5.5（有新版本 v{version}）',
    language: '语言',
  },

  // 任务卡片
  taskCard: {
    preNotify: '预告',
    allowSnooze: '允许推迟',
    lockDuration: '锁屏时长',
    clickToReset: '点击重置',
    settings: '设置',
    resetTask: '重置此任务',
  },

  // 状态
  status: {
    paused: '已暂停',
    idle: '空闲中',
    loading: '正在加载...',
    noActiveTask: '无活动任务',
    snoozed: '推迟中',
  },

  // 通知
  notification: {
    preNotifyTitle: '即将提醒：{title}',
    preNotifyBody: '还有 {seconds} 秒将触发提醒，请做好准备。',
  },

  // 更新
  update: {
    newVersion: '发现新版本 v{version}',
    updating: '正在更新...',
    upToDate: '已是最新版本！',
    checkFailed: '检查更新失败：{error}',
    networkError: '网络错误，请稍后重试',
  },

  // 托盘菜单
  tray: {
    quit: '退出',
    show: '显示主窗口',
    reset: '重置所有任务',
    pause: '暂停',
    resume: '继续',
  },
};
