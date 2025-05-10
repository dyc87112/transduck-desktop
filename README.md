# TransDuck

<div style="text-align: center;">>
    <img src="https://github.com/dyc87112/transduck-desktop/blob/main/transduck-desktop/src/assets/transduck-logo.png" width="100" height="100">
</div>

TransDuck是一个基于Tauri和Vue实现的免费跨平台桌面应用，提供各种音频/视频的处理能力。

- 官方网站：https://transduck.com/
- 开源地址：https://github.com/dyc87112/transduck-desktop

> 目前SaaS版本的功能还未全部整合入客户端版本，后续会逐步加入到客户端，尽请期待 ^_^

## 技术栈

- **前端**: Vue 3
- **后端**: Node.js 22.15.0
- **框架**: Tauri 2.0
- **核心依赖**: ffmpeg 

## 核心功能

- 音频格式转换
- 视频格式转换
- 国际化支持(目前支持：中文、英文)

TODO
- 更多功能敬请期待...

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

## TODO

1. 跨平台的客户端打包，提供直接使用的客户端
2. 内置ffmpeg实现
3. 更多功能