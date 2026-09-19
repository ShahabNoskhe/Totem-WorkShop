#[cfg(not(target_arch = "wasm32"))]
use crate::config::{save_config, AppConfig};
use crate::core::i18n::{tr, Language};
#[cfg(not(target_arch = "wasm32"))]
use crate::core::pack_builder::prepare_pack_structure;
use crate::core::pack_builder::PackMetaOptions;
use crate::core::spritesheet::{build_spritesheet, generate_animation_mcmeta};
use crate::core::video_processor::apply_corner_chroma_key;
#[cfg(not(target_arch = "wasm32"))]
use crate::core::video_processor::{extract_frames_with_ffmpeg, probe_video};
use crate::types::{
    AppPage, CanvasSize, FileUploadEvent, GenerationProgress, TaskUpdate, MC_VERSIONS,
};
use crate::ui::widgets::{
    render_clean_progress_bar, render_file_picker_card, render_mc_color_palette,
    render_pipeline_step, render_section_header, render_signature,
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use egui::{Color32, CornerRadius, RichText, Stroke, Ui, Vec2};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

pub struct AnimatedTotemState {
    pub video_path: Option<PathBuf>,
    pub video_info: String,
    pub video_fps: Option<f32>,
    pub video_total_frames: Option<usize>,
    pub video_duration: Option<f32>,
    pub icon_path: Option<PathBuf>,
    pub sound_path: Option<PathBuf>,
    pub sound_label: String,
    pub output_dir: PathBuf,
    pub version_idx: usize,
    pub pack_name: String,
    pub pack_desc: String,
    pub active_entry_is_desc: bool,
    pub max_frames_str: String,
    pub canvas_size: CanvasSize,
    pub frametime: u32,
    pub frame_skip: usize,
    pub auto_chroma_key: bool,
    pub chroma_tolerance: u8,
    pub interpolate: bool,
    pub progress: GenerationProgress,
    pub current_pipeline_step: usize,
    pub video_bytes: Option<Vec<u8>>,
    pub icon_bytes: Option<Vec<u8>>,
    pub sound_bytes: Option<(String, Vec<u8>)>,
    pub file_tx: Sender<FileUploadEvent>,
    pub file_rx: Receiver<FileUploadEvent>,
    tx: Sender<TaskUpdate>,
    rx: Receiver<TaskUpdate>,
}

impl AnimatedTotemState {
    pub fn new(initial_output_dir: PathBuf) -> Self {
        let (tx, rx) = unbounded();
        let (file_tx, file_rx) = unbounded();
        Self {
            video_path: None,
            video_info: "No video selected".to_string(),
            video_fps: None,
            video_total_frames: None,
            video_duration: None,
            icon_path: None,
            sound_path: None,
            sound_label: "Default totem sound".to_string(),
            output_dir: initial_output_dir,
            version_idx: 0,
            pack_name: "Animated Totem Pack".to_string(),
            pack_desc: "Animated Totem Pack".to_string(),
            active_entry_is_desc: false,
            max_frames_str: "25".to_string(),
            canvas_size: CanvasSize::Normal128,
            frametime: 1,
            frame_skip: 1,
            auto_chroma_key: true,
            chroma_tolerance: 45,
            interpolate: false,
            progress: GenerationProgress::default(),
            current_pipeline_step: 0,
            video_bytes: None,
            icon_bytes: None,
            sound_bytes: None,
            file_tx,
            file_rx,
            tx,
            rx,
        }
    }

    pub fn poll_updates(&mut self) {
        while let Ok(ev) = self.file_rx.try_recv() {
            match ev {
                FileUploadEvent::Video(name, bytes) => {
                    self.video_info = format!("{name} ({:.1} MB)", bytes.len() as f64 / 1_048_576.0);
                    self.video_path = Some(PathBuf::from(&name));

                    if let Some(meta) = crate::core::video_processor::parse_mp4_metadata(&bytes) {
                        self.video_fps = meta.fps;
                        self.video_total_frames = meta.total_frames;
                        self.video_duration = meta.duration_secs;
                    } else {
                        self.video_fps = None;
                        self.video_total_frames = None;
                        self.video_duration = None;

                        #[cfg(target_arch = "wasm32")]
                        {
                            let ftx = self.file_tx.clone();
                            let b_clone = bytes.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                if let Some((fps, tf, dur)) = crate::core::web_bridge::probe_video_web(&b_clone).await {
                                    let _ = ftx.send(FileUploadEvent::VideoMeta(fps, tf, dur));
                                }
                            });
                        }
                    }
                    self.video_bytes = Some(bytes);
                }
                FileUploadEvent::VideoMeta(fps, total_frames, duration) => {
                    self.video_fps = Some(fps);
                    self.video_total_frames = Some(total_frames);
                    self.video_duration = Some(duration);
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

        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                TaskUpdate::Step(s) => {
                    self.progress.step = s;
                    if self.progress.step.contains("Step 1") {
                        self.current_pipeline_step = 1;
                    } else if self.progress.step.contains("Step 2") {
                        self.current_pipeline_step = 2;
                    } else if self.progress.step.contains("Step 3") {
                        self.current_pipeline_step = 3;
                    } else if self.progress.step.contains("Step 4") {
                        self.current_pipeline_step = 4;
                    }
                }
                TaskUpdate::Progress(pct, text) => {
                    self.progress.fraction = pct;
                    self.progress.progress_text = text;
                }
                TaskUpdate::Success(msg) => {
                    self.progress.is_running = false;
                    self.progress.fraction = 1.0;
                    self.current_pipeline_step = 5;
                    self.progress.step = "🎉 All Done! Pack is Ready!".to_string();
                    self.progress.result_message = Some(Ok(msg));
                }
                TaskUpdate::Error(err) => {
                    self.progress.is_running = false;
                    self.progress.step = "❌ Error occurred!".to_string();
                    self.progress.result_message = Some(Err(err));
                }
            }
        }
    }

    pub fn start_generation(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            let video_bytes = match &self.video_bytes {
                Some(b) => b.clone(),
                None => {
                    self.progress.result_message = Some(Err("Please select or drop a video file first!".to_string()));
                    return;
                }
            };

            self.progress.is_running = true;
            self.progress.fraction = 0.05;
            self.current_pipeline_step = 1;
            self.progress.step = "🎬 Decoding video frames in browser...".to_string();
            self.progress.result_message = None;

            let tx = self.tx.clone();
            let max_frames = self.max_frames_str.trim().parse::<u32>().unwrap_or(25).max(1);
            let target_size = self.canvas_size.size();
            let frametime = self.frametime;
            let auto_key = self.auto_chroma_key;
            let tolerance = self.chroma_tolerance;
            let interpolate = self.interpolate;
            let pack_name = self.pack_name.clone();
            let pack_desc = self.pack_desc.clone();
            let pack_format = MC_VERSIONS[self.version_idx].pack_format;
            let icon_bytes = self.icon_bytes.clone();
            let sound_bytes = self.sound_bytes.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let tx_prog = tx.clone();
                let frames_res = crate::core::web_bridge::extract_video_frames(
                    &video_bytes,
                    max_frames,
                    target_size,
                    auto_key,
                    move |pct, text| {
                        let _ = tx_prog.send(TaskUpdate::Progress(0.1 + pct * 0.5, format!("{text} frames")));
                    },
                ).await;

                let mut frames = match frames_res {
                    Ok(f) => f,
                    Err(e) => {
                        let _ = tx.send(TaskUpdate::Error(format!("Failed to decode video: {e}")));
                        return;
                    }
                };

                let _ = tx.send(TaskUpdate::Step("🔥 Processing Background Transparency (Step 2/4)".to_string()));
                let _ = tx.send(TaskUpdate::Progress(0.65, "Chroma keying...".to_string()));

                if auto_key {
                    for frame in frames.iter_mut() {
                        apply_corner_chroma_key(frame, tolerance);
                    }
                }

                let _ = tx.send(TaskUpdate::Step("🛠 Building Spritesheet (Step 3/4)".to_string()));
                let _ = tx.send(TaskUpdate::Progress(0.8, "Assembling sheet...".to_string()));
                let spritesheet = build_spritesheet(&frames, target_size);
                let mcmeta_json = generate_animation_mcmeta(frames.len(), frametime, interpolate);
                let mcmeta_str = serde_json::to_string_pretty(&mcmeta_json).unwrap_or_default();

                let _ = tx.send(TaskUpdate::Step("📦 Building Resource Pack ZIP (Step 4/4)".to_string()));
                let _ = tx.send(TaskUpdate::Progress(0.9, "Compressing ZIP...".to_string()));

                let pack_opts = PackMetaOptions {
                    output_dir: std::path::PathBuf::new(),
                    pack_name: pack_name.clone(),
                    description: pack_desc,
                    pack_format,
                    icon_path: None,
                    sound_path: None,
                };

                match crate::core::pack_builder::build_pack_zip_buffer(
                    &pack_opts,
                    &spritesheet,
                    crate::types::TotemMode::TwoD,
                    Some(&mcmeta_str),
                    icon_bytes.as_deref(),
                    sound_bytes.as_ref().map(|(n, b)| (n.as_str(), b.as_slice())),
                ) {
                    Ok(zip_buf) => {
                        let zip_name = format!("{}.zip", crate::core::pack_builder::sanitize_folder_name(&pack_name));
                        let _ = crate::core::pack_builder::download_bytes_in_browser(&zip_name, &zip_buf);
                        let _ = tx.send(TaskUpdate::Success(format!("Animated Totem Pack '{zip_name}' downloaded!")));
                    }
                    Err(e) => {
                        let _ = tx.send(TaskUpdate::Error(format!("Failed to build ZIP pack: {e}")));
                    }
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let video_path = match &self.video_path {
                Some(p) => p.clone(),
                None => {
                    self.progress.result_message = Some(Err("Please select a video file first!".to_string()));
                    return;
                }
            };

            self.progress.is_running = true;
            self.progress.fraction = 0.0;
            self.current_pipeline_step = 1;
            self.progress.step = "Preparing generation...".to_string();
            self.progress.result_message = None;

            let tx = self.tx.clone();
            let max_frames = self.max_frames_str.trim().parse::<usize>().unwrap_or(25).max(1);
            let frame_skip = self.frame_skip;
            let target_size = self.canvas_size.size();
            let frametime = self.frametime;
            let auto_key = self.auto_chroma_key;
            let tolerance = self.chroma_tolerance;
            let interpolate = self.interpolate;

            let pack_opts = PackMetaOptions {
                output_dir: self.output_dir.clone(),
                pack_name: self.pack_name.clone(),
                description: self.pack_desc.clone(),
                pack_format: MC_VERSIONS[self.version_idx].pack_format,
                icon_path: self.icon_path.clone(),
                sound_path: self.sound_path.clone(),
            };

            thread::spawn(move || {
                let tx_step = |s: &str| {
                    let _ = tx.send(TaskUpdate::Step(s.to_string()));
                };

                tx_step("✂ Extracting Frames with FFmpeg (Step 1/4)");

                let tx_progress = tx.clone();
                let frames_res = extract_frames_with_ffmpeg(
                    &video_path,
                    max_frames,
                    frame_skip,
                    target_size,
                    move |curr, total| {
                        let pct = curr as f32 / total as f32 * 0.4;
                        let _ = tx_progress.send(TaskUpdate::Progress(pct, format!("{curr} / {total}")));
                    },
                );

                let mut frames = match frames_res {
                    Ok(f) => f,
                    Err(e) => {
                        let _ = tx.send(TaskUpdate::Error(e));
                        return;
                    }
                };

                if auto_key {
                    tx_step("🔥 Processing Background Transparency (Step 2/4)");
                    let total = frames.len();
                    for (i, frame) in frames.iter_mut().enumerate() {
                        apply_corner_chroma_key(frame, tolerance);
                        let pct = 0.4 + (i + 1) as f32 / total as f32 * 0.3;
                        let _ = tx.send(TaskUpdate::Progress(pct, format!("{}/{}", i + 1, total)));
                    }
                }

                tx_step("🛠 Building Spritesheet (Step 3/4)");
                let spritesheet = build_spritesheet(&frames, target_size);

                tx_step("📝 Generating Configs & Resource Pack (Step 4/4)");
                let pack_paths = match prepare_pack_structure(&pack_opts) {
                    Ok(p) => p,
                    Err(e) => {
                        let _ = tx.send(TaskUpdate::Error(e));
                        return;
                    }
                };

                let texture_dest = pack_paths.item_textures.join("totem_of_undying.png");
                if let Err(e) = spritesheet.save(&texture_dest) {
                    let _ = tx.send(TaskUpdate::Error(format!("Failed to save spritesheet: {e}")));
                    return;
                }

                let mcmeta_json = generate_animation_mcmeta(frames.len(), frametime, interpolate);
                let mcmeta_str = serde_json::to_string_pretty(&mcmeta_json).unwrap_or_default();
                let mcmeta_dest = pack_paths.item_textures.join("totem_of_undying.png.mcmeta");
                let _ = fs::write(mcmeta_dest, mcmeta_str);

                let _ = tx.send(TaskUpdate::Success(format!(
                    "Resource pack created successfully in:\n{}",
                    pack_paths.root.display()
                )));
            });
        }
    }
}

pub fn render_animated_totem(
    ui: &mut Ui,
    state: &mut AnimatedTotemState,
    current_page: &mut AppPage,
    config_path: &Path,
    lang: Language,
) {
    #[cfg(target_arch = "wasm32")]
    let _ = config_path;

    state.poll_updates();

    // Check for drag-and-drop on desktop and web
    ui.ctx().input(|i| {
        if let Some(file) = i.raw.dropped_files.first() {
            if let Some(bytes) = &file.bytes {
                let name = &file.name;
                let name_lower = name.to_lowercase();
                let video_exts = [".mp4", ".webm", ".mov", ".avi", ".mkv", ".flv", ".wmv", ".m4v", ".3gp", ".ts", ".mts", ".vob", ".ogv"];
                let image_exts = [".png", ".jpg", ".jpeg", ".webp", ".bmp", ".gif", ".ico"];
                let audio_exts = [".ogg", ".mp3", ".wav", ".flac", ".aac", ".m4a", ".wma", ".opus"];

                if video_exts.iter().any(|ext| name_lower.ends_with(ext)) {
                    state.video_info = format!("{name} ({:.1} MB)", bytes.len() as f64 / 1_048_576.0);
                    state.video_path = Some(PathBuf::from(name));
                    state.video_bytes = Some(bytes.to_vec());
                } else if image_exts.iter().any(|ext| name_lower.ends_with(ext)) {
                    state.icon_path = Some(PathBuf::from(name));
                    state.icon_bytes = Some(bytes.to_vec());
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

    // Top Header / Navigation (Minecraft Style)
    ui.horizontal(|ui| {
        let back_btn = egui::Button::new(
            RichText::new(tr("◀ Dashboard", "◀ داشبورد", lang))
                .strong()
                .color(Color32::from_rgb(220, 225, 235)),
        )
        .fill(Color32::from_rgb(52, 52, 60))
        .corner_radius(CornerRadius::same(2));
        if ui.add(back_btn).clicked() {
            *current_page = AppPage::Dashboard;
        }

        ui.add_space(10.0);
        ui.label(
            RichText::new(tr("Animated Totem Studio", "استودیوی توتم متحرک", lang))
                .size(16.0)
                .strong()
                .color(Color32::from_rgb(255, 215, 0)), // Minecraft Gold
        );
        ui.label(RichText::new("•").color(Color32::DARK_GRAY));
        ui.label(
            RichText::new(tr("Video to Animated Totem Pack", "تبدیل ویدیو به توتم متحرک ماینکرفت", lang))
                .size(12.0)
                .color(Color32::from_rgb(85, 255, 255)), // Minecraft Cyan
        );
    });

    ui.add_space(12.0);

    ui.columns(2, |cols| {
        // =========================================================
        // LEFT COLUMN: Controls & Configurations
        // =========================================================
        cols[0].vertical(|ui| {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(24, 24, 30))
                .corner_radius(CornerRadius::same(2))
                .inner_margin(18.0)
                .stroke(Stroke::new(1.5_f32, Color32::from_rgb(45, 45, 56)))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    egui::ScrollArea::vertical()
                        .id_salt("anim_left_scroll")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            egui::Frame::NONE
                                .inner_margin(egui::Margin { left: 0, right: 14, top: 0, bottom: 8 })
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // 1. Media Sources
                                        render_section_header(ui, "🎬", &tr("Media Sources", "فایل‌های چندرسانه‌ای", lang), Color32::from_rgb(56, 189, 248));
                                ui.add_space(6.0);

                                // Video Card
                                let has_video = state.video_path.is_some() || state.video_bytes.is_some();
                                let video_title = tr("Video File", "فایل ویدیوی ورودی", lang);
                                let select_text = tr("Select", "انتخاب فایل", lang);
                                if render_file_picker_card(
                                    ui, "🎥", &video_title, &state.video_info, has_video, &select_text
                                ) {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Some(file) = rfd::FileDialog::new()
                                        .add_filter("Video Files", &["mp4", "webm", "mov", "avi", "mkv", "flv", "wmv", "m4v", "3gp", "ts", "mts", "vob", "ogv"])
                                        .pick_file()
                                    {
                                        if let Ok(bytes) = fs::read(&file) {
                                            if let Some(meta) = crate::core::video_processor::parse_mp4_metadata(&bytes) {
                                                state.video_fps = meta.fps;
                                                state.video_total_frames = meta.total_frames;
                                                state.video_duration = meta.duration_secs;
                                            } else if let Ok(meta) = probe_video(&file) {
                                                state.video_fps = meta.fps;
                                                state.video_total_frames = meta.total_frames;
                                                state.video_duration = meta.duration_secs;
                                            }
                                            state.video_bytes = Some(bytes);
                                        } else if let Ok(meta) = probe_video(&file) {
                                            state.video_fps = meta.fps;
                                            state.video_total_frames = meta.total_frames;
                                            state.video_duration = meta.duration_secs;
                                        }

                                        let size_mb = fs::metadata(&file).map(|m| m.len() as f64 / 1_048_576.0).unwrap_or(0.0);
                                        let name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                                        state.video_info = format!("{name} ({size_mb:.1} MB)");
                                        state.video_path = Some(file);
                                    }

                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let ftx = state.file_tx.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            if let Some((name, bytes)) = crate::core::web_bridge::pick_file("video/*").await {
                                                let _ = ftx.send(FileUploadEvent::Video(name, bytes));
                                            }
                                        });
                                    }
                                }

                                if let (Some(fps), Some(frames)) = (state.video_fps, state.video_total_frames) {
                                    ui.add_space(4.0);
                                    egui::Frame::NONE
                                        .fill(Color32::from_rgb(20, 26, 38))
                                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 65, 95)))
                                        .corner_radius(CornerRadius::same(6))
                                        .inner_margin(egui::Margin::symmetric(12, 7))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(tr("VIDEO SPECS:", "مشخصات ویدیو:", lang)).color(Color32::from_rgb(56, 189, 248)).strong().size(11.5));
                                                ui.label(RichText::new("•").color(Color32::from_rgb(80, 95, 120)).size(11.0));
                                                ui.label(RichText::new(format!("{:.0} FPS", fps)).color(Color32::from_rgb(250, 204, 21)).strong().size(12.0));
                                                ui.label(RichText::new("•").color(Color32::from_rgb(80, 95, 120)).size(11.0));
                                                ui.label(RichText::new(format!("{} {}", frames, tr("Total Frames", "کل فریم‌ها", lang))).color(Color32::from_rgb(52, 211, 153)).strong().size(12.0));
                                                if let Some(dur) = state.video_duration {
                                                    ui.label(RichText::new("•").color(Color32::from_rgb(80, 95, 120)).size(11.0));
                                                    ui.label(RichText::new(format!("{:.1}s {}", dur, tr("Duration", "مدت زمان", lang))).color(Color32::WHITE).size(11.5));
                                                }
                                            });
                                        });
                                    ui.add_space(2.0);
                                }

                                ui.add_space(6.0);

                                // Icon Card
                                let has_icon = state.icon_path.is_some() || state.icon_bytes.is_some();
                                let def_icon_label = tr("Default Icon", "آیکون پیش‌فرض", lang);
                                let icon_label = state.icon_path.as_ref().map_or(def_icon_label, |p| {
                                    p.file_name().unwrap_or_default().to_string_lossy().to_string()
                                });
                                let icon_title = tr("Pack Icon", "آیکون ریسورس‌پک", lang);
                                let browse_text = tr("Browse", "انتخاب", lang);
                                if render_file_picker_card(
                                    ui, "🖼", &icon_title, &icon_label, has_icon, &browse_text
                                ) {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Some(file) = rfd::FileDialog::new()
                                        .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "gif", "ico"])
                                        .pick_file()
                                    {
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
                                let sound_title = tr("Totem Pop Sound", "افکت صوتی استفاده توتم", lang);
                                if render_file_picker_card(
                                    ui, "🎵", &sound_title, &state.sound_label, has_sound, &browse_text
                                ) {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Some(file) = rfd::FileDialog::new()
                                        .add_filter("Audio Files", &["ogg", "mp3", "wav", "flac", "aac", "m4a", "wma", "opus"])
                                        .pick_file()
                                    {
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

                                ui.add_space(18.0);

                                // 2. Animation & Engine Settings
                                render_section_header(ui, "⚙", &tr("Engine Settings", "تنظیمات موتور و انیمیشن", lang), Color32::from_rgb(168, 85, 247));
                                ui.add_space(6.0);

                                // Frames Presets
                                ui.label(RichText::new(tr("Target Frame Count:", "تعداد فریم‌های استخراجی:", lang)).size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                ui.horizontal(|ui| {
                                    for preset in &["5", "15", "35", "75"] {
                                        let is_sel = state.max_frames_str == *preset;
                                        let btn_color = if is_sel {
                                            Color32::from_rgb(56, 189, 248)
                                        } else {
                                            Color32::from_rgb(30, 34, 48)
                                        };
                                        let text_color = if is_sel { Color32::BLACK } else { Color32::WHITE };
                                        let btn = egui::Button::new(RichText::new(*preset).strong().color(text_color))
                                            .fill(btn_color)
                                            .min_size(Vec2::new(42.0, 28.0))
                                            .corner_radius(CornerRadius::same(6));
                                        if ui.add(btn).clicked() {
                                            state.max_frames_str = preset.to_string();
                                        }
                                    }

                                    if let Some(tf) = state.video_total_frames {
                                        let all_str = tf.to_string();
                                        let is_sel = state.max_frames_str == all_str;
                                        let btn_color = if is_sel {
                                            Color32::from_rgb(16, 185, 129)
                                        } else {
                                            Color32::from_rgb(20, 38, 30)
                                        };
                                        let text_color = if is_sel { Color32::BLACK } else { Color32::from_rgb(110, 231, 183) };
                                        let all_btn_label = if lang.is_persian() { format!("کل ({tf})") } else { format!("All ({tf})") };
                                        let btn = egui::Button::new(RichText::new(all_btn_label).strong().color(text_color))
                                            .fill(btn_color)
                                            .min_size(Vec2::new(56.0, 28.0))
                                            .corner_radius(CornerRadius::same(6));
                                        if ui.add(btn).clicked() {
                                            state.max_frames_str = all_str;
                                        }
                                    }

                                    ui.add_space(4.0);
                                    ui.add(
                                        egui::TextEdit::singleline(&mut state.max_frames_str)
                                            .desired_width(55.0)
                                            .margin(Vec2::new(8.0, 5.0)),
                                    );
                                });

                                ui.add_space(14.0);

                                // Canvas Size & Speed side by side
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.add_space(2.0);
                                        ui.label(RichText::new(tr("Canvas Size:", "کیفیت ابعاد تصویر:", lang)).size(11.5).color(Color32::from_rgb(180, 185, 195)));
                                        ui.add_space(2.0);
                                        let opt64 = tr("64px (Zero Lag)", "۶۴ پیکسل (بدون لگ)", lang);
                                        let opt128 = tr("128px (Standard)", "۱۲۸ پیکسل (استاندارد)", lang);
                                        let opt256 = tr("256px (HD)", "۲۵۶ پیکسل (اچ‌دی)", lang);

                                        let selected_label = match state.canvas_size {
                                            CanvasSize::Light64 => &opt64,
                                            CanvasSize::Normal128 => &opt128,
                                            CanvasSize::Heavy256 => &opt256,
                                        };

                                        egui::ComboBox::from_id_salt("anim_canvas_combo")
                                            .selected_text(selected_label)
                                            .width(140.0)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut state.canvas_size, CanvasSize::Light64, opt64);
                                                ui.selectable_value(&mut state.canvas_size, CanvasSize::Normal128, opt128);
                                                ui.selectable_value(&mut state.canvas_size, CanvasSize::Heavy256, opt256);
                                            });
                                    });

                                    ui.add_space(10.0);

                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(tr("Playback Speed:", "سرعت پخش انیمیشن:", lang)).size(11.5).color(Color32::from_rgb(180, 185, 195)));
                                        let spd1 = tr("1 Tick (20 FPS - Fluid)", "۱ تیک (۲۰ فریم - فوق روان)", lang);
                                        let spd2 = tr("2 Ticks (10 FPS)", "۲ تیک (۱۰ فریم)", lang);
                                        let spd3 = tr("3 Ticks (Slow)", "۳ تیک (آهسته)", lang);

                                        let selected_spd = match state.frametime {
                                            1 => &spd1,
                                            2 => &spd2,
                                            _ => &spd3,
                                        };

                                        egui::ComboBox::from_id_salt("anim_speed_combo")
                                            .selected_text(selected_spd)
                                            .width(140.0)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut state.frametime, 1, spd1);
                                                ui.selectable_value(&mut state.frametime, 2, spd2);
                                                ui.selectable_value(&mut state.frametime, 3, spd3);
                                            });
                                    });
                                });

                                ui.add_space(8.0);

                                // Smooth Interpolation (Lag killer)
                                let interp_label = tr("Enable Texture Interpolation (Blur)", "فعال‌سازی تاری بین فریم‌ها (Motion Blur)", lang);
                                ui.checkbox(&mut state.interpolate, interp_label);
                                if state.interpolate {
                                    let warn_text = tr("⚠️ Notice: Texture interpolation causes severe FPS lag in Minecraft! Keep OFF for smooth gameplay.", "⚠️ هشدار: تاری بین فریم‌ها ممکن است در بازی افت فریم ایجاد کند. بهتر است خاموش باشد.", lang);
                                    ui.label(RichText::new(warn_text).size(10.5).color(Color32::from_rgb(251, 146, 60)));
                                }

                                ui.add_space(8.0);

                                // Background Removal
                                let ai_label = tr("Auto-detect & Remove Background (AI & Cutout)", "حذف خودکار پس‌زمینه با هوش مصنوعی (BiRefNet / MediaPipe)", lang);
                                ui.checkbox(&mut state.auto_chroma_key, ai_label);
                                if state.auto_chroma_key {
                                    ui.add_space(2.0);
                                    ui.horizontal(|ui| {
                                        let sens_label = tr("Cutout Sensitivity:", "حساسیت برش هوش مصنوعی:", lang);
                                        ui.label(RichText::new(sens_label).size(11.0).color(Color32::from_rgb(180, 185, 195)));
                                        ui.add(egui::Slider::new(&mut state.chroma_tolerance, 15..=90));
                                    });
                                }

                                ui.add_space(18.0);

                                // 3. Pack Identity
                                render_section_header(ui, "🏷", &tr("Pack Identity", "مشخصات و متادیتای پک", lang), Color32::from_rgb(34, 197, 94));
                                ui.add_space(6.0);

                                // Minecraft Version Dropdown
                                ui.label(RichText::new(tr("Target Minecraft Version:", "نسخه ماینکرفت هدف:", lang)).size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                egui::ComboBox::from_id_salt("anim_mc_version")
                                    .selected_text(MC_VERSIONS[state.version_idx].display)
                                    .width(ui.available_width() - 10.0)
                                    .show_ui(ui, |ui| {
                                        for (idx, v) in MC_VERSIONS.iter().enumerate() {
                                            ui.selectable_value(&mut state.version_idx, idx, v.display);
                                        }
                                    });

                                ui.add_space(10.0);

                                // Color Palette
                                let target_ref = if state.active_entry_is_desc {
                                    &mut state.pack_desc
                                } else {
                                    &mut state.pack_name
                                };
                                render_mc_color_palette(ui, target_ref);

                                ui.add_space(10.0);

                                // Name & Description
                                ui.label(RichText::new(tr("Resource Pack Name:", "نام ریسورس‌پک:", lang)).size(12.0).color(Color32::from_rgb(200, 205, 215)));
                                let name_resp = ui.add(
                                    egui::TextEdit::singleline(&mut state.pack_name)
                                        .desired_width(ui.available_width() - 10.0)
                                        .margin(Vec2::new(10.0, 7.0)),
                                );
                                if name_resp.has_focus() {
                                    state.active_entry_is_desc = false;
                                }

                                ui.add_space(6.0);
                                ui.label(RichText::new(tr("Resource Pack Description:", "توضیحات ریسورس‌پک:", lang)).size(12.0).color(Color32::from_rgb(200, 205, 215)));
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
                                    let save_prefix = tr("📂 Save To:", "📂 مسیر ذخیره:", lang);
                                    let change_btn_text = tr("Change...", "تغییر مسیر...", lang);
                                    ui.label(RichText::new(format!("{save_prefix} .../{dir_name}")).size(11.0).color(Color32::GRAY));
                                    if ui.button(change_btn_text).clicked() {
                                        if let Some(folder) = rfd::FileDialog::new().set_directory(&state.output_dir).pick_folder() {
                                            state.output_dir = folder.clone();
                                            let _ = save_config(config_path, &AppConfig { output_dir: folder.to_string_lossy().to_string(), language: lang });
                                        }
                                    }
                                });
                                #[cfg(target_arch = "wasm32")]
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(tr("📥 Direct Browser Download (.zip pack ready to use)", "📥 دانلود مستقیم در مرورگر (فایل زیپ آماده استفاده)", lang)).size(11.5).color(Color32::from_rgb(56, 189, 248)));
                                });

                                ui.add_space(10.0);
                            });
                        });
                    });
                });
        });

        // =========================================================
        // RIGHT COLUMN: Studio Command Center & Monitor
        // =========================================================
        cols[1].vertical(|ui| {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(24, 24, 30))
                .corner_radius(CornerRadius::same(2))
                .inner_margin(20.0)
                .stroke(Stroke::new(1.5_f32, Color32::from_rgb(45, 45, 56)))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(tr("Studio Command Center", "مرکز فرماندهی و پردازش", lang))
                                .size(22.0)
                                .strong()
                                .color(Color32::from_rgb(255, 215, 0)), // Minecraft Gold
                        );
                        ui.label(
                            RichText::new(tr("Automated Minecraft Resource Pack Compiler", "موتور خودکار ساخت و کامپایل ریسورس‌پک ماینکرفت", lang))
                                .size(12.0)
                                .color(Color32::from_rgb(180, 185, 200)),
                        );

                        ui.add_space(20.0);

                        // Studio Overview Card (Minecraft Stone Item Frame)
                        let (info_card_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 80.0), egui::Sense::hover());
                        let painter = ui.painter();
                        crate::ui::widgets::paint_mc_beveled_box(
                            painter,
                            info_card_rect,
                            Color32::from_rgb(32, 32, 38),
                            Color32::from_rgb(60, 60, 72),
                            Color32::from_rgb(16, 16, 20),
                            Color32::BLACK,
                        );

                        let info_y = info_card_rect.min.y + 16.0;
                        painter.text(
                            egui::pos2(info_card_rect.min.x + 20.0, info_y),
                            egui::Align2::LEFT_TOP,
                            tr("EXPORT CONFIGURATION", "مشخصات ریسورس‌پک خروجی", lang),
                            egui::FontId::proportional(11.0),
                            Color32::from_rgb(85, 255, 255),
                        );

                        let frames_val = state.max_frames_str.trim();
                        let canvas_val = state.canvas_size.size();
                        let src_str = match (state.video_fps, state.video_total_frames) {
                            (Some(fps), Some(tf)) => format!("Source: {:.0} FPS ({} Frames)  •  ", fps, tf),
                            _ => String::new(),
                        };
                        let details = format!("{src_str}Format: {canvas_val}x{canvas_val} px  •  Target Frames: {frames_val}  •  Format: {}", MC_VERSIONS[state.version_idx].pack_format);
                        painter.text(
                            egui::pos2(info_card_rect.min.x + 20.0, info_y + 24.0),
                            egui::Align2::LEFT_TOP,
                            details,
                            egui::FontId::proportional(13.0),
                            Color32::from_rgb(240, 240, 245),
                        );

                        ui.add_space(25.0);

                        // Pipeline Stages
                        let step = state.current_pipeline_step;
                        let s_done = tr("Done ✓", "انجام شد ✓", lang);
                        let s_idle = tr("Idle", "آماده", lang);

                        let st1 = if step > 1 { &s_done } else if step == 1 { "Extracting..." } else { &s_idle };
                        let st2 = if step > 2 { &s_done } else if step == 2 { "Processing..." } else { &s_idle };
                        let st3 = if step > 3 { &s_done } else if step == 3 { "Stitching..." } else { &s_idle };
                        let st4 = if step >= 5 { &s_done } else if step == 4 { "Writing..." } else { &s_idle };

                        render_pipeline_step(ui, "1", &tr("Frame Extraction", "استخراج و تحلیل فریم‌ها", lang), st1, step == 1, step > 1);
                        ui.add_space(6.0);
                        render_pipeline_step(ui, "2", &tr("AI Background Removal", "حذف پس‌زمینه با هوش مصنوعی", lang), st2, step == 2, step > 2);
                        ui.add_space(6.0);
                        render_pipeline_step(ui, "3", &tr("Vertical Spritesheet", "تولید اسپرایت‌شیت عمودی", lang), st3, step == 3, step > 3);
                        ui.add_space(6.0);
                        render_pipeline_step(ui, "4", &tr("Pack & Meta Generation", "بسته‌بندی فایل‌های ریسورس‌پک", lang), st4, step == 4, step >= 5);

                        ui.add_space(25.0);

                        // Live Progress Bar (Minecraft XP Green)
                        render_clean_progress_bar(
                            ui,
                            state.progress.fraction,
                            ui.available_width() - 20.0,
                            14.0,
                            Color32::from_rgb(85, 255, 85),
                        );

                        ui.add_space(6.0);
                        let prog_text = if state.progress.progress_text.is_empty() {
                            tr("Ready to compile", "آماده برای ساخت ریسورس‌پک", lang)
                        } else {
                            format!("Progress: {}", state.progress.progress_text)
                        };
                        ui.label(RichText::new(prog_text).size(12.0).color(Color32::from_rgb(170, 175, 190)));

                        ui.add_space(25.0);

                        // Main Action Button (Minecraft Emerald Button)
                        let is_ready = !state.progress.is_running && state.video_path.is_some();
                        let btn_color = if is_ready {
                            Color32::from_rgb(46, 125, 50)
                        } else {
                            Color32::from_rgb(42, 42, 48)
                        };

                        let btn_label_text = if state.progress.is_running {
                            tr("⏳ COMPILING RESOURCE PACK...", "⏳ در حال ساخت ریسورس‌پک...", lang)
                        } else {
                            tr("🚀 COMPILE ANIMATED TOTEM", "🚀 ساخت و استخراج ریسورس‌پک توتم", lang)
                        };

                        let gen_btn = egui::Button::new(
                            RichText::new(btn_label_text)
                                .size(15.0)
                                .strong()
                                .color(if is_ready { Color32::WHITE } else { Color32::from_rgb(120, 125, 140) }),
                        )
                        .fill(btn_color)
                        .min_size(Vec2::new(ui.available_width() - 30.0, 48.0))
                        .corner_radius(CornerRadius::same(2));

                        if ui.add_enabled(is_ready, gen_btn).clicked() {
                            state.start_generation();
                        }

                        if let Some(ref res) = state.progress.result_message {
                            ui.add_space(16.0);
                            match res {
                                Ok(msg) => {
                                    let (res_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width() - 20.0, 44.0), egui::Sense::hover());
                                    let p = ui.painter();
                                    crate::ui::widgets::paint_mc_beveled_box(
                                        p,
                                        res_rect,
                                        Color32::from_rgb(24, 48, 30),
                                        Color32::from_rgb(85, 255, 85),
                                        Color32::from_rgb(12, 24, 16),
                                        Color32::BLACK,
                                    );
                                    p.text(res_rect.center(), egui::Align2::CENTER_CENTER, msg, egui::FontId::proportional(12.0), Color32::from_rgb(85, 255, 85));
                                }
                                Err(err) => {
                                    let (res_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width() - 20.0, 44.0), egui::Sense::hover());
                                    let p = ui.painter();
                                    crate::ui::widgets::paint_mc_beveled_box(
                                        p,
                                        res_rect,
                                        Color32::from_rgb(48, 24, 24),
                                        Color32::from_rgb(255, 85, 85),
                                        Color32::from_rgb(24, 12, 12),
                                        Color32::BLACK,
                                    );
                                    p.text(res_rect.center(), egui::Align2::CENTER_CENTER, err, egui::FontId::proportional(12.0), Color32::from_rgb(255, 85, 85));
                                }
                            }
                        }

                        ui.add_space(20.0);
                        render_signature(ui, lang);
                    });
                });
        });
    });
}
