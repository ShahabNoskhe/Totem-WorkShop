#[cfg(not(target_arch = "wasm32"))]
use crate::config::{save_config, AppConfig};
#[cfg(not(target_arch = "wasm32"))]
use crate::core::model_generator::generate_totem_3d_models;
#[cfg(not(target_arch = "wasm32"))]
use crate::core::mojang_api::fetch_skin_by_username;
use crate::core::mojang_api::normalize_skin_format;
#[cfg(not(target_arch = "wasm32"))]
use crate::core::pack_builder::prepare_pack_structure;
use crate::core::pack_builder::PackMetaOptions;
use crate::core::skin_processor::{generate_2d_totem, generate_3d_body_preview, upscale_pixel_art};
use crate::types::{AppPage, FileUploadEvent, GenerationProgress, TaskUpdate, TotemMode, MC_VERSIONS};
use crate::ui::widgets::{
    render_clean_progress_bar, render_file_picker_card, render_mc_color_palette,
    render_section_header, render_signature,
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use egui::{Color32, ColorImage, CornerRadius, RichText, Stroke, StrokeKind, TextureHandle, TextureOptions, Ui, Vec2};
use image::RgbaImage;
use std::path::{Path, PathBuf};

pub struct SkinForgeState {
    pub username_entry: String,
    pub skin_file_path: Option<PathBuf>,
    pub raw_skin: Option<RgbaImage>,
    pub totem_2d: Option<RgbaImage>,
    pub preview_3d: Option<RgbaImage>,
    pub texture_2d: Option<TextureHandle>,
    pub texture_3d: Option<TextureHandle>,
    pub totem_mode: TotemMode,
    pub icon_path: Option<PathBuf>,
    pub sound_path: Option<PathBuf>,
    pub sound_label: String,
    pub output_dir: PathBuf,
    pub version_idx: usize,
    pub pack_name: String,
    pub pack_desc: String,
    pub active_entry_is_desc: bool,
    pub is_fetching: bool,
    pub fetch_status: String,
    pub progress: GenerationProgress,
    pub icon_bytes: Option<Vec<u8>>,
    pub sound_bytes: Option<(String, Vec<u8>)>,
    pub file_tx: Sender<FileUploadEvent>,
    pub file_rx: Receiver<FileUploadEvent>,
    #[allow(dead_code)]
    tx: Sender<TaskUpdate>,
    rx: Receiver<TaskUpdate>,
    skin_tx: Sender<Result<RgbaImage, String>>,
    skin_rx: Receiver<Result<RgbaImage, String>>,
}

impl SkinForgeState {
    pub fn new(initial_output_dir: PathBuf) -> Self {
        let (tx, rx) = unbounded();
        let (skin_tx, skin_rx) = unbounded();
        let (file_tx, file_rx) = unbounded();
        Self {
            username_entry: "".to_string(),
            skin_file_path: None,
            raw_skin: None,
            totem_2d: None,
            preview_3d: None,
            texture_2d: None,
            texture_3d: None,
            totem_mode: TotemMode::TwoD,
            icon_path: None,
            sound_path: None,
            sound_label: "Default totem sound".to_string(),
            output_dir: initial_output_dir,
            version_idx: 0,
            pack_name: "Skin Totem Pack".to_string(),
            pack_desc: "Custom Totem by Skin-Forge".to_string(),
            active_entry_is_desc: false,
            is_fetching: false,
            fetch_status: "".to_string(),
            progress: GenerationProgress::default(),
            icon_bytes: None,
            sound_bytes: None,
            file_tx,
            file_rx,
            tx,
            rx,
            skin_tx,
            skin_rx,
        }
    }

    pub fn poll_updates(&mut self, ctx: &egui::Context) {
        // Poll file pickers/uploads
        while let Ok(ev) = self.file_rx.try_recv() {
            match ev {
                FileUploadEvent::Skin(_name, bytes) => {
                    self.load_skin_from_bytes(&bytes, ctx);
                }
                FileUploadEvent::Icon(name, bytes) => {
                    self.icon_path = Some(PathBuf::from(&name));
                    self.icon_bytes = Some(bytes);
                }
                FileUploadEvent::Sound(name, bytes) => {
                    self.sound_label = name.clone();
                    self.sound_path = Some(PathBuf::from(&name));
                    self.sound_bytes = Some((name, bytes));
                }
                _ => {}
            }
        }

        // Poll skin download result
        if let Ok(res) = self.skin_rx.try_recv() {
            self.is_fetching = false;
            match res {
                Ok(skin) => {
                    self.set_skin(skin, ctx);
                    self.fetch_status = "Skin loaded successfully! ✨".to_string();
                }
                Err(err) => {
                    self.fetch_status = format!("❌ {err}");
                }
            }
        }

        // Poll pack generation progress
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                TaskUpdate::Step(s) => {
                    self.progress.step = s;
                }
                TaskUpdate::Progress(pct, text) => {
                    self.progress.fraction = pct;
                    self.progress.progress_text = text;
                }
                TaskUpdate::Success(msg) => {
                    self.progress.is_running = false;
                    self.progress.fraction = 1.0;
                    self.progress.step = "🎉 Ready!".to_string();
                    self.progress.result_message = Some(Ok(msg));
                }
                TaskUpdate::Error(err) => {
                    self.progress.is_running = false;
                    self.progress.step = "❌ Error".to_string();
                    self.progress.result_message = Some(Err(err));
                }
            }
        }
    }

    pub fn set_skin(&mut self, skin: RgbaImage, ctx: &egui::Context) {
        let totem_2d = generate_2d_totem(&skin);
        let preview_3d = generate_3d_body_preview(&skin);

        // Upscale for sharp preview
        let upscaled_2d = upscale_pixel_art(&totem_2d, 160, 160);
        let upscaled_3d = upscale_pixel_art(&preview_3d, 120, 240);

        let ci_2d = ColorImage::from_rgba_unmultiplied(
            [upscaled_2d.width() as usize, upscaled_2d.height() as usize],
            upscaled_2d.as_raw(),
        );
        let ci_3d = ColorImage::from_rgba_unmultiplied(
            [upscaled_3d.width() as usize, upscaled_3d.height() as usize],
            upscaled_3d.as_raw(),
        );

        self.texture_2d = Some(ctx.load_texture("totem_2d_tex", ci_2d, TextureOptions::NEAREST));
        self.texture_3d = Some(ctx.load_texture("totem_3d_tex", ci_3d, TextureOptions::NEAREST));

        self.totem_2d = Some(totem_2d);
        self.preview_3d = Some(preview_3d);
        self.raw_skin = Some(skin);
    }

    pub fn fetch_username_skin(&mut self) {
        let username = self.username_entry.trim().to_string();
        if username.is_empty() {
            self.fetch_status = "Please enter a Minecraft username!".to_string();
            return;
        }

        self.is_fetching = true;
        self.fetch_status = format!("Fetching skin for '{username}'...");

        #[cfg(not(target_arch = "wasm32"))]
        {
            let skin_tx = self.skin_tx.clone();
            std::thread::spawn(move || {
                let res = fetch_skin_by_username(&username);
                let _ = skin_tx.send(res);
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            let skin_tx = self.skin_tx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = crate::core::mojang_api::fetch_skin_by_username_wasm(&username).await;
                let _ = skin_tx.send(res);
            });
        }
    }

    pub fn load_skin_from_bytes(&mut self, bytes: &[u8], ctx: &egui::Context) {
        if let Ok(img) = image::load_from_memory(bytes) {
            let normalized = normalize_skin_format(img.to_rgba8());
            self.set_skin(normalized, ctx);
            self.fetch_status = "Skin image loaded! ✨".to_string();
        } else {
            self.fetch_status = "Failed to decode skin image.".to_string();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_skin_file(&mut self, path: PathBuf, ctx: &egui::Context) {
        if let Ok(img) = image::open(&path) {
            let normalized = normalize_skin_format(img.to_rgba8());
            self.set_skin(normalized, ctx);
            self.skin_file_path = Some(path);
            self.fetch_status = "Local skin loaded! ✨".to_string();
        } else {
            self.fetch_status = "Failed to open skin file.".to_string();
        }
    }

    pub fn generate_pack(&mut self) {
        let raw_skin = match &self.raw_skin {
            Some(s) => s.clone(),
            None => {
                self.progress.result_message =
                    Some(Err("Search username or choose a skin file first! 🔍".to_string()));
                return;
            }
        };

        let totem_2d = match &self.totem_2d {
            Some(t) => t.clone(),
            None => return,
        };

        self.progress.is_running = true;
        self.progress.fraction = 0.2;
        self.progress.step = "Generating skin totem...".to_string();
        self.progress.result_message = None;

        let mode = self.totem_mode;
        let pack_opts = PackMetaOptions {
            output_dir: self.output_dir.clone(),
            pack_name: self.pack_name.clone(),
            description: self.pack_desc.clone(),
            pack_format: MC_VERSIONS[self.version_idx].pack_format,
            icon_path: self.icon_path.clone(),
            sound_path: self.sound_path.clone(),
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            let tx = self.tx.clone();
            std::thread::spawn(move || {
                let _ = tx.send(TaskUpdate::Progress(0.4, "Preparing pack structure...".to_string()));

                let pack_paths = match prepare_pack_structure(&pack_opts) {
                    Ok(p) => p,
                    Err(e) => {
                        let _ = tx.send(TaskUpdate::Error(e));
                        return;
                    }
                };

                let texture_dest = pack_paths.item_textures.join("totem_of_undying.png");

                match mode {
                    TotemMode::TwoD => {
                        let _ = tx.send(TaskUpdate::Progress(0.7, "Writing 2D texture...".to_string()));
                        if let Err(e) = totem_2d.save(&texture_dest) {
                            let _ = tx.send(TaskUpdate::Error(format!("Failed to save texture: {e}")));
                            return;
                        }
                    }
                    TotemMode::ThreeD => {
                        let _ = tx.send(TaskUpdate::Progress(0.6, "Writing 3D texture & models...".to_string()));
                        if let Err(e) = raw_skin.save(&texture_dest) {
                            let _ = tx.send(TaskUpdate::Error(format!("Failed to save skin texture: {e}")));
                            return;
                        }
                        if let Err(e) = generate_totem_3d_models(&pack_paths.root) {
                            let _ = tx.send(TaskUpdate::Error(format!("Failed to create 3D models: {e}")));
                            return;
                        }
                    }
                }

                let _ = tx.send(TaskUpdate::Success(format!(
                    "Skin Totem Resource Pack is ready in:\n{}",
                    pack_paths.root.display()
                )));
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            let texture = match mode {
                TotemMode::TwoD => totem_2d,
                TotemMode::ThreeD => raw_skin,
            };
            match crate::core::pack_builder::build_pack_zip_buffer(
                &pack_opts,
                &texture,
                mode,
                None,
                self.icon_bytes.as_deref(),
                self.sound_bytes.as_ref().map(|(n, b)| (n.as_str(), b.as_slice())),
            ) {
                Ok(zip_bytes) => {
                    let zip_name = format!("{}.zip", crate::core::pack_builder::sanitize_folder_name(&pack_opts.pack_name));
                    let _ = crate::core::pack_builder::download_bytes_in_browser(&zip_name, &zip_bytes);
                    self.progress.is_running = false;
                    self.progress.fraction = 1.0;
                    self.progress.step = "🎉 Downloaded!".to_string();
                    self.progress.result_message = Some(Ok(format!("Resource Pack '{zip_name}' downloaded!")));
                }
                Err(err) => {
                    self.progress.is_running = false;
                    self.progress.step = "❌ Error".to_string();
                    self.progress.result_message = Some(Err(err));
                }
            }
        }
    }
}

pub fn render_skin_forge(
    ui: &mut Ui,
    state: &mut SkinForgeState,
    current_page: &mut AppPage,
    config_path: &Path,
) {
    #[cfg(target_arch = "wasm32")]
    let _ = config_path;

    state.poll_updates(ui.ctx());

    // Support drag and drop skin, icon, and sound files
    ui.ctx().input(|i| {
        if let Some(file) = i.raw.dropped_files.first() {
            if let Some(bytes) = &file.bytes {
                let name = &file.name;
                let name_lower = name.to_lowercase();
                let image_exts = [".png", ".jpg", ".jpeg", ".webp", ".bmp", ".gif", ".ico"];
                let audio_exts = [".ogg", ".mp3", ".wav", ".flac", ".aac", ".m4a", ".wma", ".opus"];

                if image_exts.iter().any(|ext| name_lower.ends_with(ext)) {
                    if name_lower.contains("icon") {
                        state.icon_path = Some(PathBuf::from(name));
                        state.icon_bytes = Some(bytes.to_vec());
                    } else {
                        state.load_skin_from_bytes(bytes, ui.ctx());
                    }
                } else if audio_exts.iter().any(|ext| name_lower.ends_with(ext)) {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let ftx = state.file_tx.clone();
                        let name_c = name.clone();
                        let bytes_vec = bytes.to_vec();
                        wasm_bindgen_futures::spawn_local(async move {
                            let (ogg_name, ogg_bytes) = crate::core::web_bridge::convert_audio_to_ogg(&name_c, &bytes_vec).await;
                            let _ = ftx.send(FileUploadEvent::Sound(ogg_name, ogg_bytes));
                        });
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        state.sound_label = name.clone();
                        state.sound_path = Some(PathBuf::from(name));
                        state.sound_bytes = Some((name.clone(), bytes.to_vec()));
                    }
                }
            }
        }
    });

    // Top Header / Navigation
    ui.horizontal(|ui| {
        let back_btn = egui::Button::new(RichText::new("← Dashboard").strong().color(Color32::from_rgb(148, 163, 184)))
            .fill(Color32::from_rgb(26, 30, 42))
            .corner_radius(CornerRadius::same(8));
        if ui.add(back_btn).clicked() {
            *current_page = AppPage::Dashboard;
        }

        ui.add_space(10.0);
        ui.label(RichText::new("Skin-Forge Studio").size(16.0).strong().color(Color32::WHITE));
        ui.label(RichText::new("•").color(Color32::DARK_GRAY));
        ui.label(RichText::new("Custom Player Skin Totems").size(12.0).color(Color32::from_rgb(34, 197, 94)));
    });

    ui.add_space(12.0);

    ui.columns(2, |cols| {
        // =========================================================
        // LEFT COLUMN: Controls & Configurations
        // =========================================================
        cols[0].vertical(|ui| {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(18, 20, 28))
                .corner_radius(CornerRadius::same(14))
                .inner_margin(18.0)
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(34, 38, 52)))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    egui::ScrollArea::vertical()
                        .id_salt("skin_left_scroll")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            egui::Frame::NONE
                                .inner_margin(egui::Margin { left: 0, right: 14, top: 0, bottom: 8 })
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // 1. Skin Input
                                        render_section_header(ui, "👤", "Minecraft Skin Source", Color32::from_rgb(34, 197, 94));
                                ui.add_space(6.0);

                                ui.label(RichText::new("Mojang Username:").size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut state.username_entry)
                                            .hint_text("Enter player username (e.g. Notch)...")
                                            .desired_width(ui.available_width() - 95.0)
                                            .margin(Vec2::new(10.0, 7.0)),
                                    );

                                    let fetch_btn = egui::Button::new(
                                        RichText::new(if state.is_fetching { "⌛..." } else { "🔍 Fetch" })
                                            .strong()
                                            .color(Color32::WHITE),
                                    )
                                    .fill(Color32::from_rgb(37, 99, 235))
                                    .min_size(Vec2::new(80.0, 32.0))
                                    .corner_radius(CornerRadius::same(6));

                                    if ui.add_enabled(!state.is_fetching, fetch_btn).clicked() {
                                        state.fetch_username_skin();
                                    }
                                });

                                if !state.fetch_status.is_empty() {
                                    ui.add_space(2.0);
                                    ui.label(RichText::new(&state.fetch_status).size(11.0).color(Color32::from_rgb(56, 189, 248)));
                                }

                                ui.add_space(8.0);

                                // Choose local PNG
                                let skin_file_label = state.skin_file_path.as_ref().map_or_else(
                                    || {
                                        if cfg!(target_arch = "wasm32") {
                                            "Drop skin image into window".to_string()
                                        } else {
                                            "No custom skin chosen".to_string()
                                        }
                                    },
                                    |p| p.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                );
                                if render_file_picker_card(
                                    ui, "📂", "Local Skin File", &skin_file_label, state.skin_file_path.is_some() || state.raw_skin.is_some(), "Choose"
                                ) {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Some(file) = rfd::FileDialog::new()
                                        .add_filter("Skin Image", &["png", "jpg", "jpeg", "webp", "bmp", "gif"])
                                        .pick_file()
                                    {
                                        let ctx = ui.ctx().clone();
                                        state.load_skin_file(file, &ctx);
                                    }

                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let ftx = state.file_tx.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Some((name, bytes)) = crate::core::web_bridge::pick_file("image/*").await {
                                                let _ = ftx.send(FileUploadEvent::Skin(name, bytes));
                                            }
                                        });
                                    }
                                }

                                ui.add_space(18.0);

                                // 2. Totem Mode
                                render_section_header(ui, "🧊", "Totem Geometry Mode", Color32::from_rgb(56, 189, 248));
                                ui.add_space(6.0);

                                ui.horizontal(|ui| {
                                    let is_2d = state.totem_mode == TotemMode::TwoD;
                                    let btn_2d = egui::Button::new(
                                        RichText::new("  2D Vanilla Style  ")
                                            .strong()
                                            .color(if is_2d { Color32::BLACK } else { Color32::WHITE }),
                                    )
                                    .fill(if is_2d { Color32::from_rgb(56, 189, 248) } else { Color32::from_rgb(26, 30, 42) })
                                    .min_size(Vec2::new(140.0, 34.0))
                                    .corner_radius(CornerRadius::same(8));

                                    if ui.add(btn_2d).clicked() {
                                        state.totem_mode = TotemMode::TwoD;
                                    }

                                    ui.add_space(8.0);

                                    let is_3d = state.totem_mode == TotemMode::ThreeD;
                                    let btn_3d = egui::Button::new(
                                        RichText::new("  3D Voxel Model  ")
                                            .strong()
                                            .color(if is_3d { Color32::BLACK } else { Color32::WHITE }),
                                    )
                                    .fill(if is_3d { Color32::from_rgb(34, 197, 94) } else { Color32::from_rgb(26, 30, 42) })
                                    .min_size(Vec2::new(140.0, 34.0))
                                    .corner_radius(CornerRadius::same(8));

                                    if ui.add(btn_3d).clicked() {
                                        state.totem_mode = TotemMode::ThreeD;
                                    }
                                });

                                ui.add_space(18.0);

                                // 3. Pack Identity
                                render_section_header(ui, "🏷", "Pack Settings & Audio", Color32::from_rgb(245, 158, 11));
                                ui.add_space(6.0);

                                // Icon Card
                                let has_icon = state.icon_path.is_some() || state.icon_bytes.is_some();
                                let icon_label = state.icon_path.as_ref().map_or_else(
                                    || {
                                        if state.icon_bytes.is_some() {
                                            "Custom Icon Loaded".to_string()
                                        } else {
                                            "Default Icon".to_string()
                                        }
                                    },
                                    |p| p.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                );
                                if render_file_picker_card(ui, "🖼", "Pack Icon", &icon_label, has_icon, "Browse") {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Some(file) = rfd::FileDialog::new().add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "gif", "ico"]).pick_file() {
                                        state.icon_path = Some(file);
                                    }

                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let ftx = state.file_tx.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Some((name, bytes)) = crate::core::web_bridge::pick_file("image/*").await {
                                                let _ = ftx.send(FileUploadEvent::Icon(name, bytes));
                                            }
                                        });
                                    }
                                }

                                ui.add_space(6.0);

                                // Sound Card
                                let has_sound = state.sound_path.is_some() || state.sound_bytes.is_some();
                                if render_file_picker_card(ui, "🎵", "Pop Sound", &state.sound_label, has_sound, "Browse") {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Some(file) = rfd::FileDialog::new().add_filter("Audio Files", &["ogg", "mp3", "wav", "flac", "aac", "m4a", "wma", "opus"]).pick_file() {
                                        state.sound_label = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                                        state.sound_path = Some(file);
                                    }

                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let ftx = state.file_tx.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Some((name, bytes)) = crate::core::web_bridge::pick_file("audio/*").await {
                                                let (ogg_name, ogg_bytes) = crate::core::web_bridge::convert_audio_to_ogg(&name, &bytes).await;
                                                let _ = ftx.send(FileUploadEvent::Sound(ogg_name, ogg_bytes));
                                            }
                                        });
                                    }
                                }

                                ui.add_space(10.0);

                                // Version
                                ui.label(RichText::new("Target Minecraft Version:").size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                egui::ComboBox::from_id_salt("skin_mc_version")
                                    .selected_text(MC_VERSIONS[state.version_idx].display)
                                    .width(ui.available_width() - 10.0)
                                    .show_ui(ui, |ui| {
                                        for (idx, v) in MC_VERSIONS.iter().enumerate() {
                                            ui.selectable_value(&mut state.version_idx, idx, v.display);
                                        }
                                    });

                                ui.add_space(10.0);
                                let target_ref = if state.active_entry_is_desc { &mut state.pack_desc } else { &mut state.pack_name };
                                render_mc_color_palette(ui, target_ref);

                                ui.add_space(10.0);
                                ui.label(RichText::new("Resource Pack Name:").size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                let name_resp = ui.add(
                                    egui::TextEdit::singleline(&mut state.pack_name)
                                        .desired_width(ui.available_width() - 10.0)
                                        .margin(Vec2::new(10.0, 7.0)),
                                );
                                if name_resp.has_focus() {
                                    state.active_entry_is_desc = false;
                                }

                                ui.add_space(6.0);
                                ui.label(RichText::new("Resource Pack Description:").size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                let desc_resp = ui.add(
                                    egui::TextEdit::singleline(&mut state.pack_desc)
                                        .desired_width(ui.available_width() - 10.0)
                                        .margin(Vec2::new(10.0, 7.0)),
                                );
                                if desc_resp.has_focus() {
                                    state.active_entry_is_desc = true;
                                }

                                ui.add_space(12.0);
                                #[cfg(not(target_arch = "wasm32"))]
                                ui.horizontal(|ui| {
                                    let dir_name = state.output_dir.file_name().unwrap_or_default().to_string_lossy();
                                    ui.label(RichText::new(format!("📂 Save To: .../{dir_name}")).size(11.0).color(Color32::GRAY));
                                    if ui.button("Change...").clicked() {
                                        if let Some(folder) = rfd::FileDialog::new().set_directory(&state.output_dir).pick_folder() {
                                            state.output_dir = folder.clone();
                                            let _ = save_config(config_path, &AppConfig { output_dir: folder.to_string_lossy().to_string() });
                                        }
                                    }
                                });

                                #[cfg(target_arch = "wasm32")]
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("📥 Direct Browser Download (.zip pack ready to use)").size(11.5).color(Color32::from_rgb(56, 189, 248)));
                                });

                                ui.add_space(10.0);
                            });
                        });
                    });
                });
        });

        // =========================================================
        // RIGHT COLUMN: Hand Preview & Generator
        // =========================================================
        cols[1].vertical(|ui| {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(18, 20, 28))
                .corner_radius(CornerRadius::same(14))
                .inner_margin(20.0)
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(34, 38, 52)))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Hand Preview & Totem")
                                .size(22.0)
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(
                            RichText::new("Real-time In-Game Totem Rendering")
                                .size(12.0)
                                .color(Color32::from_rgb(148, 163, 184)),
                        );

                        ui.add_space(20.0);

                        // Preview Pedestal Box
                        let preview_rect = ui.allocate_space(Vec2::new(280.0, 280.0)).1;
                        let painter = ui.painter();
                        painter.rect_filled(preview_rect, CornerRadius::same(16), Color32::from_rgb(13, 15, 20));
                        painter.rect_stroke(
                            preview_rect,
                            CornerRadius::same(16),
                            Stroke::new(1.2_f32, Color32::from_rgb(45, 55, 75)),
                            StrokeKind::Inside,
                        );

                        let texture_to_show = match state.totem_mode {
                            TotemMode::TwoD => state.texture_2d.as_ref(),
                            TotemMode::ThreeD => state.texture_3d.as_ref(),
                        };

                        if let Some(tex) = texture_to_show {
                            let img_size = match state.totem_mode {
                                TotemMode::TwoD => Vec2::new(170.0, 170.0),
                                TotemMode::ThreeD => Vec2::new(125.0, 250.0),
                            };
                            let pos = preview_rect.center() - img_size / 2.0;
                            let img_rect = egui::Rect::from_min_size(pos, img_size);
                            painter.image(
                                tex.id(),
                                img_rect,
                                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        } else {
                            painter.text(
                                preview_rect.center() - Vec2::new(0.0, 14.0),
                                egui::Align2::CENTER_CENTER,
                                "🧊",
                                egui::FontId::proportional(34.0),
                                Color32::WHITE,
                            );
                            painter.text(
                                preview_rect.center() + Vec2::new(0.0, 20.0),
                                egui::Align2::CENTER_CENTER,
                                "Awaiting Skin Input\nSearch Username or Choose File",
                                egui::FontId::proportional(12.5),
                                Color32::from_rgb(120, 130, 150),
                            );
                        }

                        ui.add_space(20.0);

                        // Status Badge
                        let mode_name = match state.totem_mode {
                            TotemMode::TwoD => "Mode: 2D Vanilla Texture (16x16)",
                            TotemMode::ThreeD => "Mode: 3D Voxel Custom Model (skin.json)",
                        };
                        ui.label(RichText::new(mode_name).size(12.0).color(Color32::from_rgb(56, 189, 248)));

                        ui.add_space(20.0);

                        // Progress Bar
                        render_clean_progress_bar(
                            ui,
                            state.progress.fraction,
                            ui.available_width() - 30.0,
                            14.0,
                            Color32::from_rgb(34, 197, 94),
                        );

                        ui.add_space(20.0);

                        // Generate Button
                        let is_ready = !state.progress.is_running && state.raw_skin.is_some();
                        let btn_color = if is_ready {
                            Color32::from_rgb(22, 163, 74)
                        } else {
                            Color32::from_rgb(30, 35, 48)
                        };

                        let gen_btn = egui::Button::new(
                            RichText::new(if state.progress.is_running { "⏳ BUILDING PACK..." } else { "🚀 COMPILE SKIN TOTEM PACK" })
                                .size(15.0)
                                .strong()
                                .color(if is_ready { Color32::WHITE } else { Color32::from_rgb(100, 110, 130) }),
                        )
                        .fill(btn_color)
                        .min_size(Vec2::new(ui.available_width() - 30.0, 48.0))
                        .corner_radius(CornerRadius::same(10));

                        if ui.add_enabled(is_ready, gen_btn).clicked() {
                            state.generate_pack();
                        }

                        if let Some(ref res) = state.progress.result_message {
                            ui.add_space(16.0);
                            match res {
                                Ok(msg) => {
                                    let (res_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width() - 20.0, 42.0), egui::Sense::hover());
                                    let p = ui.painter();
                                    p.rect_filled(res_rect, CornerRadius::same(8), Color32::from_rgb(20, 45, 30));
                                    p.rect_stroke(res_rect, CornerRadius::same(8), Stroke::new(1.0_f32, Color32::from_rgb(34, 197, 94)), StrokeKind::Inside);
                                    p.text(res_rect.center(), egui::Align2::CENTER_CENTER, msg, egui::FontId::proportional(12.0), Color32::from_rgb(74, 222, 128));
                                }
                                Err(err) => {
                                    let (res_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width() - 20.0, 42.0), egui::Sense::hover());
                                    let p = ui.painter();
                                    p.rect_filled(res_rect, CornerRadius::same(8), Color32::from_rgb(45, 20, 20));
                                    p.rect_stroke(res_rect, CornerRadius::same(8), Stroke::new(1.0_f32, Color32::from_rgb(239, 68, 68)), StrokeKind::Inside);
                                    p.text(res_rect.center(), egui::Align2::CENTER_CENTER, err, egui::FontId::proportional(12.0), Color32::from_rgb(248, 113, 113));
                                }
                            }
                        }

                        ui.add_space(20.0);
                        render_signature(ui);
                    });
                });
        });
    });
}
