# 🌐 Totem Workshop Pro (WebAssembly) - Linux Guide

## 🚀 راهنمای اجرای نسخه تحت وب در لینوکس / Arch Linux

### ۱. اجرای سرور محلی با یک دستور:
```bash
chmod +x start_web.sh
./start_web.sh
```
اسکریپت به صورت خودکار یک سرور لوکال روی پورت 8080 راه‌اندازی کرده و مرورگر پیش‌فرض شما را باز می‌کند.

### ۲. پیش‌نیازها:
کافیست پایتون روی سیستم شما نصب باشد:
```bash
sudo pacman -S python
```
یا در صورت داشتن NodeJS:
```bash
npx serve dist
```
