# Tauri + Vue + TypeScript

## 如何开发

1. 安装依赖：`npm install`
2. 运行调试: `npm run tauri dev`

## 如何打包

以下是基于MacOS打开，制作不同平台安装包的方法

国内安装加速：

```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
sh rustup-init.sh
```

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
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
# Install cargo-xwin
cargo install --locked cargo-xwin
```

2. Windows 64位系统打包

```bash
rustup target add x86_64-pc-windows-msvc
# 为64位系统打包
npm run tauri build -- --runner cargo-xwin --target x86_64-pc-windows-msvc
```

3. Windows 32位系统打包

```bash
# 32位系统打包需要的依赖
rustup target add i686-pc-windows-msvc
# 32位系统打包命令
# ARM系统打包命令
npm run tauri build -- --runner cargo-xwin --target i686-pc-windows-msvc
```

3. Windows ARM系统打包

```bash
# ARM系统打包需要的依赖
rustup target add aarch64-pc-windows-msvc
# ARM系统打包命令
npm run tauri build -- --runner cargo-xwin --target aarch64-pc-windows-msvc
```
