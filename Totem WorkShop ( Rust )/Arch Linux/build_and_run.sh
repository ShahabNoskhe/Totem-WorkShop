#!/bin/bash
# ===================================================
#   Totem Workshop Pro 🪽 - Desktop Studio for Linux
#   Support: Arch Linux, Debian, Ubuntu, Fedora
#   Author: @CanBeShahab
# ===================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.." 2>/dev/null || cd "$SCRIPT_DIR"

echo "🪽 Starting Totem Workshop Pro (Desktop Linux)..."

# Detect package manager and install dependencies
if command -v pacman &> /dev/null; then
    echo "[INFO] Detected Arch Linux environment (pacman)."
    MISSING_PKGS=()
    for pkg in rust ffmpeg pkg-config libxkbcommon fontconfig wayland; do
        if ! pacman -Qi "$pkg" &> /dev/null && ! pacman -Qg "$pkg" &> /dev/null; then
            MISSING_PKGS+=("$pkg")
        fi
    done
    if [ ${#MISSING_PKGS[@]} -gt 0 ]; then
        echo "[INFO] Installing required dependencies: ${MISSING_PKGS[*]}"
        sudo pacman -S --needed --noconfirm "${MISSING_PKGS[@]}"
    fi
elif command -v apt &> /dev/null; then
    echo "[INFO] Detected Debian/Ubuntu environment (apt)."
    sudo apt-get update -y
    sudo apt-get install -y build-essential libfontconfig1-dev libasound2-dev libssl-dev libx11-xcb-dev libxkbcommon-dev libwayland-dev ffmpeg pkg-config
elif command -v dnf &> /dev/null; then
    echo "[INFO] Detected Fedora environment (dnf)."
    sudo dnf install -y fontconfig-devel alsa-lib-devel openssl-devel libxkbcommon-devel wayland-devel ffmpeg pkgconf-pkg-config
fi

# Ensure Rust & Cargo are available
if ! command -v cargo &> /dev/null; then
    echo "[INFO] Cargo not found. Installing Rust toolchain via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "[INFO] Compiling and running Totem Workshop Pro (Release)..."
cargo run --release
