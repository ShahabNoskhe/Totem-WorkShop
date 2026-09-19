use crate::config::{load_config, AppConfig};
use crate::types::AppPage;
use crate::ui::animated_totem::{render_animated_totem, AnimatedTotemState};
use crate::ui::dashboard::render_dashboard;
use crate::ui::skin_forge::{render_skin_forge, SkinForgeState};
use egui::{Color32, CornerRadius, Stroke, Vec2, Visuals};
use std::path::PathBuf;

pub struct TotemWorkshopApp {
    pub current_page: AppPage,
    #[allow(dead_code)]
    pub config: AppConfig,
    pub config_path: PathBuf,
    pub animated_totem: AnimatedTotemState,
    pub skin_forge: SkinForgeState,
}

impl TotemWorkshopApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Authentic Minecraft Dark GUI Theme (Bedrock / Deepslate & Stone Bevels)
        let mut visuals = Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(18, 18, 22);
        visuals.window_fill = Color32::from_rgb(26, 26, 32);
        visuals.extreme_bg_color = Color32::from_rgb(12, 12, 16);

        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(28, 28, 34);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.5_f32, Color32::from_rgb(45, 45, 56));
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(2);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(52, 52, 60);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.5_f32, Color32::from_rgb(32, 32, 38));
        visuals.widgets.inactive.corner_radius = CornerRadius::same(2);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(72, 72, 84);
        visuals.widgets.hovered.bg_stroke = Stroke::new(2.0_f32, Color32::from_rgb(255, 215, 0)); // Minecraft Gold
        visuals.widgets.hovered.corner_radius = CornerRadius::same(2);

        visuals.widgets.active.bg_fill = Color32::from_rgb(40, 40, 46);
        visuals.widgets.active.bg_stroke = Stroke::new(2.0_f32, Color32::from_rgb(85, 255, 85)); // Minecraft Emerald
        visuals.widgets.active.corner_radius = CornerRadius::same(2);

        visuals.selection.bg_fill = Color32::from_rgb(56, 142, 60);
        visuals.selection.stroke = Stroke::new(1.0_f32, Color32::from_rgb(129, 199, 132));

        cc.egui_ctx.set_visuals(visuals);

        cc.egui_ctx.style_mut(|s| {
            s.spacing.item_spacing = Vec2::new(8.0, 7.0);
            s.spacing.button_padding = Vec2::new(12.0, 6.0);
            s.spacing.scroll.floating = false;
            s.spacing.scroll.bar_width = 8.0;
            s.spacing.scroll.bar_inner_margin = 4.0;
            s.spacing.scroll.bar_outer_margin = 2.0;
        });

        // Configure Smooth Modern Persian & English Typography (Vazirmatn Medium)
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Vazirmatn".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/fonts/Vazirmatn-Medium.ttf"
            ))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "Vazirmatn".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "Vazirmatn".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        let config_path = PathBuf::from("totem_settings.json");
        let config = load_config(&config_path);
        let out_dir = PathBuf::from(&config.output_dir);

        let animated_totem = AnimatedTotemState::new(out_dir.clone());
        let skin_forge = SkinForgeState::new(out_dir);

        Self {
            current_page: AppPage::Dashboard,
            config,
            config_path,
            animated_totem,
            skin_forge,
        }
    }
}

impl eframe::App for TotemWorkshopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Continuous repaint during task execution or smooth background animations
        if self.animated_totem.progress.is_running
            || self.skin_forge.progress.is_running
            || self.skin_forge.is_fetching
        {
            ctx.request_repaint();
        } else {
            // Keep background pulses, XP glint and hover lerps buttery smooth
            ctx.request_repaint_after(std::time::Duration::from_millis(33));
        }

        let panel_frame = egui::Frame::NONE
            .fill(Color32::from_rgb(13, 15, 22))
            .inner_margin(16.0);

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| match self.current_page {
            AppPage::Dashboard => {
                render_dashboard(ui, &mut self.current_page, &mut self.config, &self.config_path);
            }
            AppPage::AnimatedTotem => {
                render_animated_totem(
                    ui,
                    &mut self.animated_totem,
                    &mut self.current_page,
                    &self.config_path,
                    self.config.language,
                );
            }
            AppPage::SkinForge => {
                render_skin_forge(
                    ui,
                    &mut self.skin_forge,
                    &mut self.current_page,
                    &self.config_path,
                    self.config.language,
                );
            }
        });
    }
}
