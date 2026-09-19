# 🪽 Totem Workshop Pro (Dual-Target Rust & WebAssembly Studio)

<div align="center">

[![GitHub Release](https://img.shields.io/github/v/release/ShahabNoskhe/Totem-WorkShop?color=brightgreen&style=for-the-badge&logo=github)](https://github.com/ShahabNoskhe/Totem-WorkShop/releases)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/Platform-WebAssembly-654FF0.svg?style=for-the-badge&logo=webassembly)](https://webassembly.org/)
[![Minecraft](https://img.shields.io/badge/Minecraft-1.12%20--%201.21%2B-green.svg?style=for-the-badge&logo=minecraft)](https://minecraft.net/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)

<br/>

[![Language English](https://img.shields.io/badge/Language-English-blue?style=flat-square)](#-english)
[![Language Persian](https://img.shields.io/badge/زبان-فارسی-green?style=flat-square)](#-فارسی)

> **Created & Designed by:** `@CanBeShahab`  
> **Official Repository:** [ShahabNoskhe/Totem-WorkShop](https://github.com/ShahabNoskhe/Totem-WorkShop)  
> **Architecture:** Native High-Performance Rust & Serverless WebAssembly (Wasm)  
> **Target:** Minecraft 1.12 - 1.21+ (Pack Formats 3 to 84)

</div>

---

# 🇬🇧 English

## 📖 Overview

**Totem Workshop Pro** is an all-in-one studio engineered to craft custom **Totem of Undying** resource packs for **Minecraft**. Designed for players, modders, and content creators, it transforms videos and player skins into fluid animated or custom 3D in-game totems within seconds.

The project features a high-performance **Dual-Target architecture**: it compiles natively as a standalone desktop executable (Windows `.exe` & Linux native) and as a 100% serverless **WebAssembly (Wasm)** web application that runs directly inside modern web browsers without requiring backend servers.

---

## ⚡ Project Structure

```text
Totem WorkShop/
├── Totem WorkShop ( Rust )/          # Native Desktop Studio (Windows .exe & Linux)
│   ├── totem_workshop.exe            # Standalone Windows Executable
│   ├── Run.bat                       # Desktop Quick Launcher
│   ├── Arch Linux/                   # Arch Linux build scripts & PKGBUILD
│   └── src/                          # Rust Source Code (egui / core engine)
├── Totem WorkShop ( Web Assembely )/ # Client-Side Web Studio (Wasm / HTML5 / JS)
│   ├── dist/                         # Production Web Build (HTML / JS / Wasm)
│   ├── Start_Web.bat                 # Local Web Server Launcher
│   └── index.html                    # Web Container & MediaPipe / ONNX Bridge
├── Totem WorkShop ( Python )/        # Reference Python Prototype (@CanBeShahab)
├── Totem_WorkShop_Repository.zip     # Full Repository Package Archive
└── README.md                         # Official Documentation
```

---

## 🎮 Key Features

### 1. Animated Totem Studio (Video to Animated Totem)
* **Broad Media Format Support:** `MP4`, `WebM`, `MOV`, `AVI`, `MKV`, `FLV`, `WMV`, `TS`, `OGV`.
* **Neural AI & Smart Background Removal:**
  * Direct integration of `BiRefNet-general-lite.onnx` (Swin Transformer v1 Tiny) for studio-grade subject cutout without edge artifacts.
  * Browser `CacheStorage` / `IndexedDB` caching for instant 0-second model loading.
  * Fast Google MediaPipe Selfie Segmentation and adaptive multi-corner chroma-key fallback.
* **Pro Video Specs Prober:** Automatic parsing of video duration, FPS, and total frame count.
* **Anti-Lag & Fluid Animation:**
  * Zero-lag default 20 FPS (1 Tick) fluid playback.
  * Disabled interpolation blur by default to eliminate FPS drops in Minecraft.
* **Vertical Spritesheet Generation:** Stitching frames into standard vertical sheets (64px, 128px, or 256px HD).

### 2. Skin-Forge Studio (Player Skin to Totem)
* **Instant Mojang API Lookup:** Fetch player skins directly from Mojang & Minotar servers.
* **Drag & Drop Upload:** Seamless file dropping onto the window.
* **2D Vanilla Totem Mode:** Slices skin layers into the authentic 16×16 totem texture layout matching standard cute Minecraft totems.
* **3D Voxel Model Mode:** Automatic generation of `totem_of_undying.json` and `skin.json` with calibrated first-person, third-person, and GUI item perspective transforms.

### 3. Resource Pack Ecosystem
* **Universal Audio Converter:** Automatic conversion of any audio file (`mp3`, `wav`, `ogg`, `aac`, `flac`) to Minecraft `use.ogg` with `sounds.json`.
* **Custom Pack Icon:** Auto-rescaling of any image to 64×64 PNG `pack.png`.
* **Official Minecraft Color Palette:** Fast insertion of color formatting codes (`§4`, `§c`, `§a`, `§e`, `§l`, etc.).
* **Universal Version Compatibility:** Supports Pack Formats 3 through 84 (Minecraft 1.12 up to 1.21+).

---

## 🚀 How to Run

### 1. Desktop Edition (Windows)
* Double click `totem_workshop.exe` or `Run.bat` inside `Totem WorkShop ( Rust )/`.
* Or build and run from source:
  ```powershell
  cd "Totem WorkShop ( Rust )"
  cargo run --release
  ```

### 2. Desktop Edition (Arch Linux)
```bash
cd "Totem WorkShop ( Rust )/Arch Linux"
chmod +x build_and_run.sh
./build_and_run.sh
```

### 3. Web Edition (Localhost)
* **Windows:** Double-click `Start_Web.bat` inside `Totem WorkShop ( Web Assembely )/`.
* **Linux:** Run `./start_web.sh` inside `Totem WorkShop ( Web Assembely )/Arch Linux/`.
* Open your browser at: `http://localhost:8080`

### 4. Deploying to Web Hosts
The web build in `Totem WorkShop ( Web Assembely )/dist/` is 100% client-side. You can host it anywhere for free:
* **GitHub Pages**
* **Cloudflare Pages / Vercel / Netlify**
* Any static web server (`nginx`, `apache`, `cPanel public_html`).

---

# 🇮🇷 فارسی

## 📖 معرفی پروژه

نرم‌افزار **Totem Workshop Pro** یک استودیوی تخصصی و کامل برای بازیکنان، ماد‌سازان و تولیدکنندگان محتوای بازی **Minecraft** است. این ابزار فرآیند پیچیده و زمان‌بر ساخت ریسورس‌پک‌های سفارشی برای **آیتم توتم (Totem of Undying)** را به یک فرآیند چندثانیه‌ای و کاملاً بصری تبدیل می‌کند.

پروژه دارای معماری **دوگانه (Dual-Target)** با زبان سریع و ایمن **Rust** و فریم‌ورک رابط کاربری **egui / eframe** با تم تاریک است. برنامه هم به صورت باینری بومی مستقل دسکتاپ (ویندوز و لینوکس) و هم به صورت وب‌اپلیکیشن بدون نیاز به سرور (**WebAssembly / Wasm**) اجرا می‌شود.

---

## ⚡ امکانات تخصصی

* **استودیوی توتم متحرک:** تبدیل ویدیوهای مختلف به توتم‌های متحرک با حذف هوشمند پس‌زمینه با هوش مصنوعی BiRefNet و MediaPipe.
* **استودیوی توتم از روی اسکین:** دریافت اسکین بازیکن با نام کاربری یا فایل محلی و تبدیل آن به توتم ۲بعدی کلاسیک یا مدل‌های ۳بعدی JSON.
* **تولید خودکار ریسورس‌پک:** ساخت فایل‌های `pack.mcmeta`، تبدیل صدا به `use.ogg`، تنظیم آیکون پک و پالت رنگی رسمی ماینکرفت.
* **پشتیبانی دو زبانه کامل:** پشتیبانی پیشرفته از زبان فارسی (BiDi Shaping راست‌به‌چپ هوشمند) و انگلیسی.

---

## 👨‍💻 سازنده و حقوق معنوی

* **طراحی و توسعه اولیه:** `@CanBeShahab`  
* **مخزن رسمی گیت‌هاب:** [ShahabNoskhe/Totem-WorkShop](https://github.com/ShahabNoskhe/Totem-WorkShop)
* **لایسنس:** انتشار یافته تحت مجوز متن‌باز MIT.
