# Tauri + Vue + TypeScript

## 如何开发

1. 安装依赖：`npm install`
2. 运行调试: `npm run tauri dev`

## 如何打包

以下是基于MacOS打开，制作不同平台安装包的方法

### 制作MacOS安装包

- 打包命令: `npm run tauri build`

需要注意，MacOS需要开发者签名，不然只能本机使用。

### 制作Windows安装包

1. 安装打包依赖

```bash
# Install NSIS
brew install nsis
# Install LLVM and the LLD Linker
brew install llvm
# Install the Windows Rust target
rustup target add x86_64-pc-windows-msvc
# Install cargo-xwin
cargo install --locked cargo-xwin
```

2. 开始打包

```bash
npm run tauri build -- --runner cargo-xwin --target x86_64-pc-windows-msvc
```