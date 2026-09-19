# 🐧 Totem Workshop Pro - Arch Linux & Linux Guide

## 🚀 راهنمای اجرای آسان در لینوکس / Arch Linux

### ۱. اجرای مستقیم با اسکریپت خودکار (یک‌کلیکه):
کافیست در ترمینال دستور زیر را اجرا کنید:
```bash
chmod +x build_and_run.sh
./build_and_run.sh
```
این اسکریپت خودکار بسته‌های موردنیاز (`rust`, `ffmpeg`, `libxkbcommon`, `fontconfig`, `wayland`) را بررسی و نصب کرده و برنامه را به صورت بهینه کامپایل و اجرا می‌کند.

---

### ۲. نصب پکیج با makepkg (ویژه آرچ لینوکس):
```bash
makepkg -si
```

---

### ۳. نصب دستی پیش‌نیازها در آرچ لینوکس:
```bash
sudo pacman -S --needed rust ffmpeg libxkbcommon fontconfig wayland pkg-config
cargo run --release
```

### ۴. نصب پیش‌نیازها در اوبونتو / دبیان (Ubuntu / Debian):
```bash
sudo apt update
sudo apt install -y build-essential libfontconfig1-dev libasound2-dev libssl-dev libx11-xcb-dev libxkbcommon-dev libwayland-dev ffmpeg pkg-config
cargo run --release
```
