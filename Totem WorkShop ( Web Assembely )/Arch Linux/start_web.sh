#!/bin/bash
# ===================================================
#   Totem Workshop Pro 🪽 - WebAssembly Server (Linux)
#   Support: Arch Linux, Ubuntu, Debian, Fedora
#   Author: @CanBeShahab
# ===================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.." 2>/dev/null || cd "$SCRIPT_DIR"

echo "🪽 Launching Totem Workshop WebAssembly Local Server..."

PORT=8080
URL="http://localhost:$PORT"

open_browser() {
    sleep 1
    if command -v xdg-open &> /dev/null; then
        xdg-open "$URL" &
    elif command -v google-chrome &> /dev/null; then
        google-chrome "$URL" &
    elif command -v firefox &> /dev/null; then
        firefox "$URL" &
    fi
}

open_browser &

if command -v python3 &> /dev/null; then
    echo "[INFO] Running web server via Python 3 on $URL..."
    python3 -m http.server "$PORT" --directory dist
elif command -v python &> /dev/null; then
    echo "[INFO] Running web server via Python on $URL..."
    python -m http.server "$PORT" --directory dist
elif command -v trunk &> /dev/null; then
    echo "[INFO] Running web server via Trunk..."
    trunk serve --open
elif command -v npx &> /dev/null; then
    echo "[INFO] Running web server via npx serve..."
    npx serve dist -l "$PORT"
elif command -v miniserve &> /dev/null; then
    echo "[INFO] Running web server via miniserve..."
    miniserve dist -p "$PORT"
else
    echo "[ERROR] No web server found. Please install Python (pacman -S python) or NodeJS."
    exit 1
fi
