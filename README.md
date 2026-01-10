# 健康办公助手 (Health Reminder)

**中文** | [English](./README.en.md)

> **极简、精准、高效。为现代办公族量身定制的健康守护小程序。**

[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Version](https://img.shields.io/badge/Version-1.5.4-orange?style=flat-square)](https://github.com/)

在快节奏的数字时代，健康的身体是高效生产力的基石。**健康办公助手** 是一款基于 Rust 与 Tauri 开发的高性能桌面应用，旨在通过智能化的任务排程与多维提醒，帮助你在专注工作的同时，科学地进行久坐、补水与用眼休息。

---

##  展示

### 主界面 (Dashboard) & 自定义任务排程 (Tasks)

<p align="center">
  <img src="./docs/screenshots/展示1.png" alt="Dashboard" width="30%">

  <img src="./docs/screenshots/展示2.png" alt="Tasks" width="30%">

  <img src="./docs/screenshots/展示3.png" alt="Tasks" width="30%">
</p>

### 预告与弹窗 (Notifications)

<p align="center">
  <img src="./docs/screenshots/预告.png" alt="In-app notification" width="40%">

  <img src="./docs/screenshots/弹窗提醒.png" alt="System notification" width="40%">
</p>

### 托盘显示

<p align="center">
  <img width="176" height="184" alt="托盘图标" src="https://github.com/user-attachments/assets/f7b40cf9-bf2e-4459-84f8-72ee05a4a549" />
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <img alt="托盘菜单" src="docs\screenshots\托盘菜单.png" />
</p>



### 多显示屏锁屏功能（支持严格模式）

![锁屏](./docs/screenshots/锁屏.png)



### 设置界面（简洁&高级设置）



<p align="center">
  <img src="./docs/screenshots/设置1.png" alt="Dashboard" width="30%">
	&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <img src="./docs/screenshots/设置2.png" alt="Tasks" width="30%">

</p>



---

## 为什么你需要它？

### 1. 真的能让你休息
- **强制锁屏**：到了休息时间，屏幕会自动锁定（支持多显示器），让你不得不放下鼠标，站起来活动一下。
- **严格模式**：如果你总是忍不住点“跳过”，试试开启严格模式，它会藏起解锁按钮，直到休息结束。
- **自动解锁**：休息时间一到，屏幕自动解开，不需要你多点一下鼠标，无缝回归工作。

### 2. 贴心不打扰
- **提前预告**：锁屏前（默认5秒）会弹窗提醒你，让你有时间保存手头的工作，不会被突然打断。
- **暂时推迟**：正在开会或处理急事？可以点“推迟”再工作几分钟。当然，你可以设置推迟次数上限，防止自己一直拖延。
- **智能空闲检测**：如果你离开电脑去倒水了，它会自动帮你重置久坐计时，不会等你回来刚坐下又提醒你休息。

### 3. 简单好用
- **多任务并行**：久坐、喝水、护眼，想提醒什么就开什么，互不冲突。
- **一键设置**：所有设置就在卡片上，点一下小齿轮就能改时间，不用去复杂的菜单里找。
- **轻量安静**：平时它就安静地待在托盘里，占用资源极低，几乎感觉不到它的存在。

---

## 技术架构

本产品追求极致的内存占用与启动速度：

- **Backend**: [Rust](https://www.rust-lang.org/) (Tauri 2.0) - 提供安全、高性能的系统底层能力。
- **Frontend**: [Vite](https://vitejs.dev/) + Vanilla TypeScript - 极简的渲染逻辑，确保 UI 响应零延迟。
- **Communication**: Tauri IPC - 高速的前后端异步通信协议。
- **Styles**: CSS Variables (Modern Design System) - 丝滑的 Apple 风格 UI。

---

## 下载与安装

### 方式1.安装包

您可以直接前往 [GitHub Releases](https://github.com/kaima2022/desk-reminder/releases) 页面下载适用于您系统的最新版安装包。支持 Windows (.exe), macOS (.dmg), 以及 Linux (.deb, .AppImage)。

### 方式2. Windows Scoop 安装或更新

```powershell
# 添加 bucket（首次安装）
scoop bucket add health-reminder https://github.com/kaima2022/desk-reminder

# 安装
scoop install health-reminder

# 更新
scoop update health-reminder
```

<img width="1025" height="245" alt="image" src="https://github.com/user-attachments/assets/11282b88-4665-4374-9de8-54bdcec27e7f" />


---

## 从代码构建

如果您希望从源码编译并运行本项目，请确保您的系统中已安装 Rust 工具链与 Node.js 环境。

### 1. 开发环境运行
```bash
# 安装依赖
npm install

# 启动开发服务器与应用
npm run tauri dev
```

### 2. 生成安装包
```bash
# 构建适用于当前系统的正式版本
npm run tauri build
```

---

## 后续路线图

- [ ] 数据统计视图：查看周/月健康达成率。
- [ ] 更多系统音效：支持自定义上传提醒音。
- [ ] 专注模式联动：在电脑全屏工作或游戏时智能静默。

## 版本记录

> ### 多种更新方式：自动检查更新 || 手动检查更新 || 安装包更新 || Scoop 更新 
> <img width="505" height="192" alt="image" src="https://github.com/user-attachments/assets/31e4d792-6800-4540-bead-58024aa3ba08" />
> <img width="377" height="184" alt="image" src="https://github.com/user-attachments/assets/e54b30f4-97c8-4561-9c69-9373a15137ea" />

> #### scoop 更新命令
> scoop update health-reminder

### v1.5.4 (2026-01-10)
- **国际化支持 (i18n)**：新增中英文语言切换功能，界面右上角可快速切换语言
- **多语言托盘菜单**：托盘右键菜单支持中英文实时切换
- **多屏锁屏语言同步**：修复副屏锁屏显示语言与主屏不一致的问题
- **英文 README**：新增 [README.en.md](./README.en.md) 英文文档

### v1.5.3 (2026-01-09)
- **锁屏保活与修复**：修复了 Windows 10 下强制锁屏窗口可能被最小化或丢失焦点的问题（Watchdog 机制）。
- **推迟功能 (Snooze)**：新增“推迟”功能，支持自定义每次推迟的时长（默认 5 分钟），并正确显示推迟倒计时。
- **新增自动退出锁屏**：新增“倒计时结束自动解锁”选项，无需手动点击确认即可退出锁屏。
- **严格模式**：**新增严格模式，严格模式下，必须好好休息，禁止退出，要好好爱护身体奥！**
- **严格模式增强**：新增“严格模式允许推迟”选项；严格模式下默认隐藏推迟按钮，且支持设置最大推迟次数。
- **提醒预告**：新增任务触发前的预告通知（默认提前 5 秒），支持每任务独立配置。
- **UI/UX 升级**：
    - **高级设置折叠**：将不常用的系统设置收纳至“高级设置”折叠面板，界面更清爽。
    - **卡片设置优化**：重构任务卡片底部设置栏，采用 Grid 布局且默认收起，支持一键展开配置。
    - **右键菜单增强**：托盘右键菜单新增“重置单个任务”子菜单。
- **性能优化**：降低了长期运行的内存占用。

### v1.5.2 (2026-01-04)
- **跨平台空闲检测**：新增系统空闲检测功能，支持 Windows、macOS、Linux 三平台。
- **空闲自动重置**：用户无操作超过设定阈值时，自动重置已勾选的任务倒计时。
- **空闲阈值可配置**：支持在设置中自定义空闲检测阈值（1-60 分钟）。
- **任务级别控制**：每个任务可独立设置是否启用空闲时重置功能。
- **UI 优化**：统一任务卡片底部区域样式，提升视觉一致性。

### v1.5.1 (2025-12-27)
- **后端定时器重构**：将计时逻辑从前端 JavaScript 移至 Rust 后端，解决 macOS App Nap 和窗口最小化时计时器被节流导致倒计时变慢的问题。
- **跨平台计时精准**：采用系统级线程定时器，不受 WebView 节流影响，Windows/macOS/Linux 表现一致。
- **准时提醒保障**：即使应用在后台或最小化状态，也能准时触发提醒通知。

### v1.5.0 (2025-12-26)
- **自动更新系统**：启动时自动检测新版本，支持设置中手动检查，显示版本状态，一键完成更新安装。
- **安全签名验证**：采用非对称加密签名机制，GitHub Actions 自动构建、签名并生成更新文件。
- **优化用户体验**：Toast 消息提示替代弹窗，3秒自动消失，操作流畅不打断。



### v1.4.9 (2025-12-25)
- **锁屏停止计时**：锁屏期间暂停所有任务计时，避免休息时错过提醒。
- **托盘暂停功能**：右键菜单新增"暂停/继续"选项，快速控制计时状态。

### v1.4.8 (2025-12-24)
- **多屏锁屏适配**：锁屏时覆盖所有显示器，防止在副屏继续工作。
- **手动确认完成**：锁屏倒计时结束后需手动点击确认是否完成休息。
- **自动最小化**：锁屏结束后软件自动最小化到托盘。
- **Scoop 安装支持**：Windows 用户可通过 Scoop 包管理器安装和更新。

### v1.4.7 (2025-12-23)
- **强制休息锁屏**：新增锁屏功能，提醒触发时全屏锁定，确保真正休息。
- **锁屏时长可配置**：支持 10s / 20s / 30s 三档锁屏时长选择。
- **紧急解锁**：长按 3 秒可紧急解锁，防止耽误紧急任务。
- **自动弹出**：锁屏时自动弹出窗口，即使最小化到托盘也能正常触发。

### v1.4.6 (2025-12-22)
- **双图标修复**：修复显示两个图标的问题。
- **托盘悬浮提示**：鼠标悬浮显示所有任务剩余时间。
- **右键菜单增强**：右键菜单增添"重置所有任务时间"功能。
- **版本显示修复**：修复软件版本显示问题。

### v1.4.4 (2025-12-21)
- **打包优化**：修复了 Linux Debian 包名规范问题，并内置了完整的运行时依赖声明。
- **逻辑修复**：实现了非阻塞计时，提醒触发后立即开始下一轮计时，不再依赖用户点击。
- **功能实做**：彻底完成“系统设置”模块，包括真实的开机自启控制和音效测试功能。
- **CI/CD 增强**：优化了 GitHub Actions 脚本，支持 macOS Apple Silicon 原生构建。

### v1.3.0 (2025-12-21)
- **UI 重写**：全面采用 Apple 设计语言，重构了看板和任务卡片。
- **通知系统**：修复了 Tauri 2.0 系统通知失效问题，集成原生 Rust 通知插件。
- **输入优化**：重构渲染引擎实现局部刷新，解决了修改分钟数时的焦点丢失问题。

### v1.0.0 (2025-12-20)
- 初始版本发布，包含基础的久坐、喝水、护眼倒计时功能。

---

## 📄 许可证

本项目遵循 **MIT License**。您可以自由地使用、修改和分发。

---
© 2025 健康办公助手. 愿你每天都有好身体。
