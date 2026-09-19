use crate::config::{save_config, AppConfig};
use crate::core::i18n::{tr, Language};
use crate::types::AppPage;
use crate::ui::widgets::{lerp_color, paint_mc_beveled_box, render_hero_card, render_signature};
use egui::{Color32, RichText, Ui, Vec2};
use std::path::Path;

pub fn render_dashboard(
    ui: &mut Ui,
    current_page: &mut AppPage,
    config: &mut AppConfig,
    config_path: &Path,
) {
    let time = ui.input(|i| i.time) as f32;
    let lang = config.language;

    // Top Bar: Language Switcher
    ui.horizontal(|ui| {
        let left_space = (ui.available_width() - 215.0).max(0.0);
        ui.add_space(left_space);

        let is_en = lang == Language::English;
        let is_fa = lang == Language::Persian;

        let (en_btn_rect, en_resp) = ui.allocate_exact_size(Vec2::new(100.0, 26.0), egui::Sense::click());
        let en_bg = if is_en { Color32::from_rgb(32, 60, 45) } else { Color32::from_rgb(36, 38, 48) };
        let en_high = if is_en { Color32::from_rgb(85, 255, 85) } else { Color32::from_rgb(60, 65, 80) };
        paint_mc_beveled_box(ui.painter(), en_btn_rect, en_bg, en_high, Color32::from_rgb(14, 15, 20), Color32::BLACK);
        ui.painter().text(
            en_btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            "🇬🇧 English",
            egui::FontId::proportional(11.5),
            if is_en { Color32::from_rgb(255, 255, 200) } else { Color32::from_rgb(170, 175, 190) },
        );
        if en_resp.clicked() && !is_en {
            config.language = Language::English;
            #[cfg(not(target_arch = "wasm32"))]
            let _ = save_config(config_path, config);
            #[cfg(target_arch = "wasm32")]
            let _ = save_config(config_path, config);
        }

        ui.add_space(8.0);

        let (fa_btn_rect, fa_resp) = ui.allocate_exact_size(Vec2::new(100.0, 26.0), egui::Sense::click());
        let fa_bg = if is_fa { Color32::from_rgb(32, 60, 45) } else { Color32::from_rgb(36, 38, 48) };
        let fa_high = if is_fa { Color32::from_rgb(85, 255, 85) } else { Color32::from_rgb(60, 65, 80) };
        paint_mc_beveled_box(ui.painter(), fa_btn_rect, fa_bg, fa_high, Color32::from_rgb(14, 15, 20), Color32::BLACK);
        ui.painter().text(
            fa_btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            crate::core::i18n::shape_text("🇮🇷 فارسی"),
            egui::FontId::proportional(11.5),
            if is_fa { Color32::from_rgb(255, 255, 200) } else { Color32::from_rgb(170, 175, 190) },
        );
        if fa_resp.clicked() && !is_fa {
            config.language = Language::Persian;
            #[cfg(not(target_arch = "wasm32"))]
            let _ = save_config(config_path, config);
            #[cfg(target_arch = "wasm32")]
            let _ = save_config(config_path, config);
        }
    });

    ui.vertical_centered(|ui| {
        ui.add_space(16.0);

        // Minecraft Version Chip (Beveled Stone Frame with smooth pulsing diamond glow)
        let pulse = ((time * 2.5).sin() * 0.5 + 0.5) * 0.35;
        let (chip_rect, _) = ui.allocate_exact_size(Vec2::new(320.0, 28.0), egui::Sense::hover());
        let painter = ui.painter();
        
        let chip_highlight = lerp_color(Color32::from_rgb(55, 60, 75), Color32::from_rgb(85, 255, 255), pulse);
        paint_mc_beveled_box(
            painter,
            chip_rect,
            Color32::from_rgb(24, 26, 34),
            chip_highlight,
            Color32::from_rgb(12, 13, 18),
            Color32::BLACK,
        );

        let chip_text_color = lerp_color(Color32::from_rgb(85, 255, 255), Color32::from_rgb(180, 255, 255), pulse);
        painter.text(
            chip_rect.center(),
            egui::Align2::CENTER_CENTER,
            "⚡ MINECRAFT 1.12 - 1.21+ & PACK V84",
            egui::FontId::proportional(11.5),
            chip_text_color,
        );

        ui.add_space(14.0);

        // App Title (Minecraft Gold Style with warm golden glow)
        ui.label(
            RichText::new("Totem Workshop Pro 🪽")
                .size(42.0)
                .strong()
                .color(Color32::from_rgb(255, 218, 50)), // Vibrant Minecraft Gold
        );

        ui.add_space(4.0);
        ui.label(
            RichText::new(tr(
                "High-Performance Voxel & Neural Resource Pack Studio for Minecraft",
                "استودیوی پیشرفته ساخت ریسورس‌پک وکسل و توتم ماینکرفت",
                lang,
            ))
            .size(14.5)
            .color(Color32::from_rgb(190, 200, 220)),
        );

        ui.add_space(38.0);

        // Cards Section
        ui.horizontal(|ui| {
            let card_size = Vec2::new(345.0, 255.0);
            let total_width = card_size.x * 2.0 + 35.0;
            let left_pad = ((ui.available_width() - total_width) / 2.0).max(0.0);
            ui.add_space(left_pad);

            // Card 1: Animated Totem (Vibrant Diamond Cyan Theme)
            let anim_title = tr("Animated Totem", "توتم متحرک", lang);
            let anim_sub = tr(
                "Convert Videos into Dynamic Animated Totems",
                "تبدیل ویدیو به توتم‌های متحرک با هوش مصنوعی",
                lang,
            );
            let anim_btn = tr("Launch Studio ➔", "ورود به استودیو ◀", lang);
            let tag1 = tr("BiRefNet AI", "هوش مصنوعی", lang);
            let tag2 = tr("Matting", "حذف بک‌گراند", lang);
            let tag3 = tr("Animation", "اسپرایت‌شیت", lang);

            let anim_clicked = render_hero_card(
                ui,
                "card_anim_totem",
                "🎬",
                &anim_title,
                &anim_sub,
                &anim_btn,
                &[&tag1, &tag2, &tag3],
                Color32::from_rgb(14, 165, 233), // Sky Diamond Blue
                card_size,
            );
            if anim_clicked {
                *current_page = AppPage::AnimatedTotem;
            }

            ui.add_space(35.0);

            // Card 2: Skin-Forge (Vibrant Emerald Theme)
            let skin_title = tr("Skin-Forge Studio", "استودیوی اسکین", lang);
            let skin_sub = tr(
                "Craft 2D & 3D Totems from Minecraft Skins",
                "ساخت توتم‌های ۲بعدی و سه‌بعدی از اسکین بازیکن",
                lang,
            );
            let skin_btn = tr("Launch Studio ➔", "ورود به استودیو ◀", lang);
            let stag1 = tr("Mojang API", "سرویس موجانگ", lang);
            let stag2 = tr("2D Totem", "توتم ۲بعدی", lang);
            let stag3 = tr("3D Models", "مدل ۳بعدی", lang);

            let skin_clicked = render_hero_card(
                ui,
                "card_skin_forge",
                "🧊",
                &skin_title,
                &skin_sub,
                &skin_btn,
                &[&stag1, &stag2, &stag3],
                Color32::from_rgb(34, 197, 94), // Emerald Green
                card_size,
            );
            if skin_clicked {
                *current_page = AppPage::SkinForge;
            }
        });

        ui.add_space(50.0);
        render_signature(ui, lang);
    });
}
