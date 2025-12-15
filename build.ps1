$ErrorActionPreference = 'Stop'

Set-Location -LiteralPath $PSScriptRoot

$cargoBin = Join-Path (Join-Path $env:USERPROFILE '.cargo') 'bin'
if (Test-Path -LiteralPath $cargoBin) {
  $env:PATH = $cargoBin + ';' + $env:PATH
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  Write-Error "cargo not found. Install Rust (rustup): https://www.rust-lang.org/tools/install"
}

try {
  & cargo tauri --version | Out-Null
} catch {
  Write-Error 'cargo-tauri not found. Install: cargo install tauri-cli --version "^2" --locked. If you see "feature edition2024 is required": rustup update stable (or use nightly).'
}

& cargo tauri build --no-bundle @Args
exit $LASTEXITCODE
