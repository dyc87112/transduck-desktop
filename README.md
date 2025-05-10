# TransDuck - 跨平台音视频处理桌面应用

TransDuck是一个基于Tauri和Vue的跨平台桌面应用，提供音频/视频格式转换功能。

## 技术栈

- **前端**: Vue 3 + Vue Admin Bootstrap
- **后端**: Node.js 22.15.0
- **框架**: Tauri 2.0
- **核心依赖**: ffmpeg (已内置集成)

## 功能特点

- 支持所有ffmpeg兼容的音频/视频格式转换
- 直观的进度显示
- 多语言支持(中文/英文)
- 现代化UI界面

## 安装与运行

### 开发环境

1. 安装依赖:

```bash
cd transduck-desktop
npm install
```

2. 运行开发服务器:

```bash
npm run tauri dev
```

### 生产构建

```bash
npm run tauri build
```

## 项目结构

```
├── docs            # 项目文档
├── example         # 测试用音视频文件
└── transduck-desktop # 项目源码
    ├── src         # 前端源码
    └── src-tauri   # Tauri后端配置
```

## 贡献指南

欢迎提交Pull Request。对于重大变更，请先创建Issue讨论。

## 许可证

[MIT](LICENSE)