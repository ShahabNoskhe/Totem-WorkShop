@echo off
title Totem Workshop Pro
cd /d "%~dp0"

if exist "target\release\totem_workshop.exe" (
    start "" "target\release\totem_workshop.exe"
) else if exist "target\debug\totem_workshop.exe" (
    start "" "target\debug\totem_workshop.exe"
) else (
    echo [INFO] Building Totem Workshop Pro... Please wait...
    cargo run --release
)
