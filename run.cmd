@echo off
setlocal

cd /d "%~dp0"

set "EXE=%~dp0target\release\rustreader.exe"
if not exist "%EXE%" (
  echo [ERROR] "%EXE%" not found.
  echo         Run build.cmd first.
  exit /b 1
)

"%EXE%" %*
exit /b %errorlevel%
