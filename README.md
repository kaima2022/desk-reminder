# 健康办公助手 (Desk Reminder)

> **极简、精准、高效。为现代办公族量身定制的健康守护小程序。**

[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Version](https://img.shields.io/badge/Version-1.4.0-orange?style=flat-square)](https://github.com/)

在快节奏的数字时代，健康的身体是高效生产力的基石。**健康办公助手** 是一款基于 Rust 与 Tauri 开发的高性能桌面应用，旨在通过智能化的任务排程与多维提醒，帮助你在专注工作的同时，科学地进行久坐、补水与用眼休息。

---

## 视觉呈现

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

---

## 核心特性

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
- **开机自启**：集成 Windows 注册表管理，实现一键开启随心启动。

---

## 技术架构

本产品追求极致的内存占用与启动速度：

- **Backend**: [Rust](https://www.rust-lang.org/) (Tauri 2.0) - 提供安全、高性能的系统底层能力。
- **Frontend**: [Vite](https://vitejs.dev/) + Vanilla TypeScript - 极简的渲染逻辑，确保 UI 响应零延迟。
- **Communication**: Tauri IPC - 高速的前后端异步通信协议。
- **Styles**: CSS Variables (Modern Design System) - 丝滑的 Apple 风格 UI。

---

## 快速开始

### 开发环境配置
确保你已安装 Rust 工具链与 Node.js 环境。

```bash
# 安装依赖
npm install

# 启动开发环境
npm run tauri dev
```

### 构建正式版本
```bash
# 构建所有平台
npm run tauri build

# 构建特定平台
npm run tauri build -- --target nsis    # Windows
npm run tauri build -- --target dmg     # macOS
npm run tauri build -- --target appimage # Linux
```

---

## 后续路线图

- [ ] 数据统计视图：查看周/月健康达成率。
- [ ] 更多系统音效：支持自定义上传提醒音。
- [ ] 专注模式联动：在电脑全屏工作或游戏时智能静默。

## 版本记录

### v1.4.0 (2025-12-21)
- **本地化完善**：完成中文本地化内容，提供更友好的用户体验。
- **自启动集成**：集成开机自启动插件，实现一键开启随心启动功能。
- **Bug 修复**：修复若干bug。

### v1.3.0 (2025-12-21)
- **UI 重写**：全面采用 Apple 设计语言，重构了看板和任务卡片。
- **通知系统**：修复了 Tauri 2.0 系统通知失效问题，集成原生 Rust 通知插件。
- **输入优化**：重构渲染引擎实现局部刷新，解决了修改分钟数时的焦点丢失问题。
- **多任务支持**：支持自定义任务的新增、修改与删除，各任务拥有独立进度环。
- **音效升级**：将简单的蜂鸣音替换为温和的系统级 Notify 音效。

### v1.0.0 (2025-12-20)
- 初始版本发布，包含基础的久坐、喝水、护眼倒计时功能。

---

## 许可证

本项目遵循 **MIT License**。您可以自由地使用、修改和分发。

---
© 2025 健康办公助手. 愿你每天都有好身体。