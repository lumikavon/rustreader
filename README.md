# rustreader (Tauri 2.x 桌面应用封装)

本项目使用 **Tauri 2.x** 将 `./ui` 目录下的静态网站打包为跨平台桌面应用。

## 目录结构

- `ui/`：静态网站（已作为前端资源目录）
- `src-tauri/`：Tauri (Rust) 工程与配置
- `target/release/bundle/`：`cargo tauri build` 的安装包输出目录

## 关键配置

- `src-tauri/tauri.conf.json`：
  - 应用版本：`1.0.0`
  - 默认窗口尺寸：`1280x800`
  - 前端资源目录：`build.frontendDist = "../ui"`
  - 图标：`bundle.icon` 指向 `src-tauri/icons/*`
- 平台打包目标（自动合并平台配置）：
  - `src-tauri/tauri.windows.conf.json`：`msi` + `nsis(.exe)`
  - `src-tauri/tauri.macos.conf.json`：`dmg` + `app`
  - `src-tauri/tauri.linux.conf.json`：`deb` + `appimage`

## 构建前置依赖

### Linux (Ubuntu 24.04 / WSL2 Ubuntu)

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config libssl-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev librsvg2-dev \
  patchelf file
```

安装 Rust（推荐 rustup）：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

安装 Tauri CLI (2.x)：

```bash
cargo install tauri-cli --version "^2"
```

### Windows

在 **Windows 本机**构建（不能在 Linux/WSL 直接产出 MSI/EXE 安装包）：

- 安装 Rust（`x86_64-pc-windows-msvc`）
- 安装 Visual Studio Build Tools（C++ 工具链）
- 安装 WiX Toolset（用于 `msi`）与 NSIS（用于 `.exe` 安装包）

快速构建/运行脚本（项目根目录）：

- `build.cmd` / `run.cmd`（cmd.exe）
- `build.ps1` / `run.ps1`（PowerShell）

### macOS

在 **macOS 本机**构建（不能在 Linux/WSL 直接产出 DMG/APP 安装包）：

- 安装 Xcode Command Line Tools
- 安装 Rust（Apple targets）

## Release 打包

在项目根目录执行：

```bash
source "$HOME/.cargo/env"
cargo tauri build
```

产物输出到：

- `target/release/bundle/deb/`（`.deb`）
- `target/release/bundle/appimage/`（`.AppImage`）

在 Windows/macOS 上执行同样的 `cargo tauri build`，会按对应平台配置生成：

- Windows：`msi` / `nsis(.exe)`
- macOS：`dmg` / `app`

## 应用签名（如有证书）

签名涉及证书与平台工具链，需在对应系统上构建：

- Windows：在 `src-tauri/tauri.conf.json` 的 `bundle.windows` 中配置 `certificateThumbprint` / `timestampUrl` 等
- macOS：在 `src-tauri/tauri.conf.json` 的 `bundle.macOS` 中配置 `signingIdentity` / `providerShortName` 等

如需临时跳过签名：

```bash
cargo tauri build --no-sign
```

## 自动更新（可选）

当前配置 `bundle.createUpdaterArtifacts = false`。
如需生成更新包与签名，可将其改为 `true` 后重新 `cargo tauri build`，
并按 Tauri 2 的 updater 插件文档接入更新源与公钥。

## 验证（本机）

### 直接运行二进制

```bash
./target/release/rustreader
```

### 命令行打开指定文件/文件夹

支持：

- `rustreader <path>`
- `rustreader --open <path>` / `rustreader -o <path>`
- `rustreader --site-name <name>`（应用名称显示为 `rustreader - <name>`）

示例：

```bash
./target/release/rustreader /path/to/folder
./target/release/rustreader /path/to/file.md
./target/release/rustreader --site-name "我的站点" /path/to/folder
```

### 安装并启动（Linux .deb）

```bash
sudo dpkg -i ./target/release/bundle/deb/rustreader_1.0.0_amd64.deb
rustreader
```

卸载：

```bash
sudo dpkg -r rustreader
```
