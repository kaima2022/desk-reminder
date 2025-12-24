# 健康办公助手 (Desk Reminder)

> **极简、精准、高效。为现代办公族量身定制的健康守护小程序。**

[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Version](https://img.shields.io/badge/Version-1.4.8-orange?style=flat-square)](https://github.com/)

在快节奏的数字时代，健康的身体是高效生产力的基石。**健康办公助手** 是一款基于 Rust 与 Tauri 开发的高性能桌面应用，旨在通过智能化的任务排程与多维提醒，帮助你在专注工作的同时，科学地进行久坐、补水与用眼休息。

---

## 🖼️ 视觉呈现

### 核心看板 (Dashboard) & 自定义任务排程 (Tasks)

<p align="center">
  <img src="./docs/screenshots/展示1.png" alt="Dashboard" width="30%">
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <img src="./docs/screenshots/展示2.png" alt="Tasks" width="30%">
</p>

### 提醒与弹窗 (Notifications)

<p align="center">
  <img src="./docs/screenshots/notification.png" alt="In-app notification" width="30%">
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <img src="./docs/screenshots/弹窗提醒.png" alt="System notification" width="48%">
</p>

### 锁屏功能（选择开启）

<p align="center">
  <img src="./docs/screenshots/锁屏设置.png" alt="Lock screen settings" width="30%">
</p>

<p align="center">
  <img src="./docs/screenshots/锁屏.png" alt="Lock screen" width="70%">
</p>



---

## 💎 核心特性

### 1. 科学排程，精准反馈
- **多任务并行倒计时**：支持久坐、喝水、护眼等多个健康任务同时运行，互不干扰。
- **可视化进度环**：采用 Apple 风格的环形进度设计，剩余时间一目了然。
- **一键重置**：支持单个任务点击进度环快速重启，适配灵活的工作节奏。

### 2. 深度定制，人性交互
- **智能预设**：提供 15m / 30m / 45m / 60m 快捷键，极速配置。
- **动态任务管理**：支持自定义添加新任务、修改标题、删除非核心提醒。
- **防干扰输入**：重构的局部渲染引擎，确保你在手动修改分钟数时，界面刷新不会干扰你的输入焦点。

### 3. 系统集成，静默运行
- **原生系统通知**：采用底层 Rust 通讯，支持在最小化到托盘时发送系统级弹窗。
- **托盘常驻**：支持最小化至任务栏托盘，不占用宝贵的任务栏空间。
- **开机自启**：集成官方 Autostart 插件，实现一键开启随心启动。

---

## 🛠️ 技术架构

本产品追求极致的内存占用与启动速度：

- **Backend**: [Rust](https://www.rust-lang.org/) (Tauri 2.0) - 提供安全、高性能的系统底层能力。
- **Frontend**: [Vite](https://vitejs.dev/) + Vanilla TypeScript - 极简的渲染逻辑，确保 UI 响应零延迟。
- **Communication**: Tauri IPC - 高速的前后端异步通信协议。
- **Styles**: CSS Variables (Modern Design System) - 丝滑的 Apple 风格 UI。

---

## 📦 下载与安装

您可以直接前往 [GitHub Releases](https://github.com/kaima2022/desk-reminder/releases) 页面下载适用于您系统的最新版安装包。支持 Windows (.exe), macOS (.dmg), 以及 Linux (.deb, .AppImage)。

### Windows Scoop 安装

```powershell
# 添加 bucket（首次安装）
scoop bucket add desk-reminder https://github.com/kaima2022/desk-reminder

# 安装
scoop install health-reminder

# 更新
scoop update health-reminder
```

---

## 🛠️ 从代码构建

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

## 📈 后续路线图

- [ ] 数据统计视图：查看周/月健康达成率。
- [ ] 更多系统音效：支持自定义上传提醒音。
- [ ] 专注模式联动：在电脑全屏工作或游戏时智能静默。

## 📜 版本记录

> **请下载最新安装包重新安装（会自动覆盖）**

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