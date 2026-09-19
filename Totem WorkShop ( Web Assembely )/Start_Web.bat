@echo off
title Totem Workshop Pro 🪽 [WebAssembly Local Server]
cd /d "%~dp0"

echo ===================================================
echo   Totem Workshop Pro 🪽 - WebAssembly Studio
echo   Author: @CanBeShahab
echo ===================================================
echo.

set PORT=8080
start http://localhost:%PORT%

where python >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Starting web server using Python on http://localhost:%PORT% ...
    python -m http.server %PORT% --directory dist
    exit /b 0
)

where py >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Starting web server using Python launcher on http://localhost:%PORT% ...
    py -m http.server %PORT% --directory dist
    exit /b 0
)

if exist "trunk.exe" (
    echo [INFO] Starting web server using Trunk on http://localhost:%PORT% ...
    trunk.exe serve --open
    exit /b 0
)

where npx >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Starting web server using npx serve on http://localhost:%PORT% ...
    npx serve dist -l %PORT%
    exit /b 0
)

echo [INFO] Starting Windows PowerShell Embedded Web Server on http://localhost:%PORT% ...
powershell -Command "$listener = New-Object System.Net.HttpListener; $listener.Prefixes.Add('http://localhost:%PORT%/'); $listener.Start(); Write-Host 'Serving at http://localhost:%PORT%/'; while($listener.IsListening) { $context = $listener.GetContext(); $path = Join-Path 'dist' $context.Request.Url.LocalPath.TrimStart('/'); if (!(Test-Path $path) -or (Get-Item $path).PSIsContainer) { $path = Join-Path 'dist' 'index.html' }; $bytes = [System.IO.File]::ReadAllBytes($path); $context.Response.ContentLength64 = $bytes.Length; $context.Response.OutputStream.Write($bytes, 0, $bytes.Length); $context.Response.Close() }"
