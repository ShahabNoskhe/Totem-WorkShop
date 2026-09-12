use crate::types::AppPage;
use crate::ui::widgets::{render_hero_card, render_signature};
use egui::{Color32, CornerRadius, RichText, Stroke, StrokeKind, Ui, Vec2};

pub fn render_dashboard(ui: &mut Ui, current_page: &mut AppPage) {
    ui.vertical_centered(|ui| {
        ui.add_space(35.0);

        // Version Chip
        let (chip_rect, _) = ui.allocate_exact_size(Vec2::new(260.0, 26.0), egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(chip_rect, CornerRadius::same(13), Color32::from_rgb(20, 24, 35));
        painter.rect_stroke(
            chip_rect,
            CornerRadius::same(13),
            Stroke::new(1.0_f32, Color32::from_rgb(45, 55, 80)),
            StrokeKind::Inside,
        );
        painter.text(
            chip_rect.center(),
            egui::Align2::CENTER_CENTER,
            "⚡ MINECRAFT 1.12 - 1.21+ COMPATIBLE",
            egui::FontId::proportional(10.5),
            Color32::from_rgb(56, 189, 248),
        );

        ui.add_space(14.0);

        // App Title
        ui.label(
            RichText::new("Totem Workshop Pro")
                .size(42.0)
                .strong()
                .color(Color32::WHITE),
        );

        ui.add_space(4.0);
        ui.label(
            RichText::new("High-Performance Native Resource Pack Generator for Minecraft")
                .size(14.0)
                .color(Color32::from_rgb(148, 163, 184)),
        );

        ui.add_space(45.0);

        // Cards Section
        ui.horizontal(|ui| {
            let card_size = Vec2::new(340.0, 250.0);
            let total_width = card_size.x * 2.0 + 35.0;
            let left_pad = ((ui.available_width() - total_width) / 2.0).max(0.0);
            ui.add_space(left_pad);

            // Card 1: Animated Totem
            let anim_resp = render_hero_card(
                ui,
                "🎬",
                "Animated Totem",
                "Convert Videos into Dynamic Animated Totems",
                &["FFmpeg", "ChromaKey", "Spritesheet"],
                Color32::from_rgb(56, 189, 248),
                card_size,
            );
            if anim_resp.clicked() {
                *current_page = AppPage::AnimatedTotem;
            }

            ui.add_space(35.0);

            // Card 2: Skin-Forge
            let skin_resp = render_hero_card(
                ui,
                "🧊",
                "Skin-Forge Studio",
                "Craft 2D & 3D Totems from Minecraft Skins",
                &["Mojang API", "2D Totem", "3D Models"],
                Color32::from_rgb(34, 197, 94),
                card_size,
            );
            if skin_resp.clicked() {
                *current_page = AppPage::SkinForge;
            }
        });

        ui.add_space(60.0);
        render_signature(ui);
    });
}
