# 🪽 Totem Workshop Pro (Dual-Target Rust & WebAssembly Edition)

<div align="center">

[![Language English](https://img.shields.io/badge/Language-English-blue?style=for-the-badge)](#-english)
[![Language Persian](https://img.shields.io/badge/زبان-فارسی-green?style=for-the-badge)](#-فارسی)

> **Created & Maintained by:** `@CanBeShahab`  
> **Architecture:** Native High-Performance Rust & Serverless WebAssembly (Wasm)  
> **Target:** Minecraft 1.12 - 1.21+ (Pack Formats 3 to 84)

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/Platform-WebAssembly-654FF0.svg?style=flat-square&logo=webassembly)](https://webassembly.org/)
[![Minecraft](https://img.shields.io/badge/Minecraft-1.12%20--%201.21%2B-green.svg?style=flat-square&logo=minecraft)](https://minecraft.net/)
[![License](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=flat-square)](LICENSE)

</div>

---

# 🇬🇧 English

## 📖 Overview

**Totem Workshop Pro** is an all-in-one studio engineered to craft custom **Totem of Undying** resource packs for **Minecraft**. Designed for gamers, modders, and content creators, it transforms videos and player skins into fluid animated or custom 3D in-game totems within seconds.

The project features a high-performance **Dual-Target architecture**: it compiles natively as a standalone desktop executable (Windows & Linux) and as a 100% serverless **WebAssembly (Wasm)** web app that runs directly inside modern web browsers.

---

## ⚡ Project Flavors

```text
Totem WorkShop/
├── Totem WorkShop ( Rust )/          # Native Desktop Studio (Windows .exe & Linux)
├── Totem WorkShop ( Web Assembely )/ # Client-Side Web Studio (Wasm / HTML5 / JS)
├── Totem WorkShop ( Python )/        # Reference Python Prototype
└── note.txt                          # Monolithic source code bundle
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
* **2D Vanilla Mode:** Precise geometric slicing of skin layers (head, body, limbs, outer jacket) with vanilla totem contour transparency.
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

## 📖 معرفی جامع پروژه

نرم‌افزار **Totem Workshop Pro** یک استودیوی تخصصی و کامل برای بازیکنان، ماد‌سازان و تولیدکنندگان محتوای بازی **Minecraft** است. این ابزار فرآیند ساخت ریسورس‌پک‌های سفارشی برای آیتم **توتم (Totem of Undying)** را به یک فرآیند چندثانیه‌ای، بصری و فوق‌العاده سریع تبدیل می‌کند.

این نرم‌افزار به صورت **Dual-Target** بازسازی شده و همزمان به صورت فایل اجرایی دسکتاپ بومی و وب‌اپلیکیشن مستقل مبتنی بر **WebAssembly (Wasm)** بدون نیاز به هیچ سروری اجرا می‌شود.

---

## 🎮 قابلیت‌های برجسته

### ۱. ماژول Animated Totem (توتم متحرک از ویدیو)
* **پشتیبانی چندرسانه‌ای:** پشتیبانی از فرمت‌های `mp4`, `webm`, `mov`, `avi`, `mkv`, `flv`.
* **حذف پس‌زمینه با هوش مصنوعی BiRefNet:**
  * استفاده از مدل قدرتمند `BiRefNet-general-lite.onnx` برای جداسازی دقیق سوژه و جلوگیری از مکعبی شدن پس‌زمینه در دست بازیکن.
  * کش هوشمند مدل در حافظه مرورگر برای لود آنی.
  * پشتیبانی سریع MediaPipe و الگوریتم کروماکی چندگانه.
* **انیمیشن روان و بدون افت فریم (Anti-Lag):** پیش‌تنظیم ۱ تیک (۲۰ فریم بر ثانیه) و غیرفعال‌سازی پیش‌فرض تاری برای جلوگیری از لگ در بازی.
* **تولید خودکار اسپرایت‌شیت عمودی:** خروجی در ابعاد ۶۴، ۱۲۸ و ۲۵۶ پیکسل.

### ۲. ماژول Skin-Forge (توتم از روی اسکین بازیکن)
* **دریافت خودکار از نام‌کاربری:** دانلود مستقیم اسکین از سرورهای Mojang و Minotar.
* **پشتیبانی از Drag & Drop:** کشیدن و رها کردن فایل اسکین روی پنجره برنامه.
* **حالت ۲بعدی (2D Mode):** برش هندسی لایه‌ها و ترکیب کلاه و اورلی با کانتور توتم وانیلا.
* **حالت سه‌بعدی (3D Mode):** تولید مدل‌های سه‌بعدی وکسل `totem_of_undying.json` و `skin.json` با زاویه دید دقیق.

### ۳. امکانات عمومی ریسورس‌پک
* **مبدل خودکار صدا:** تبدیل هر نوع فایل صوتی به فرمت `use.ogg` توتم ماینکرفت.
* **آیکون سفارشی پک:** مقیاس‌بندی خودکار تصاویر به ۶۴×۶۴ پیکسل و ذخیره در `pack.png`.
* **پالت کدهای رنگی ماینکرفت:** درج آسان کدهای فرمت رنگی (`§`).
* **سازگاری با نسخه‌ها:** پشتیبانی از ماینکرفت 1.12 تا 1.21+ (Pack Format 3 تا 84).

---

## 📜 سازندگان و لایسنس | Credits & License

* **طراحی، پیاده‌سازی و نگهداری:** `@CanBeShahab`
* **لایسنس:** منتشر شده تحت مجوز آزاد [MIT License](LICENSE).
