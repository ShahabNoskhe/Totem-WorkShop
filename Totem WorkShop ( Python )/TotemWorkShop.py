import customtkinter as ctk
from tkinter import filedialog, messagebox
import os
import sys
import threading
import shutil
import json
import subprocess
import requests
import re
from PIL import Image
from io import BytesIO

# --- تنظیمات ظاهر برنامه ---
ctk.set_appearance_mode("dark")
ctk.set_default_color_theme("blue")

# استایل مشترک برای امضای برنامه‌نویس
SIGNATURE_STYLE = {"font": ("Consolas", 12, "italic"), "text_color": "#555555"}

class TotemWorkshopPro(ctk.CTk):
    def __init__(self):
        super().__init__()
        self.title("Totem Workshop Pro 🪽 | Created by @SirZigo")
        self.geometry("1100x800")
        
        self.attributes('-alpha', 0.0)
        self.fade_in()

        # لود کردن تنظیمات ذخیره شده
        self.config_file = "totem_settings.json"
        self.settings = self.load_config()

        self.container = ctk.CTkFrame(self, fg_color="transparent")
        self.container.pack(fill="both", expand=True)
        
        self.load_dashboard()

    def load_config(self):
        if os.path.exists(self.config_file):
            try:
                with open(self.config_file, "r", encoding="utf-8") as f:
                    return json.load(f)
            except Exception:
                pass
        return {"output_dir": os.getcwd()}

    def save_config(self, key, value):
        self.settings[key] = value
        try:
            with open(self.config_file, "w", encoding="utf-8") as f:
                json.dump(self.settings, f, indent=4)
        except Exception:
            pass

    def fade_in(self):
        alpha = self.attributes('-alpha')
        if alpha < 1.0:
            self.attributes('-alpha', alpha + 0.05)
            self.after(30, self.fade_in)

    def clear_container(self):
        for widget in self.container.winfo_children():
            widget.destroy()

    def load_dashboard(self):
        self.clear_container()
        dash = ctk.CTkFrame(self.container, fg_color="transparent")
        dash.pack(fill="both", expand=True)
        
        ctk.CTkLabel(dash, text="✨ Totem Workshop Pro ✨", font=("Arial", 36, "bold"), text_color="#5DADE2").pack(pady=(70, 10))
        # امضا در صفحه اصلی زیر تایتل
        ctk.CTkLabel(dash, text="Created By : @SirZigo", **SIGNATURE_STYLE).pack(pady=(0, 40))
        
        cards_frame = ctk.CTkFrame(dash, fg_color="transparent")
        cards_frame.pack()

        btn_anim = ctk.CTkButton(
            cards_frame, text="✨ Animated Totem\n\n(Motion Studio & Video to Totem)", 
            width=320, height=220, corner_radius=25, 
            fg_color="#2E86C1", hover_color="#1B4F72",
            font=("Arial", 18, "bold"),
            command=self.load_animated_totem
        )
        btn_anim.pack(side="left", padx=25)

        btn_skin = ctk.CTkButton(
            cards_frame, text="🧊 2D & 3D Totem\n\n(Skin-Forge & Hand Preview)", 
            width=320, height=220, corner_radius=25, 
            fg_color="#27AE60", hover_color="#1E8449",
            font=("Arial", 18, "bold"),
            command=self.load_skin_totem
        )
        btn_skin.pack(side="left", padx=25)

    def load_animated_totem(self):
        self.clear_container()
        AnimatedTotemModule(self.container, self.load_dashboard, self)

    def load_skin_totem(self):
        self.clear_container()
        SkinForgeModule(self.container, self.load_dashboard, self)


# ============================================================
# ۱. ماژول کامل انیمیتد توتم (Animated Totem Module) 🎬
# ============================================================
class AnimatedTotemModule(ctk.CTkFrame):
    def __init__(self, parent, back_callback, app_instance):
        super().__init__(parent, fg_color="transparent")
        self.app = app_instance
        self.pack(fill="both", expand=True)
        
        self.grid_columnconfigure(0, weight=1)
        self.grid_columnconfigure(1, weight=1)
        self.grid_rowconfigure(0, weight=1) 
        
        self.video_path = ""
        self.icon_path = ""
        self.sound_path = ""
        self.output_dir = self.app.settings.get("output_dir", os.getcwd()) 

        # اینجا فقط اسکرول عمودی رو گذاشتم تا دیگه غیب نشه
        self.left_frame = ctk.CTkScrollableFrame(self, corner_radius=15, fg_color="#212121")
        self.left_frame.grid(row=0, column=0, padx=20, pady=20, sticky="nsew")

        ctk.CTkLabel(self.left_frame, text="⚙️ Pack Settings", font=("Arial", 20, "bold")).pack(pady=(10, 5))

        self.vid_btn = ctk.CTkButton(self.left_frame, text="🎬 1. Select Video", command=self.select_video)
        self.vid_btn.pack(pady=3)
        self.vid_label = ctk.CTkLabel(self.left_frame, text="No video selected", text_color="gray")
        self.vid_label.pack()

        self.icon_btn = ctk.CTkButton(self.left_frame, text="🖼️ 2. Select Pack Icon", command=self.select_icon)
        self.icon_btn.pack(pady=3)

        self.snd_btn = ctk.CTkButton(self.left_frame, text="🎵 3. Select Pop Sound", command=self.select_sound)
        self.snd_btn.pack(pady=3)
        self.snd_label = ctk.CTkLabel(self.left_frame, text="No sound selected", text_color="gray")
        self.snd_label.pack()

        self.out_btn = ctk.CTkButton(self.left_frame, text="📁 4. Save Location", command=self.select_output_dir)
        self.out_btn.pack(pady=3)
        
        disp_path = self.output_dir
        self.out_label = ctk.CTkLabel(self.left_frame, text="..." + disp_path[-32:] if len(disp_path) > 35 else disp_path, text_color="gray")
        self.out_label.pack()

        ctk.CTkLabel(self.left_frame, text="🌍 Minecraft Version:").pack(pady=(8, 0))
        
        self.version_mapping = {
            "1.12 (1.12 - 1.12.2)": 3, "1.13 (1.13 - 1.13.2)": 4, "1.14 (1.14 - 1.14.4)": 4,
            "1.15 (1.15 - 1.15.2)": 5, "1.16 (1.16 - 1.16.5)": 6, "1.17 (1.17 - 1.17.1)": 7,
            "1.18 (1.18 - 1.18.2)": 8, "1.19 (1.19 - 1.19.4)": 9, "1.20 (1.20 - 1.20.6)": 15,
            "1.21 (1.21 - 1.21.11)": 34, "26 (26.1+)": 84
        }
        
        self.version_var = ctk.StringVar(value="1.21 (1.21 - 1.21.11)")
        self.version_dropdown = ctk.CTkOptionMenu(
            self.left_frame, values=list(self.version_mapping.keys()),
            variable=self.version_var, width=280
        )
        self.version_dropdown.pack(pady=5)

        ctk.CTkLabel(self.left_frame, text="🎨 Minecraft Colors:").pack(pady=(5, 0))
        self.color_btns_frame = ctk.CTkFrame(self.left_frame, fg_color="transparent")
        self.color_btns_frame.pack(pady=(0, 5))

        mc_colors = [
            ("§4", "#AA0000"), ("§c", "#FF5555"), ("§6", "#FFAA00"), ("§e", "#FFFF55"), ("§2", "#00AA00"), ("§a", "#55FF55"), 
            ("§b", "#55FFFF"), ("§3", "#00AAAA"), ("§1", "#0000AA"), ("§9", "#5555FF"), ("§d", "#FF55FF"), ("§5", "#AA00AA"), 
            ("§f", "#FFFFFF"), ("§7", "#AAAAAA"), ("§8", "#555555"), ("§0", "#000000"), ("Bold(§l)", "#444444"), ("Reset(§r)", "#222222")
        ]

        row, col = 0, 0
        for name, hex_color in mc_colors:
            code = name.replace("Bold(", "").replace("Reset(", "").replace(")", "")
            light_colors = ["#FF5555", "#55FF55", "#55FFFF", "#FFFF55", "#FFAA00", "#FF55FF", "#FFFFFF", "#AAAAAA"]
            text_col = "black" if hex_color in light_colors else "white"
            
            btn = ctk.CTkButton(
                self.color_btns_frame, text=name, width=40, height=26, 
                fg_color=hex_color, hover_color=hex_color, text_color=text_col,
                font=("Arial", 11, "bold"),
                command=lambda c=code: self.insert_mc_color(c)
            )
            btn.grid(row=row, column=col, padx=2, pady=2)
            col += 1
            if col > 5:
                col = 0
                row += 1

        self.name_entry = ctk.CTkEntry(self.left_frame, placeholder_text="Pack Name (e.g. My Totem)", width=280)
        self.name_entry.pack(pady=3)

        self.desc_entry = ctk.CTkEntry(self.left_frame, placeholder_text="Pack Description (e.g. Dancing Angel)", width=280)
        self.desc_entry.pack(pady=3)

        self.last_focused_entry = self.name_entry
        self.name_entry.bind("<FocusIn>", lambda e: self.set_focus(self.name_entry))
        self.desc_entry.bind("<FocusIn>", lambda e: self.set_focus(self.desc_entry))

        self.frame_label = ctk.CTkLabel(self.left_frame, text="🎞️ Max Frames:")
        self.frame_label.pack(pady=(5, 0))

        self.frames_container = ctk.CTkFrame(self.left_frame, fg_color="transparent")
        self.frames_container.pack(pady=3)

        self.frame_preset_seg = ctk.CTkSegmentedButton(
            self.frames_container, values=["5", "15", "35", "75"], command=self.on_preset_click
        )
        self.frame_preset_seg.pack(side="left", padx=5)
        self.frame_preset_seg.set("35")

        self.frame_custom_entry = ctk.CTkEntry(self.frames_container, width=50)
        self.frame_custom_entry.pack(side="left", padx=5)
        self.frame_custom_entry.insert(0, "35")

        self.adv_frame = ctk.CTkFrame(self.left_frame, fg_color="transparent")
        self.adv_frame.pack(pady=3)

        self.canvas_var = ctk.StringVar(value="64 (Light)")
        self.canvas_dropdown = ctk.CTkOptionMenu(
            self.adv_frame, values=["64 (Light)", "128 (Normal)", "256 (Heavy!)"], variable=self.canvas_var, width=130
        )
        self.canvas_dropdown.grid(row=0, column=0, padx=5, pady=3)

        self.frametime_var = ctk.StringVar(value="2 (Normal Speed)")
        self.frametime_dropdown = ctk.CTkOptionMenu(
            self.adv_frame, values=["1 (Very Fast)", "2 (Normal Speed)", "3 (Slow)"], variable=self.frametime_var, width=130
        )
        self.frametime_dropdown.grid(row=0, column=1, padx=5, pady=3)

        self.skip_var = ctk.StringVar(value="1 (All Frames)")
        self.skip_dropdown = ctk.CTkOptionMenu(
            self.adv_frame, 
            values=["1 (All Frames)", "2 (Skip 1 Frame)", "3 (Skip 2 Frames)", "4 (Skip 3 Frames)"], 
            variable=self.skip_var, width=130
        )
        self.skip_dropdown.grid(row=1, column=0, columnspan=2, padx=5, pady=3)

        ctk.CTkButton(self.left_frame, text="🏠 Back to Dashboard", fg_color="#C0392B", hover_color="#962D22", height=35, command=back_callback).pack(fill="x", padx=20, pady=10)

        # ==========================================
        # ستون سمت راست: مانیتورینگ
        # ==========================================
        self.right_frame = ctk.CTkFrame(self)
        self.right_frame.grid(row=0, column=1, padx=(10, 20), pady=(20, 20), sticky="nsew")
        self.right_frame.grid_columnconfigure(0, weight=1)
        self.right_frame.grid_rowconfigure(3, weight=1) 

        self.title_label = ctk.CTkLabel(self.right_frame, text="🛠️ Animated Totem Generator", font=("Arial", 24, "bold"))
        self.title_label.grid(row=0, column=0, pady=(40, 20))

        self.progress_frame = ctk.CTkFrame(self.right_frame, fg_color="transparent")
        self.progress_frame.grid(row=1, column=0, pady=20)

        self.step_label = ctk.CTkLabel(self.progress_frame, text="Waiting for your magic... ✨", font=("Arial", 16, "bold"), text_color="cyan")
        self.step_label.pack(pady=10)

        self.progress_bar = ctk.CTkProgressBar(self.progress_frame, width=350, height=15)
        self.progress_bar.set(0)
        self.progress_bar.pack(pady=15)

        self.progress_text = ctk.CTkLabel(self.progress_frame, text="0 / 0", text_color="gray", font=("Arial", 14))
        self.progress_text.pack()

        self.start_btn = ctk.CTkButton(
            self.right_frame, text="🚀 GENERATE TOTEM", 
            fg_color="green", hover_color="darkgreen", height=50, width=250, font=("Arial", 16, "bold"),
            command=self.start_process
        )
        self.start_btn.grid(row=2, column=0, pady=40)

        ctk.CTkLabel(self.right_frame, text="Created By : @SirZigo", **SIGNATURE_STYLE).grid(row=4, column=0, pady=10, sticky="s")

    # ==========================================
    # توابع و لاجیک برنامه
    # ==========================================

    def set_focus(self, entry):
        self.last_focused_entry = entry

    def insert_mc_color(self, code):
        self.last_focused_entry.insert("end", code)

    def on_preset_click(self, val):
        self.frame_custom_entry.delete(0, "end")
        self.frame_custom_entry.insert(0, val)

    def select_video(self):
        path = filedialog.askopenfilename(filetypes=[("Video Files", "*.mp4 *.avi *.mov *.mkv")])
        if path:
            self.video_path = path
            try:
                import cv2
                cap = cv2.VideoCapture(path)
                fps = int(cap.get(cv2.CAP_PROP_FPS))
                frames = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
                cap.release()
                self.vid_label.configure(text=f"📂 {os.path.basename(path)}  |  {fps} FPS  |  {frames} Frames")
            except Exception:
                self.vid_label.configure(text=f"📂 {os.path.basename(path)}")

    def select_icon(self):
        path = filedialog.askopenfilename(filetypes=[("Image Files", "*.png *.jpg *.webp")])
        if path: self.icon_path = path

    def select_sound(self):
        path = filedialog.askopenfilename(filetypes=[("Audio Files", "*.mp3 *.ogg *.wav")])
        if path:
            self.sound_path = path
            self.snd_label.configure(text=f"🎵 {os.path.basename(path)}")

    def select_output_dir(self):
        path = filedialog.askdirectory(initialdir=self.output_dir)
        if path:
            self.output_dir = path
            self.out_label.configure(text="..." + path[-32:] if len(path) > 35 else path)
            self.app.save_config("output_dir", path)

    def start_process(self):
        if not self.video_path:
            messagebox.showerror("Error", "Brat video entekhab kon! 🎬")
            return
        self.start_btn.configure(state="disabled")
        threading.Thread(target=self.generate_totem).start()

    def generate_totem(self):
        try:
            try:
                import cv2
                from rembg import remove, new_session
            except ImportError:
                messagebox.showerror("Missing Packages ❌", "رفیق پکیج‌های cv2 یا rembg نصب نیستن!")
                self.step_label.configure(text="Missing packages...", text_color="orange")
                return

            raw_pack_name = self.name_entry.get().strip() or "Animated Totem Pack"
            clean_folder_name = re.sub(r'[<>:"/\\|?*]', '', raw_pack_name)
            desc = self.desc_entry.get() or "Animated Totem Pack"
            max_frames = int(self.frame_custom_entry.get().strip())
            
            selected_version = self.version_var.get()
            pack_format_val = self.version_mapping.get(selected_version, 34)

            target_size = 128 if "128" in self.canvas_var.get() else 256 if "256" in self.canvas_var.get() else 64
            frametime_val = int(self.frametime_var.get().split(" ")[0])
            frame_skip = int(self.skip_var.get().split(" ")[0])

            pack_full_path = os.path.join(self.output_dir, clean_folder_name)
            if os.path.exists(pack_full_path):
                shutil.rmtree(pack_full_path)
            item_dir = os.path.join(pack_full_path, "assets", "minecraft", "textures", "item")
            os.makedirs(item_dir, exist_ok=True)

            self.step_label.configure(text="✂️ Extracting Frames (Step 1/4)", text_color="cyan")
            cap = cv2.VideoCapture(self.video_path)
            extracted_frames, count, read_count = [], 0, 0
            
            while cap.isOpened() and count < max_frames:
                ret, frame = cap.read()
                if not ret: break
                if read_count % frame_skip == 0:
                    frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
                    extracted_frames.append(Image.fromarray(frame_rgb))
                    count += 1
                read_count += 1
            cap.release()

            self.step_label.configure(text="🔥 Removing Backgrounds (Step 2/4)")
            processed_images = []
            total_ext = len(extracted_frames)
            
            my_session = new_session()
            
            for i, img in enumerate(extracted_frames):
                bg_removed = remove(img, session=my_session, alpha_matting=True, post_process_mask=True)
                w, h = bg_removed.size
                min_dim = min(w, h)
                cropped = bg_removed.crop(((w-min_dim)/2, (h-min_dim)/2, (w+min_dim)/2, (h+min_dim)/2))
                processed_images.append(cropped.resize((target_size, target_size), Image.Resampling.LANCZOS))
                
                self.progress_bar.set((i + 1) / total_ext)
                self.progress_text.configure(text=f"{i + 1} / {total_ext}")

            self.step_label.configure(text="🛠️ Building Spritesheet (Step 3/4)")
            spritesheet = Image.new("RGBA", (target_size, len(processed_images) * target_size), (0, 0, 0, 0))
            for i, img in enumerate(processed_images):
                spritesheet.paste(img, (0, i * target_size))
            spritesheet.save(os.path.join(item_dir, "totem_of_undying.png"))

            self.step_label.configure(text="📝 Generating Configs (Step 4/4)")
            
            with open(os.path.join(item_dir, "totem_of_undying.png.mcmeta"), "w", encoding="utf-8") as f:
                json.dump({"animation": {"frametime": frametime_val, "interpolate": True, "frames": list(range(len(processed_images)))}}, f, indent=4, ensure_ascii=False)
            
            with open(os.path.join(pack_full_path, "pack.mcmeta"), "w", encoding="utf-8") as f:
                json.dump({"pack": {"pack_format": pack_format_val, "description": desc}}, f, indent=4, ensure_ascii=False)

            if self.icon_path:
                Image.open(self.icon_path).convert("RGBA").resize((64, 64)).save(os.path.join(pack_full_path, "pack.png"), format="PNG")
            
            if self.sound_path:
                sound_dir = os.path.join(pack_full_path, "assets", "minecraft", "sounds", "item", "totem")
                os.makedirs(sound_dir, exist_ok=True)
                dest_path = os.path.join(sound_dir, "use.ogg")
                if self.sound_path.lower().endswith(".ogg"):
                    shutil.copy(self.sound_path, dest_path)
                else:
                    try:
                        from pydub import AudioSegment
                        AudioSegment.converter = "ffmpeg"
                        audio = AudioSegment.from_file(self.sound_path)
                        audio.export(dest_path, format="ogg")
                    except Exception:
                        shutil.copy(self.sound_path, dest_path)
                
                sounds_json_path = os.path.join(pack_full_path, "assets", "minecraft", "sounds.json")
                with open(sounds_json_path, "w", encoding="utf-8") as f:
                    json.dump({
                        "item.totem.use": {
                            "sounds": ["item/totem/use"]
                        }
                    }, f, indent=4)

            self.step_label.configure(text="🎉 All Done! Pack is Ready!", text_color="green")
            self.progress_bar.set(1.0)
            messagebox.showinfo("Success!", f"Resource Pack is ready in:\n{self.output_dir} 🎉")
        except Exception as e:
            self.step_label.configure(text="❌ Error occurred!", text_color="red")
            messagebox.showerror("Error", str(e))
        finally:
            self.start_btn.configure(state="normal")


# ============================================================
# ۲. ماژول کامل 2D & 3D Totem (Skin-Forge Module) 🧊
# ============================================================
class SkinForgeModule(ctk.CTkFrame):
    def __init__(self, parent, back_callback, app_instance):
        super().__init__(parent, fg_color="transparent")
        self.app = app_instance
        self.pack(fill="both", expand=True)
        
        self.grid_columnconfigure(0, weight=1)
        self.grid_columnconfigure(1, weight=1)
        self.grid_rowconfigure(0, weight=1)
        
        self.icon_path = ""
        self.sound_path = ""
        self.skin_file_path = ""
        self.output_dir = self.app.settings.get("output_dir", os.getcwd())

        self.left_frame = ctk.CTkScrollableFrame(self, corner_radius=15, fg_color="#212121")
        self.left_frame.grid(row=0, column=0, padx=20, pady=20, sticky="nsew")

        ctk.CTkLabel(self.left_frame, text="🧊 Skin-Forge Studio", font=("Arial", 22, "bold"), text_color="#27AE60").pack(pady=(10, 15))

        ctk.CTkLabel(self.left_frame, text="Username:", font=("Arial", 11)).pack(anchor="w", padx=20)
        self.username_entry = ctk.CTkEntry(self.left_frame, placeholder_text="Enter Username (e.g. Notch)...", height=32)
        self.username_entry.pack(fill="x", padx=20, pady=3)
        ctk.CTkButton(self.left_frame, text="🔍 Fetch Skin by Username", height=32, fg_color="#2E86C1", command=self.fetch_skin).pack(fill="x", padx=20, pady=3)

        ctk.CTkLabel(self.left_frame, text="OR", text_color="gray", font=("Arial", 11)).pack(pady=5)

        ctk.CTkButton(self.left_frame, text="📂 Choose Skin File (.png)", height=32, fg_color="#34495E", command=self.select_skin_file).pack(fill="x", padx=20, pady=3)
        self.skin_file_label = ctk.CTkLabel(self.left_frame, text="No skin file chosen", text_color="gray", font=("Arial", 10))
        self.skin_file_label.pack()

        self.type_var = ctk.StringVar(value="2D")
        ctk.CTkLabel(self.left_frame, text="Totem Mode:").pack(pady=(10, 5))
        
        self.mode_seg = ctk.CTkSegmentedButton(self.left_frame, values=["2D", "3D"], variable=self.type_var, command=self.toggle_preview_mode)
        self.mode_seg.pack(pady=5)

        ctk.CTkLabel(self.left_frame, text="--- Pack Settings ---", font=("Arial", 14, "bold"), text_color="gray").pack(pady=(15, 10))

        self.icon_btn = ctk.CTkButton(self.left_frame, text="🖼️ Select Pack Icon", command=self.select_icon, height=30)
        self.icon_btn.pack(pady=2, fill="x", padx=20)

        self.snd_btn = ctk.CTkButton(self.left_frame, text="🎵 Select Pop Sound", command=self.select_sound, height=30)
        self.snd_btn.pack(pady=2, fill="x", padx=20)
        self.snd_label = ctk.CTkLabel(self.left_frame, text="No sound selected", text_color="gray", font=("Arial", 10))
        self.snd_label.pack()

        self.out_btn = ctk.CTkButton(self.left_frame, text="📁 Save Location", command=self.select_output_dir, height=30)
        self.out_btn.pack(pady=2, fill="x", padx=20)
        
        disp_path = self.output_dir
        self.out_label = ctk.CTkLabel(self.left_frame, text="..." + disp_path[-32:] if len(disp_path) > 35 else disp_path, text_color="gray", font=("Arial", 10))
        self.out_label.pack()

        ctk.CTkLabel(self.left_frame, text="🌍 Minecraft Version:", font=("Arial", 11)).pack(pady=(10, 0))
        self.version_mapping = {
            "1.12 (1.12 - 1.12.2)": 3, "1.13 (1.13 - 1.13.2)": 4, "1.14 (1.14 - 1.14.4)": 4,
            "1.15 (1.15 - 1.15.2)": 5, "1.16 (1.16 - 1.16.5)": 6, "1.17 (1.17 - 1.17.1)": 7,
            "1.18 (1.18 - 1.18.2)": 8, "1.19 (1.19 - 1.19.4)": 9, "1.20 (1.20 - 1.20.6)": 15,
            "1.21 (1.21 - 1.21.11)": 34, "26 (26.1+)": 84
        }
        self.version_var = ctk.StringVar(value="1.21 (1.21 - 1.21.11)")
        self.version_dropdown = ctk.CTkOptionMenu(self.left_frame, values=list(self.version_mapping.keys()), variable=self.version_var, width=240, height=26)
        self.version_dropdown.pack(pady=2)

        ctk.CTkLabel(self.left_frame, text="🎨 Minecraft Colors:", font=("Arial", 11)).pack(pady=(10, 0))
        self.color_btns_frame = ctk.CTkFrame(self.left_frame, fg_color="transparent")
        self.color_btns_frame.pack(pady=(0, 2))
        mc_colors = [
            ("§4", "#AA0000"), ("§c", "#FF5555"), ("§6", "#FFAA00"), ("§e", "#FFFF55"), ("§2", "#00AA00"), ("§a", "#55FF55"), 
            ("§b", "#55FFFF"), ("§3", "#00AAAA"), ("§1", "#0000AA"), ("§9", "#5555FF"), ("§d", "#FF55FF"), ("§5", "#AA00AA"), 
            ("§f", "#FFFFFF"), ("§7", "#AAAAAA"), ("§8", "#555555"), ("§0", "#000000"), ("Bold(§l)", "#444444"), ("Reset(§r)", "#222222")
        ]
        row, col = 0, 0
        for name, hex_color in mc_colors:
            code = name.replace("Bold(", "").replace("Reset(", "").replace(")", "")
            light_cols = ["#FF5555", "#55FF55", "#55FFFF", "#FFFF55", "#FFAA00", "#FF55FF", "#FFFFFF", "#AAAAAA"]
            text_col = "black" if hex_color in light_cols else "white"
            btn = ctk.CTkButton(self.color_btns_frame, text=name, width=32, height=20, fg_color=hex_color, hover_color=hex_color, text_color=text_col, font=("Arial", 10, "bold"), command=lambda c=code: self.insert_mc_color(c))
            btn.grid(row=row, column=col, padx=1, pady=1)
            col += 1
            if col > 5: col = 0; row += 1

        self.name_entry = ctk.CTkEntry(self.left_frame, placeholder_text="Pack Name (e.g. My Skin Totem)", width=240, height=26)
        self.name_entry.pack(pady=(10, 2))
        self.desc_entry = ctk.CTkEntry(self.left_frame, placeholder_text="Pack Description", width=240, height=26)
        self.desc_entry.pack(pady=2)

        self.last_focused_entry = self.name_entry
        self.name_entry.bind("<FocusIn>", lambda e: self.set_focus(self.name_entry))
        self.desc_entry.bind("<FocusIn>", lambda e: self.set_focus(self.desc_entry))

        ctk.CTkButton(self.left_frame, text="🏠 Back to Dashboard", fg_color="#C0392B", hover_color="#962D22", height=30, command=back_callback).pack(fill="x", padx=20, pady=20)

        self.right_frame = ctk.CTkFrame(self, corner_radius=15, fg_color="#212121")
        self.right_frame.grid(row=0, column=1, padx=(10, 20), pady=20, sticky="nsew")
        self.right_frame.grid_columnconfigure(0, weight=1)
        self.right_frame.grid_rowconfigure(3, weight=1) 

        ctk.CTkLabel(self.right_frame, text="🤲 Hand Preview & Totem", font=("Arial", 18, "bold")).pack(pady=20)
        
        self.hand_preview = ctk.CTkLabel(self.right_frame, text="[No Skin Loaded]\nSearch Username or Choose File", width=250, height=250, fg_color="#181818", corner_radius=20, text_color="gray")
        self.hand_preview.pack(pady=10)

        self.skin_progress = ctk.CTkProgressBar(self.right_frame, width=300)
        self.skin_progress.pack(pady=20)
        self.skin_progress.set(0)

        ctk.CTkButton(self.right_frame, text="🚀 GENERATE SKIN TOTEM", fg_color="#27AE60", hover_color="#1E8449", height=50, font=("Arial", 16, "bold"), command=self.generate_skin_totem).pack(pady=20)

        ctk.CTkLabel(self.right_frame, text="Created By : @SirZigo", **SIGNATURE_STYLE).grid(row=4, column=0, pady=10, sticky="s")

    def toggle_preview_mode(self, mode):
        if not hasattr(self, 'preview_2d') or not hasattr(self, 'preview_3d'):
            return
            
        if mode == "2D":
            img_thumb = ctk.CTkImage(light_image=self.preview_2d, dark_image=self.preview_2d, size=(160, 160))
        else:
            img_thumb = ctk.CTkImage(light_image=self.preview_3d, dark_image=self.preview_3d, size=(120, 240))
            
        self.hand_preview.configure(image=img_thumb, text="")

    def set_focus(self, entry):
        self.last_focused_entry = entry

    def insert_mc_color(self, code):
        self.last_focused_entry.insert("end", code)

    def select_icon(self):
        path = filedialog.askopenfilename(filetypes=[("Image Files", "*.png *.jpg *.webp")])
        if path: self.icon_path = path

    def select_sound(self):
        path = filedialog.askopenfilename(filetypes=[("Audio Files", "*.mp3 *.ogg *.wav")])
        if path:
            self.sound_path = path
            self.snd_label.configure(text=f"🎵 {os.path.basename(path)}")

    def select_skin_file(self):
        path = filedialog.askopenfilename(filetypes=[("Skin Image", "*.png")])
        if path:
            self.skin_file_path = path
            self.skin_file_label.configure(text=f"📂 {os.path.basename(path)}")
            img = Image.open(path).convert("RGBA")
            self.process_and_preview_skin(img)

    def select_output_dir(self):
        path = filedialog.askdirectory(initialdir=self.output_dir)
        if path:
            self.output_dir = path
            self.out_label.configure(text="..." + path[-32:] if len(path) > 35 else path)
            self.app.save_config("output_dir", path)

    def fetch_skin(self):
        username = self.username_entry.get().strip()
        if not username:
            messagebox.showerror("Error", "Please enter a Minecraft username!")
            return
        threading.Thread(target=self._get_skin_thread, args=(username,)).start()

    def _get_skin_thread(self, username):
        try:
            self.skin_progress.set(0.3)
            uuid_res = requests.get(f"https://api.mojang.com/users/profiles/minecraft/{username}", timeout=5)
            if uuid_res.status_code != 200:
                raise Exception("Player not found!")
            uuid = uuid_res.json()["id"]
            
            self.skin_progress.set(0.6)
            skin_res = requests.get(f"https://sessionserver.mojang.com/session/minecraft/profile/{uuid}", timeout=5)
            import base64
            texture_data = json.loads(base64.b64decode(skin_res.json()["properties"][0]["value"]))
            skin_url = texture_data["textures"]["SKIN"]["url"]
            
            self.skin_progress.set(0.9)
            img_res = requests.get(skin_url, timeout=5)
            img = Image.open(BytesIO(img_res.content)).convert("RGBA")
            
            self.process_and_preview_skin(img)
            self.skin_progress.set(1.0)
            messagebox.showinfo("Success", f"Skin for '{username}' loaded successfully! ✨")
        except Exception as e:
            self.skin_progress.set(0)
            messagebox.showerror("Internet Error", f"Could not fetch skin. Make sure you have internet!\nError: {e}")

    def process_and_preview_skin(self, skin_img):
        skin_img = skin_img.convert("RGBA")
        is_new_skin = skin_img.size[1] == 64
        
        if not is_new_skin:
            new_skin = Image.new("RGBA", (64, 64), (0, 0, 0, 0))
            new_skin.paste(skin_img, (0, 0))
            self.downloaded_skin = new_skin
        else:
            self.downloaded_skin = skin_img

        # =========================================================
        # ساخت پیش‌نمایش 2D
        # =========================================================
        totem_img = Image.new("RGBA", (16, 16), (0, 0, 0, 0))
        
        head_2d = skin_img.crop((8, 8, 16, 16))         
        body_2d = skin_img.crop((20, 20, 28, 27))       
        arm_r_2d = skin_img.crop((44, 20, 48, 24))  
        
        if is_new_skin:
            arm_l_2d = skin_img.crop((36, 52, 40, 56)) 
        else:
            arm_l_2d = arm_r_2d.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            
        head_ov_2d = skin_img.crop((40, 8, 48, 16))
        if is_new_skin:
            body_ov_2d = skin_img.crop((20, 36, 28, 43))
            arm_r_ov_2d = skin_img.crop((44, 36, 48, 40))
            arm_l_ov_2d = skin_img.crop((52, 52, 56, 56))
        
        totem_img.paste(arm_r_2d, (0, 9))
        totem_img.paste(body_2d, (4, 9))
        totem_img.paste(arm_l_2d, (12, 9))
        totem_img.paste(head_2d, (4, 1))
        
        totem_img.paste(head_ov_2d, (4, 1), head_ov_2d)
        if is_new_skin:
            totem_img.paste(arm_r_ov_2d, (0, 9), arm_r_ov_2d)
            totem_img.paste(body_ov_2d, (4, 9), body_ov_2d)
            totem_img.paste(arm_l_ov_2d, (12, 9), arm_l_ov_2d)
            
        pixels_to_clear = [
            (0, 10), (15, 10),                                                            
            (0, 11), (1, 11), (14, 11), (15, 11),                                         
            (0, 12), (1, 12), (2, 12), (3, 12), (12, 12), (13, 12), (14, 12), (15, 12),   
            (4, 13), (11, 13),                                                            
            (4, 14), (5, 14), (10, 14), (11, 14),                                         
            (4, 15), (5, 15), (10, 15), (11, 15)                                          
        ]
        for px in pixels_to_clear:
            totem_img.putpixel(px, (0, 0, 0, 0))
            
        self.final_totem_image = totem_img
        self.preview_2d = totem_img.resize((160, 160), Image.Resampling.NEAREST)

        # =========================================================
        # ساخت پیش‌نمایش 3D
        # =========================================================
        preview_3d_img = Image.new("RGBA", (16, 32), (0, 0, 0, 0))
        
        head_3d = skin_img.crop((8, 8, 16, 16))
        body_3d = skin_img.crop((20, 20, 28, 32))
        arm_r_3d = skin_img.crop((44, 20, 48, 32))
        leg_r_3d = skin_img.crop((4, 20, 8, 32))
        
        if is_new_skin:
            arm_l_3d = skin_img.crop((36, 52, 40, 64))
            leg_l_3d = skin_img.crop((20, 52, 24, 64))
        else:
            arm_l_3d = arm_r_3d.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            leg_l_3d = leg_r_3d.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            
        preview_3d_img.paste(arm_r_3d, (0, 8))
        preview_3d_img.paste(body_3d, (4, 8))
        preview_3d_img.paste(arm_l_3d, (12, 8))
        preview_3d_img.paste(leg_r_3d, (4, 20))
        preview_3d_img.paste(leg_l_3d, (8, 20))
        preview_3d_img.paste(head_3d, (4, 0))
        
        head_ov_3d = skin_img.crop((40, 8, 48, 16))
        preview_3d_img.paste(head_ov_3d, (4, 0), head_ov_3d)
        
        if is_new_skin:
            body_ov_3d = skin_img.crop((20, 36, 28, 48))
            arm_r_ov_3d = skin_img.crop((44, 36, 48, 48))
            arm_l_ov_3d = skin_img.crop((52, 52, 56, 64))
            leg_r_ov_3d = skin_img.crop((4, 36, 8, 48))
            leg_l_ov_3d = skin_img.crop((4, 52, 8, 64))
            
            preview_3d_img.paste(arm_r_ov_3d, (0, 8), arm_r_ov_3d)
            preview_3d_img.paste(body_ov_3d, (4, 8), body_ov_3d)
            preview_3d_img.paste(arm_l_ov_3d, (12, 8), arm_l_ov_3d)
            preview_3d_img.paste(leg_r_ov_3d, (4, 20), leg_r_ov_3d)
            preview_3d_img.paste(leg_l_ov_3d, (8, 20), leg_l_ov_3d)

        self.preview_3d = preview_3d_img.resize((120, 240), Image.Resampling.NEAREST)

        self.toggle_preview_mode(self.type_var.get())

    def generate_skin_totem(self):
        if not hasattr(self, 'downloaded_skin'):
            messagebox.showerror("Error", "Please search username or choose a skin file first! 🔍")
            return
        try:
            raw_pack_name = self.name_entry.get().strip() or "Skin Totem Pack"
            clean_folder_name = re.sub(r'[<>:"/\\|?*]', '', raw_pack_name)
            desc = self.desc_entry.get() or "Skin Totem Pack"
            pack_format_val = self.version_mapping.get(self.version_var.get(), 34)

            pack_full_path = os.path.join(self.output_dir, clean_folder_name)
            if os.path.exists(pack_full_path): shutil.rmtree(pack_full_path)
            item_dir = os.path.join(pack_full_path, "assets", "minecraft", "textures", "item")
            os.makedirs(item_dir, exist_ok=True)
            
            mode = self.type_var.get()
            
            if mode == "2D":
                self.final_totem_image.save(os.path.join(item_dir, "totem_of_undying.png"))
            else:
                self.downloaded_skin.save(os.path.join(item_dir, "totem_of_undying.png"))
                
                models_dir = os.path.join(pack_full_path, "assets", "minecraft", "models", "item")
                os.makedirs(models_dir, exist_ok=True)
                
                totem_model = {
                    "parent": "minecraft:item/skin",
                    "textures": {
                        "layer0": "minecraft:item/totem_of_undying"
                    }
                }
                with open(os.path.join(models_dir, "totem_of_undying.json"), "w", encoding="utf-8") as f:
                    json.dump(totem_model, f, indent=4)
                    
                skin_model = {
                    "texture_size": [64, 64],
                    "textures": {"0": "item/totem_of_undying"},
                    "elements": [
                        {"from": [-4, 24, -4], "to": [4, 32, 4], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [2, 2, 4, 4], "texture": "#0"}, "east": {"uv": [0, 2, 2, 4], "texture": "#0"}, "south": {"uv": [6, 2, 8, 4], "texture": "#0"}, "west": {"uv": [4, 2, 6, 4], "texture": "#0"}, "up": {"uv": [4, 2, 2, 0], "texture": "#0"}, "down": {"uv": [6, 0, 4, 2], "texture": "#0"}}},
                        {"from": [-4.5, 23, -4.5], "to": [4.5, 32, 4.5], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [10, 2, 12, 4], "texture": "#0"}, "east": {"uv": [8, 2, 10, 4], "texture": "#0"}, "south": {"uv": [14, 2, 16, 4], "texture": "#0"}, "west": {"uv": [12, 2, 14, 4], "texture": "#0"}, "up": {"uv": [12, 2, 10, 0], "texture": "#0"}, "down": {"uv": [14, 0, 12, 2], "texture": "#0"}}},
                        {"from": [-4, 12, -2], "to": [4, 24, 2], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [5, 5, 7, 8], "texture": "#0"}, "east": {"uv": [4, 5, 5, 8], "texture": "#0"}, "south": {"uv": [8, 5, 10, 8], "texture": "#0"}, "west": {"uv": [7, 5, 8, 8], "texture": "#0"}, "up": {"uv": [7, 5, 5, 4], "texture": "#0"}, "down": {"uv": [9, 4, 7, 5], "texture": "#0"}}},
                        {"from": [-4.25, 11.75, -2.25], "to": [4.25, 24.25, 2.25], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [5, 9, 7, 12], "texture": "#0"}, "east": {"uv": [4, 9, 5, 12], "texture": "#0"}, "south": {"uv": [8, 9, 10, 12], "texture": "#0"}, "west": {"uv": [7, 9, 8, 12], "texture": "#0"}, "up": {"uv": [7, 9, 5, 8], "texture": "#0"}, "down": {"uv": [9, 8, 7, 9], "texture": "#0"}}},
                        {"from": [4, 12, -2], "to": [8, 24, 2], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0.5, 0]}, "faces": {"north": {"uv": [11, 5, 12, 8], "texture": "#0"}, "east": {"uv": [10, 5, 11, 8], "texture": "#0"}, "south": {"uv": [13, 5, 14, 8], "texture": "#0"}, "west": {"uv": [12, 5, 13, 8], "texture": "#0"}, "up": {"uv": [12, 5, 11, 4], "texture": "#0"}, "down": {"uv": [13, 4, 12, 5], "texture": "#0"}}},
                        {"from": [3.75, 11.75, -2.25], "to": [8.25, 24.25, 2.25], "rotation": {"angle": 0, "axis": "y", "origin": [5.5, 18, 0]}, "faces": {"north": {"uv": [11, 9, 12, 12], "texture": "#0"}, "east": {"uv": [10, 9, 11, 12], "texture": "#0"}, "south": {"uv": [13, 9, 14, 12], "texture": "#0"}, "west": {"uv": [12, 9, 13, 12], "texture": "#0"}, "up": {"uv": [12, 9, 11, 8], "texture": "#0"}, "down": {"uv": [13, 8, 12, 9], "texture": "#0"}}},
                        {"from": [-8, 12, -2], "to": [-4, 24, 2], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0.5, 0]}, "faces": {"north": {"uv": [9, 13, 10, 16], "texture": "#0"}, "east": {"uv": [8, 13, 9, 16], "texture": "#0"}, "south": {"uv": [11, 13, 12, 16], "texture": "#0"}, "west": {"uv": [10, 13, 11, 16], "texture": "#0"}, "up": {"uv": [10, 13, 9, 12], "texture": "#0"}, "down": {"uv": [11, 12, 10, 13], "texture": "#0"}}},
                        {"from": [-8.25, 11.75, -2.25], "to": [-3.75, 24.25, 2.25], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0.5, 0]}, "faces": {"north": {"uv": [13, 13, 14, 16], "texture": "#0"}, "east": {"uv": [12, 13, 13, 16], "texture": "#0"}, "south": {"uv": [15, 13, 16, 16], "texture": "#0"}, "west": {"uv": [14, 13, 15, 16], "texture": "#0"}, "up": {"uv": [14, 13, 13, 12], "texture": "#0"}, "down": {"uv": [15, 12, 14, 13], "texture": "#0"}}},
                        {"from": [-0.1, 0, -2], "to": [3.9, 12, 2], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [1, 5, 2, 8], "texture": "#0"}, "east": {"uv": [0, 5, 1, 8], "texture": "#0"}, "south": {"uv": [3, 5, 4, 8], "texture": "#0"}, "west": {"uv": [2, 5, 3, 8], "texture": "#0"}, "up": {"uv": [2, 5, 1, 4], "texture": "#0"}, "down": {"uv": [3, 4, 2, 5], "texture": "#0"}}},
                        {"from": [-0.35, -0.25, -2.25], "to": [4.15, 12.25, 2.25], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [1, 9, 2, 12], "texture": "#0"}, "east": {"uv": [0, 9, 1, 12], "texture": "#0"}, "south": {"uv": [3, 9, 4, 12], "texture": "#0"}, "west": {"uv": [2, 9, 3, 12], "texture": "#0"}, "up": {"uv": [2, 9, 1, 8], "texture": "#0"}, "down": {"uv": [3, 8, 2, 9], "texture": "#0"}}},
                        {"from": [-3.9, 0, -2], "to": [0.1, 12, 2], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [5, 13, 6, 16], "texture": "#0"}, "east": {"uv": [4, 13, 5, 16], "texture": "#0"}, "south": {"uv": [7, 13, 8, 16], "texture": "#0"}, "west": {"uv": [6, 13, 7, 16], "texture": "#0"}, "up": {"uv": [6, 13, 5, 12], "texture": "#0"}, "down": {"uv": [7, 12, 6, 13], "texture": "#0"}}},
                        {"from": [-4.15, -0.25, -2.25], "to": [0.35, 12.25, 2.25], "rotation": {"angle": 0, "axis": "y", "origin": [0, 0, 0]}, "faces": {"north": {"uv": [1, 13, 2, 16], "texture": "#0"}, "east": {"uv": [0, 13, 1, 16], "texture": "#0"}, "south": {"uv": [3, 13, 4, 16], "texture": "#0"}, "west": {"uv": [2, 13, 3, 16], "texture": "#0"}, "up": {"uv": [2, 13, 1, 12], "texture": "#0"}, "down": {"uv": [3, 12, 2, 13], "texture": "#0"}}}
                    ],
                    "display": {
                        "thirdperson_righthand": {"rotation": [0, 90, 0], "translation": [1.75, -0.25, 0.25], "scale": [0.2, 0.2, 0.2]},
                        "thirdperson_lefthand": {"rotation": [0, 90, 0], "translation": [1.75, -0.25, 3.5], "scale": [0.2, 0.2, 0.2]},
                        "firstperson_righthand": {"rotation": [0, 103, 0], "translation": [-0.5, 3, 2], "scale": [0.2, 0.2, 0.2]},
                        "firstperson_lefthand": {"rotation": [0, 103, 0], "translation": [0, 3, 5], "scale": [0.2, 0.2, 0.2]},
                        "ground": {"translation": [1.5, 0, 1.5], "scale": [0.19, 0.2, 0.2]},
                        "gui": {"rotation": [0, -180, 0], "translation": [-4.25, -4.25, 0], "scale": [0.5, 0.5, 0.49]},
                        "head": {"translation": [1.5, 8, -3], "scale": [0.2, 0.2, 0.2]},
                        "fixed": {"rotation": [-5, 0, 0], "translation": [3, -3, 2], "scale": [0.4, 0.4, 0.4]}
                    },
                    "groups": [
                        {"name": "skin", "origin": [8, 8, 8], "children": [
                            {"name": "skin", "origin": [8, 8, 8], "children": [
                                {"name": "skin", "origin": [8, 8, 8], "children": [
                                    {"name": "Head", "origin": [0, 24, 0], "children": [0, 1]},
                                    {"name": "Body", "origin": [0, 24, 0], "children": [2, 3]},
                                    {"name": "RightArm", "origin": [5, 21.5, 0], "children": [4, 5]},
                                    {"name": "LeftArm", "origin": [-5, 21.5, 0], "children": [6, 7]},
                                    {"name": "RightLeg", "origin": [1.9, 12, 0], "children": [8, 9]},
                                    {"name": "LeftLeg", "origin": [-1.9, 12, 0], "children": [10, 11]}
                                ]}
                            ]}
                        ]}
                    ]
                }
                with open(os.path.join(models_dir, "skin.json"), "w", encoding="utf-8") as f:
                    json.dump(skin_model, f, indent=4)

            with open(os.path.join(pack_full_path, "pack.mcmeta"), "w", encoding="utf-8") as f:
                json.dump({"pack": {"pack_format": pack_format_val, "description": desc}}, f, indent=4, ensure_ascii=False)

            if hasattr(self, 'icon_path') and self.icon_path:
                Image.open(self.icon_path).convert("RGBA").resize((64, 64)).save(os.path.join(pack_full_path, "pack.png"), format="PNG")

            if hasattr(self, 'sound_path') and self.sound_path:
                sound_dir = os.path.join(pack_full_path, "assets", "minecraft", "sounds", "item", "totem")
                os.makedirs(sound_dir, exist_ok=True)
                dest_sound_path = os.path.join(sound_dir, "use.ogg")
                if self.sound_path.lower().endswith(".ogg"):
                    shutil.copy(self.sound_path, dest_sound_path)
                else:
                    try:
                        from pydub import AudioSegment
                        AudioSegment.converter = "ffmpeg"
                        audio = AudioSegment.from_file(self.sound_path)
                        audio.export(dest_sound_path, format="ogg")
                    except Exception:
                        shutil.copy(self.sound_path, dest_sound_path)
                
                sounds_json_path = os.path.join(pack_full_path, "assets", "minecraft", "sounds.json")
                with open(sounds_json_path, "w", encoding="utf-8") as f:
                    json.dump({
                        "item.totem.use": {
                            "sounds": ["item/totem/use"]
                        }
                    }, f, indent=4)

            messagebox.showinfo("Success!", f"Skin Totem Resource Pack is ready in:\n{self.output_dir} 🎉")
        except Exception as e:
            messagebox.showerror("Error", str(e))

if __name__ == "__main__":
    app = TotemWorkshopPro()
    app.mainloop()