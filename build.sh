#!/bin/bash
set -euo pipefail

if [ -f "$HOME/.cargo/env" ]; then
	# shellcheck disable=SC1090
	source "$HOME/.cargo/env"
fi

if ! command -v cargo >/dev/null 2>&1; then
	echo "[ERROR] cargo not found. Install Rust (rustup) and reopen your terminal: https://www.rust-lang.org/tools/install" >&2
	exit 1
fi

if ! cargo tauri --version >/dev/null 2>&1; then
	echo "[ERROR] cargo-tauri not found." >&2
	echo "        Install: cargo install tauri-cli --version \"^2\" --locked" >&2
	echo "        If you see \"feature edition2024 is required\": update Rust (rustup update stable) or use nightly." >&2
	exit 1
fi

cargo tauri build --no-bundle
