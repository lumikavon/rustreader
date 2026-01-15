@echo off
setlocal

cd /d "%~dp0"

if exist "%USERPROFILE%\.cargo\bin" (
  set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
)

where cargo >nul 2>nul
if errorlevel 1 (
  echo [ERROR] cargo not found. Install Rust ^(rustup^) and reopen your terminal.
  echo         https://www.rust-lang.org/tools/install
  exit /b 1
)

cargo tauri --version >nul 2>nul
if errorlevel 1 (
  echo [ERROR] cargo-tauri not found.
  echo         Install: cargo install tauri-cli --version "^2" --locked
  echo         If you see "feature edition2024 is required": update Rust ^(rustup update stable^) or use nightly.
  exit /b 1
)

cargo tauri build %*
exit /b %errorlevel%
