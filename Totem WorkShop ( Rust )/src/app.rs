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
        // Deep obsidian modern dark theme
        let mut visuals = Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(13, 15, 22);
        visuals.window_fill = Color32::from_rgb(18, 20, 30);
        visuals.extreme_bg_color = Color32::from_rgb(9, 10, 15);

        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(18, 20, 28);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(30, 34, 46));
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(25, 28, 40);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(42, 47, 65));
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(36, 42, 60);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(56, 189, 248));
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);

        visuals.widgets.active.bg_fill = Color32::from_rgb(45, 53, 76);
        visuals.widgets.active.bg_stroke = Stroke::new(1.2_f32, Color32::from_rgb(56, 189, 248));
        visuals.widgets.active.corner_radius = CornerRadius::same(8);

        cc.egui_ctx.set_visuals(visuals);

        cc.egui_ctx.style_mut(|s| {
            s.spacing.item_spacing = Vec2::new(8.0, 7.0);
            s.spacing.button_padding = Vec2::new(12.0, 6.0);
            s.spacing.scroll.floating = false;
            s.spacing.scroll.bar_width = 8.0;
            s.spacing.scroll.bar_inner_margin = 4.0;
            s.spacing.scroll.bar_outer_margin = 2.0;
        });

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
        // Continuous repaint during task execution
        if self.animated_totem.progress.is_running
            || self.skin_forge.progress.is_running
            || self.skin_forge.is_fetching
        {
            ctx.request_repaint();
        }

        let panel_frame = egui::Frame::NONE
            .fill(Color32::from_rgb(13, 15, 22))
            .inner_margin(16.0);

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| match self.current_page {
            AppPage::Dashboard => {
                render_dashboard(ui, &mut self.current_page);
            }
            AppPage::AnimatedTotem => {
                render_animated_totem(
                    ui,
                    &mut self.animated_totem,
                    &mut self.current_page,
                    &self.config_path,
                );
            }
            AppPage::SkinForge => {
                render_skin_forge(
                    ui,
                    &mut self.skin_forge,
                    &mut self.current_page,
                    &self.config_path,
                );
            }
        });
    }
}
