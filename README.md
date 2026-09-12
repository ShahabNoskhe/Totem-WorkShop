# 🪽 Totem Workshop Pro (Rust & WebAssembly Edition)

> **Original Python Design & Implementation by:** @SirZigo  
> **Native Rust & WebAssembly Port:** High-Performance Dual-Target Edition

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/Platform-WebAssembly-654FF0.svg?style=flat-square&logo=webassembly)](https://webassembly.org/)
[![Minecraft](https://img.shields.io/badge/Minecraft-1.12%20--%201.21%2B-green.svg?style=flat-square&logo=minecraft)](https://minecraft.net/)
[![OS](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20Web-blue.svg?style=flat-square)](https://github.com/ridambezendegi4-hub/Totem-WorkShop)
[![License](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=flat-square)](LICENSE)

---

## 📖 معرفی پروژه | Overview

نرم‌افزار **Totem Workshop Pro** یک ابزار حرفه‌ای، سریع و همه‌فن‌حریف برای ساخت ریسورس‌پک‌های اختصاصی آیتم **توتم (Totem of Undying)** در بازی **Minecraft** است. این ابزار به گیمرها، سازندگان محتوا و ماد‌سازان اجازه می‌دهد در کمترین زمان ممکن ویدیوها و اسکین‌های مورد علاقه خود را به توتم‌های متحرک یا سه‌بعدی درون بازی تبدیل کنند.

این پروژه از زبان پایتون به زبان مدرن و فوق‌العاده سریع **Rust** بازنویسی شده و به صورت **Dual-Target** هم به عنوان یک برنامه اجرایی پرسرعت دسکتاپ (ویندوز و لینوکس) و هم به عنوان یک وب‌اپلیکیشن مستقل مبتنی بر **WebAssembly (Wasm)** بدون نیاز به هیچ‌گونه سرور کار می‌کند.

---

## ⚡ نسخه‌های پروژه | Available Flavors

پروژه به صورت سه بخش مجزا سازماندهی شده است:

`
Totem WorkShop/
├── Totem WorkShop ( Rust )/          # نسخه بومی دسکتاپ (Windows & Arch Linux)
├── Totem WorkShop ( Web Assembely )/ # نسخه تحت وب بدون نیاز به سرور (Wasm)
├── Totem WorkShop ( Python )/        # نسخه مرجع اولیه پایتون
└── note.txt                          # کدهای تجمیع شده پروژه در یک فایل
`

---

## 🎮 قابلیت‌های برجسته | Key Features

### ۱. ماژول Animated Totem (توتم متحرک از ویدیو)
- **پشتیبانی گسترده از فرمت‌های ویدیویی:** MP4, WebM, MOV, AVI, MKV و غیره.
- **استخراج هوشمند مشخصات ویدیو:** تشخیص خودکار نرخ فریم (FPS)، طول ویدیو و تعداد کل فریم‌ها.
- **حذف هوشمند پس‌زمینه (AI & Cutout):** جداسازی سوژه با فیلتر ترنسپرنسی و مدیاپایپ تا پس‌زمینه در دست بازیکن مکعبی نشود.
- **کنترل نرخ پرش و سرعت پخش:** قابلیت انتخاب فریم‌تایم استاندارد ۲۰ فریم بر ثانیه بدون افت فریم (FPS Drop) درون ماینکرفت.
- **تولید خودکار اسپرایت‌شیت (Vertical Spritesheet):** اتصال عمودی فریم‌ها و تولید فایل 	otem_of_undying.png.mcmeta.

### ۲. ماژول Skin-Forge (توتم از روی اسکین بازیکن)
- **دریافت خودکار از نام‌کاربری:** دانلود مستقیم اسکین با اتصال به سرورهای Mojang و Minotar.
- **پشتیبانی از Drag & Drop:** امکان کشیدن و رها کردن فایل اسکین محلی روی پنجره برنامه.
- **حالت ۲بعدی (2D Mode):** برش دقیق لایه‌های سر، دست‌ها، تنه و اعمال لایه‌های بیرونی (Outer Overlay).
- **حالت ۳بعدی (3D Mode):** ساخت مدل‌های سفارشی سه‌بعدی ماینکرفت (skin.json و 	otem_of_undying.json) با تنظیم زوایای دید اول‌شخص و سوم‌شخص.

### ۳. امکانات ریسورس‌پک
- **مبدل خودکار صدا:** تبدیل هر فرمت صوتی (mp3, wav, ogg) به فرمت صدای توتم ماینکرفت (use.ogg) به همراه sounds.json.
- **آیکون سفارشی پک:** تغییر اندازه خودکار تصویر به ابعاد استاندارد ۶۴×۶۴ پیکسل.
- **پالت رنگ‌های رسمی ماینکرفت:** درج آسان کدهای فرمت رنگی (§4, §c, §a, §e, §l, ...) در عنوان و توضیحات پک.
- **سازگاری با تمامی نسخه‌ها:** پشتیبانی از نسخه 1.12 تا 1.21+ و 26 (Pack Format 3 تا 84).

---

## 🚀 راهنمای اجرا | How to Run

### ۱. اجرای نسخه دسکتاپ (ویندوز)
کافیست وارد پوشه Totem WorkShop ( Rust ) شوید و فایل زیر را باز کنید:
`	ext
totem_workshop.exe
`
یا با دستور زیر از روی سورس‌کد اجرا کنید:
`powershell
cargo run --release
`

### ۲. اجرای نسخه دسکتاپ (آرچ لینوکس - Arch Linux)
وارد پوشه Totem WorkShop ( Rust )/Arch Linux شوید:
`ash
chmod +x build_and_run.sh
./build_and_run.sh
`
یا برای نصب به عنوان پکیج سیستمی:
`ash
makepkg -si
`

### ۳. اجرای نسخه تحت وب (WebAssembly)
* **در ویندوز:** داخل پوشه Totem WorkShop ( Web Assembely ) روی فایل Start_Web.bat دبل‌کلیک کنید.
* **در لینوکس / مک:** داخل پوشه Totem WorkShop ( Web Assembely )/Arch Linux اسکریپت start_web.sh را اجرا کنید:
  `ash
  chmod +x start_web.sh
  ./start_web.sh
  `
* سپس آدرس زیر را در مرورگر باز کنید:
  `	ext
  http://localhost:8080
  `

---

## 🌐 نحوه هاست کردن نسخه وب (Deploy to Web Host)

نسخه وب این نرم‌افزار **۱۰۰٪ کلاینت‌ساید (Client-side)** است؛ یعنی به هیچ پایگاه داده یا زبان سمت سروری (PHP، پایتون، نودجی‌اس) نیاز ندارد!

برای قرار دادن روی اینترنت:
کافیست تمامی محتویات پوشه Totem WorkShop ( Web Assembely )/dist را داخل:
- **GitHub Pages** (کاملاً رایگان)
- **Cloudflare Pages / Vercel / Netlify** (کاملاً رایگان)
- یا پوشه public_html در هاست‌های اشتراکی (cPanel، DirectAdmin، Nginx، Apache)
آپلود کنید.

---

## 📄 ساختار خروجی Resource Pack

پکیج تولید شده دارای ساختار استاندارد رسمی ماینکرفت است:

`
[نام انتخابی پک]/
├── pack.mcmeta                  # متادیتای فرمت نسخه و توضیحات رنگی
├── pack.png                     # تصویر آیکون پک (۶۴×۶۴)
└── assets/
    └── minecraft/
        ├── sounds.json          # تعریف رویداد صوتی use توتم
        ├── sounds/
        │   └── item/
        │       └── totem/
        │           └── use.ogg  # صدای اختصاصی پاپ شدن توتم
        ├── textures/
        │   └── item/
        │       ├── totem_of_undying.png         # تکسچر یا اسپرایت‌شیت انیمیشن
        │       └── totem_of_undying.png.mcmeta  # تعریف فریم‌ها و سرعت انیمیشن
        └── models/
            └── item/            # در صورت انتخاب حالت سه‌بعدی
                ├── totem_of_undying.json
                └── skin.json
`

---

## 📜 سازندگان و لایسنس | Credits & License

- **ایده، طراحی و پیاده‌سازی اولیه پایتون:** @SirZigo
- **پورت به زبان Rust، معماری Dual-Target و بهینه‌سازی Wasm:** توسط تیم توسعه
- **لایسنس:** پروژه تحت لایسنس آزاد [MIT](LICENSE) منتشر شده است.
