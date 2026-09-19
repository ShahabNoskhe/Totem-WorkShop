@echo off
title Totem Workshop Pro 🪽 [Desktop]
cd /d "%~dp0"

echo ===================================================
echo   Totem Workshop Pro 🪽 - Desktop Studio
echo   Author: @CanBeShahab
echo ===================================================
echo.

if exist "totem_workshop.exe" (
    echo [INFO] Launching Totem Workshop Pro...
    start "" "totem_workshop.exe"
    exit /b 0
)

if exist "target\release\totem_workshop.exe" (
    echo [INFO] Launching release binary...
    start "" "target\release\totem_workshop.exe"
    exit /b 0
)

if exist "target\debug\totem_workshop.exe" (
    echo [INFO] Launching debug binary...
    start "" "target\debug\totem_workshop.exe"
    exit /b 0
)

echo [INFO] Building Totem Workshop Pro with Cargo (Release mode)...
cargo run --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Failed to launch Totem Workshop Pro.
    echo Please make sure Rust is installed: https://rustup.rs
    pause
)
