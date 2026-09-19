use crate::types::MC_COLORS;
use egui::{Color32, CornerRadius, RichText, Stroke, Ui, Vec2};

/// Linear color interpolation helper for ultra-smooth transitions
pub fn lerp_color(c1: Color32, c2: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let r = (c1.r() as f32 + (c2.r() as f32 - c1.r() as f32) * t).round() as u8;
    let g = (c1.g() as f32 + (c2.g() as f32 - c1.g() as f32) * t).round() as u8;
    let b = (c1.b() as f32 + (c2.b() as f32 - c1.b() as f32) * t).round() as u8;
    let a = (c1.a() as f32 + (c2.a() as f32 - c1.a() as f32) * t).round() as u8;
    Color32::from_rgba_premultiplied(r, g, b, a)
}

/// Paint an authentic Minecraft 3D beveled box / button with optional smooth glow
pub fn paint_mc_beveled_box(
    painter: &egui::Painter,
    rect: egui::Rect,
    fill_color: Color32,
    highlight_color: Color32,
    shadow_color: Color32,
    border_color: Color32,
) {
    // 1. Black outer border (1px)
    painter.rect_filled(rect, CornerRadius::ZERO, border_color);

    // 2. Inner beveled rectangle
    let inner_rect = rect.shrink(1.5);
    painter.rect_filled(inner_rect, CornerRadius::ZERO, fill_color);

    // 3. Top & Left highlight lines (Minecraft 3D lighting)
    painter.line_segment(
        [inner_rect.left_bottom(), inner_rect.left_top()],
        Stroke::new(1.8_f32, highlight_color),
    );
    painter.line_segment(
        [inner_rect.left_top(), inner_rect.right_top()],
        Stroke::new(1.8_f32, highlight_color),
    );

    // 4. Bottom & Right shadow lines (Minecraft 3D shadow)
    painter.line_segment(
        [inner_rect.left_bottom(), inner_rect.right_bottom()],
        Stroke::new(1.8_f32, shadow_color),
    );
    painter.line_segment(
        [inner_rect.right_top(), inner_rect.right_bottom()],
        Stroke::new(1.8_f32, shadow_color),
    );
}

pub fn render_section_header(ui: &mut Ui, icon: &str, title: &str, accent_color: Color32) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(32.0, 32.0), egui::Sense::hover());
        let painter = ui.painter();

        // Minecraft Item Slot Style Header Icon with vibrant accent highlight
        paint_mc_beveled_box(
            painter,
            rect,
            Color32::from_rgb(28, 30, 38),
            lerp_color(Color32::from_rgb(60, 65, 80), accent_color, 0.4),
            Color32::from_rgb(14, 15, 20),
            Color32::from_rgb(8, 8, 12),
        );

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(17.0),
            accent_color,
        );

        ui.add_space(8.0);
        ui.label(
            RichText::new(title)
                .size(17.5)
                .strong()
                .color(Color32::from_rgb(255, 225, 100)), // Vibrant Minecraft Gold
        );
    });
    ui.add_space(5.0);
}

/// Minecraft Experience / Boss Bar Style Progress Indicator with smooth animated shimmer
pub fn render_clean_progress_bar(
    ui: &mut Ui,
    fraction: f32,
    desired_width: f32,
    desired_height: f32,
    fill_color: Color32,
) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(desired_width, desired_height), egui::Sense::hover());
    let painter = ui.painter();
    let time = ui.input(|i| i.time) as f32;

    let clamped_frac = fraction.clamp(0.0, 1.0);

    // Minecraft XP Bar Dark Outer Frame
    paint_mc_beveled_box(
        painter,
        rect,
        Color32::from_rgb(16, 20, 16),
        Color32::from_rgb(34, 44, 34),
        Color32::from_rgb(8, 12, 8),
        Color32::BLACK,
    );

    // Filled XP Bar
    if clamped_frac > 0.005 {
        let inner_rect = rect.shrink(2.0);
        let fill_w = (inner_rect.width() * clamped_frac).max(2.0).min(inner_rect.width());
        let fill_rect = egui::Rect::from_min_size(inner_rect.min, Vec2::new(fill_w, inner_rect.height()));

        let clipped_painter = painter.with_clip_rect(rect);
        clipped_painter.rect_filled(fill_rect, CornerRadius::ZERO, fill_color);

        // Minecraft XP Top Glint highlight
        let glint_rect = egui::Rect::from_min_size(fill_rect.min, Vec2::new(fill_w, fill_rect.height() / 2.0));
        clipped_painter.rect_filled(glint_rect, CornerRadius::ZERO, Color32::from_white_alpha(55));

        // Smooth animated XP shimmer wave
        let wave_pos = (time * 120.0) % (fill_rect.width() + 40.0) - 20.0;
        let wave_rect = egui::Rect::from_min_size(
            egui::pos2(fill_rect.min.x + wave_pos, fill_rect.min.y),
            Vec2::new(20.0, fill_rect.height()),
        );
        clipped_painter.rect_filled(wave_rect, CornerRadius::ZERO, Color32::from_white_alpha(40));
    }

    // Centered percentage / XP level text
    let pct_str = format!("{:.0}%", clamped_frac * 100.0);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        pct_str,
        egui::FontId::proportional(12.0),
        Color32::from_rgb(255, 255, 255),
    );
}

pub fn render_file_picker_card(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    info_text: &str,
    is_selected: bool,
    btn_text: &str,
) -> bool {
    let mut clicked = false;
    let available_w = ui.available_width();
    let card_h = 54.0;

    let (rect, _) = ui.allocate_exact_size(Vec2::new(available_w, card_h), egui::Sense::hover());
    let painter = ui.painter();

    let (bg, highlight, shadow, border) = if is_selected {
        (
            Color32::from_rgb(24, 38, 30),
            Color32::from_rgb(52, 211, 153),
            Color32::from_rgb(14, 24, 18),
            Color32::from_rgb(16, 185, 129),
        )
    } else {
        (
            Color32::from_rgb(30, 32, 40),
            Color32::from_rgb(60, 65, 80),
            Color32::from_rgb(16, 17, 22),
            Color32::from_rgb(10, 11, 15),
        )
    };

    paint_mc_beveled_box(painter, rect, bg, highlight, shadow, border);

    // Left Icon with item slot look
    let icon_box = egui::Rect::from_center_size(egui::pos2(rect.min.x + 26.0, rect.center().y), Vec2::new(34.0, 34.0));
    paint_mc_beveled_box(
        painter,
        icon_box,
        Color32::from_rgb(18, 20, 26),
        Color32::from_rgb(10, 12, 16),
        Color32::from_rgb(48, 52, 68),
        Color32::BLACK,
    );

    painter.text(
        icon_box.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(17.0),
        if is_selected { Color32::from_rgb(85, 255, 85) } else { Color32::from_rgb(85, 255, 255) },
    );

    // Title & Info
    let title_pos = egui::pos2(rect.min.x + 52.0, rect.min.y + 16.0);
    painter.text(
        title_pos,
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(13.5),
        Color32::from_rgb(255, 255, 255),
    );

    let info_pos = egui::pos2(rect.min.x + 52.0, rect.min.y + 37.0);
    let info_color = if is_selected {
        Color32::from_rgb(85, 255, 85)
    } else {
        Color32::from_rgb(160, 170, 190)
    };
    let char_count = info_text.chars().count();
    let display_info = if char_count > 32 {
        let suffix: String = info_text.chars().skip(char_count.saturating_sub(29)).collect();
        format!("...{suffix}")
    } else {
        info_text.to_string()
    };
    painter.text(
        info_pos,
        egui::Align2::LEFT_CENTER,
        display_info,
        egui::FontId::proportional(11.5),
        info_color,
    );

    // Right Action Button (Minecraft 3D Stone Button with smooth hover animation)
    let btn_w = 88.0;
    let btn_h = 32.0;
    let btn_rect = egui::Rect::from_min_size(
        egui::pos2(rect.max.x - btn_w - 10.0, rect.center().y - btn_h / 2.0),
        Vec2::new(btn_w, btn_h),
    );

    let btn_resp = ui.interact(btn_rect, ui.auto_id_with(title), egui::Sense::click());
    let hover_t = ui.ctx().animate_bool_with_time(ui.auto_id_with((title, "hover")), btn_resp.hovered(), 0.16);

    let (btn_bg, btn_high, btn_shad, btn_text_color) = if is_selected {
        if btn_resp.is_pointer_button_down_on() {
            (Color32::from_rgb(20, 44, 32), Color32::from_rgb(14, 28, 20), Color32::from_rgb(52, 211, 153), Color32::WHITE)
        } else {
            let base_bg = Color32::from_rgb(32, 68, 50);
            let hover_bg = Color32::from_rgb(46, 96, 70);
            let bg = lerp_color(base_bg, hover_bg, hover_t);
            (bg, Color32::from_rgb(85, 255, 85), Color32::from_rgb(16, 36, 26), Color32::from_rgb(220, 255, 220))
        }
    } else {
        if btn_resp.is_pointer_button_down_on() {
            (Color32::from_rgb(36, 36, 44), Color32::from_rgb(18, 18, 24), Color32::from_rgb(70, 72, 85), Color32::WHITE)
        } else {
            let base_bg = Color32::from_rgb(54, 56, 68);
            let hover_bg = Color32::from_rgb(78, 82, 100);
            let bg = lerp_color(base_bg, hover_bg, hover_t);
            let high = lerp_color(Color32::from_rgb(95, 100, 120), Color32::from_rgb(255, 215, 0), hover_t);
            (bg, high, Color32::from_rgb(24, 25, 32), Color32::from_rgb(235, 240, 250))
        }
    };

    paint_mc_beveled_box(painter, btn_rect, btn_bg, btn_high, btn_shad, Color32::BLACK);

    let display_btn_text = if is_selected { "Change" } else { btn_text };
    painter.text(
        btn_rect.center(),
        egui::Align2::CENTER_CENTER,
        display_btn_text,
        egui::FontId::proportional(12.0),
        btn_text_color,
    );

    if btn_resp.clicked() {
        clicked = true;
        btn_resp.surrender_focus();
    }

    clicked
}

pub fn render_mc_color_palette(ui: &mut Ui, target_text: &mut String) {
    ui.label(
        RichText::new("🎨 Minecraft Chat Colors:")
            .size(12.5)
            .strong()
            .color(Color32::from_rgb(255, 215, 0)),
    );

    let cols = 6;
    egui::Grid::new("mc_color_grid")
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            for (i, c) in MC_COLORS.iter().enumerate() {
                let bg_color = Color32::from_rgb(c.rgb[0], c.rgb[1], c.rgb[2]);
                let text_color = if c.is_light {
                    Color32::BLACK
                } else {
                    Color32::WHITE
                };

                let btn = egui::Button::new(
                    RichText::new(c.display_name)
                        .color(text_color)
                        .strong()
                        .size(11.0),
                )
                .fill(bg_color)
                .min_size(Vec2::new(45.0, 26.0))
                .corner_radius(CornerRadius::same(2));

                if ui.add(btn).clicked() {
                    target_text.push_str(c.code);
                }

                if (i + 1) % cols == 0 {
                    ui.end_row();
                }
            }
        });
}

pub fn render_signature(ui: &mut Ui, lang: crate::core::i18n::Language) {
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        let sig_text = crate::core::i18n::tr(
            "⚡ Totem Workshop Pro • Crafted by @CanBeShahab",
            "⚡ توتم ورک‌شاپ پرو • ساخته‌شده توسط @CanBeShahab",
            lang,
        );
        ui.label(
            RichText::new(sig_text)
                .italics()
                .size(11.5)
                .color(Color32::from_rgb(150, 160, 180)),
        );
    });
}

pub fn render_hero_card(
    ui: &mut Ui,
    id_source: &str,
    icon: &str,
    title: &str,
    subtitle: &str,
    btn_label: &str,
    tags: &[&str],
    accent_color: Color32,
    size: Vec2,
) -> bool {
    let card_id = ui.make_persistent_id(id_source);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let card_interact = ui.interact(rect, card_id, egui::Sense::click());

    let painter = ui.painter();
    let is_hovered = response.hovered() || card_interact.hovered();
    let is_down = response.is_pointer_button_down_on() || card_interact.is_pointer_button_down_on();

    if is_hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let hover_t = ui.ctx().animate_bool_with_time(card_id, is_hovered, 0.20);

    let base_bg = Color32::from_rgb(30, 32, 40);
    let hover_bg = lerp_color(Color32::from_rgb(42, 46, 58), accent_color, 0.15);
    let bg_fill = if is_down {
        Color32::from_rgb(20, 22, 28)
    } else {
        lerp_color(base_bg, hover_bg, hover_t)
    };

    let base_high = Color32::from_rgb(60, 65, 80);
    let hover_high = accent_color;
    let highlight = if is_down {
        Color32::from_rgb(14, 14, 18)
    } else {
        lerp_color(base_high, hover_high, hover_t)
    };

    let shadow = if is_down {
        Color32::from_rgb(60, 65, 80)
    } else {
        Color32::from_rgb(14, 15, 20)
    };

    let base_border = Color32::from_rgb(8, 9, 12);
    let hover_border = lerp_color(base_border, Color32::from_rgb(255, 215, 0), hover_t);

    // Minecraft Stone Block 3D Beveled Frame
    paint_mc_beveled_box(painter, rect, bg_fill, highlight, shadow, hover_border);

    // Glowing top accent bar with smooth expansion animation
    let bar_width = (rect.width() - 12.0) * (0.3 + 0.7 * hover_t);
    let top_bar = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.min.y + 3.0),
        Vec2::new(bar_width, 3.0),
    );
    let bar_color = lerp_color(Color32::from_rgba_premultiplied(0, 0, 0, 0), accent_color, hover_t);
    painter.rect_filled(top_bar, CornerRadius::ZERO, bar_color);

    // Minecraft Item Slot / Badge for Icon with smooth lift animation
    let icon_lift = -3.0 * hover_t;
    let circle_center = rect.center() - Vec2::new(0.0, 58.0 - icon_lift);
    let icon_box = egui::Rect::from_center_size(circle_center, Vec2::new(54.0, 54.0));
    let icon_bg = lerp_color(Color32::from_rgb(20, 22, 28), Color32::from_rgb(28, 30, 40), hover_t);
    paint_mc_beveled_box(
        painter,
        icon_box,
        icon_bg,
        lerp_color(Color32::from_rgb(12, 12, 16), accent_color, hover_t * 0.5),
        Color32::from_rgb(50, 55, 70),
        Color32::BLACK,
    );

    let icon_color = lerp_color(Color32::WHITE, Color32::from_rgb(255, 235, 120), hover_t);
    painter.text(
        circle_center,
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(26.0),
        icon_color,
    );

    // Title (Minecraft Gold Style with smooth shine)
    let title_pos = rect.center() - Vec2::new(0.0, 16.0);
    let title_color = lerp_color(Color32::from_rgb(240, 245, 255), Color32::from_rgb(255, 225, 80), hover_t);
    painter.text(
        title_pos,
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(20.0),
        title_color,
    );

    // Subtitle
    let sub_pos = rect.center() + Vec2::new(0.0, 10.0);
    let sub_color = lerp_color(Color32::from_rgb(160, 168, 185), Color32::from_rgb(200, 210, 230), hover_t);
    painter.text(
        sub_pos,
        egui::Align2::CENTER_CENTER,
        subtitle,
        egui::FontId::proportional(12.0),
        sub_color,
    );

    // Feature tags pills at middle-bottom (Minecraft item badges with dynamic width)
    let tag_font = egui::FontId::proportional(11.0);
    let tag_spacing = 8.0;
    let tag_h = 22.0;

    let tag_widths: Vec<f32> = tags
        .iter()
        .map(|t| {
            let text_w = painter
                .layout_no_wrap((*t).to_string(), tag_font.clone(), Color32::WHITE)
                .rect
                .width();
            (text_w + 16.0).max(64.0)
        })
        .collect();

    let total_tags_width: f32 =
        tag_widths.iter().sum::<f32>() + (tags.len().saturating_sub(1) as f32) * tag_spacing;
    let mut current_x = rect.center().x - total_tags_width / 2.0;
    let tag_y = rect.center().y + 34.0;

    for (i, tag) in tags.iter().enumerate() {
        let tag_w = tag_widths[i];
        let tag_rect =
            egui::Rect::from_min_size(egui::pos2(current_x, tag_y), Vec2::new(tag_w, tag_h));
        let tag_bg = lerp_color(
            Color32::from_rgb(22, 24, 30),
            Color32::from_rgb(32, 36, 46),
            hover_t,
        );
        paint_mc_beveled_box(
            painter,
            tag_rect,
            tag_bg,
            lerp_color(Color32::from_rgb(45, 50, 62), accent_color, hover_t * 0.4),
            Color32::from_rgb(12, 13, 16),
            Color32::BLACK,
        );
        painter.text(
            tag_rect.center(),
            egui::Align2::CENTER_CENTER,
            *tag,
            tag_font.clone(),
            lerp_color(Color32::from_rgb(190, 195, 210), Color32::WHITE, hover_t),
        );
        current_x += tag_w + tag_spacing;
    }

    // Action Button at Bottom
    let btn_rect = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.max.y - 24.0),
        Vec2::new(rect.width() - 32.0, 30.0),
    );
    let (btn_bg, btn_high, btn_shad, btn_text_c) = if is_down {
        (Color32::from_rgb(24, 26, 32), Color32::from_rgb(14, 15, 18), Color32::from_rgb(50, 55, 70), Color32::WHITE)
    } else {
        let base_b = Color32::from_rgb(44, 48, 62);
        let hover_b = lerp_color(Color32::from_rgb(60, 68, 90), accent_color, 0.4);
        let bg = lerp_color(base_b, hover_b, hover_t);
        let high = lerp_color(Color32::from_rgb(80, 88, 110), Color32::from_rgb(255, 215, 0), hover_t);
        let text_c = lerp_color(Color32::from_rgb(220, 230, 245), Color32::from_rgb(255, 255, 200), hover_t);
        (bg, high, Color32::from_rgb(20, 22, 28), text_c)
    };
    paint_mc_beveled_box(painter, btn_rect, btn_bg, btn_high, btn_shad, Color32::BLACK);
    painter.text(
        btn_rect.center(),
        egui::Align2::CENTER_CENTER,
        btn_label,
        egui::FontId::proportional(12.5),
        btn_text_c,
    );

    let clicked = response.clicked()
        || card_interact.clicked()
        || (is_hovered && ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary)));

    clicked
}

pub fn render_pipeline_step(
    ui: &mut Ui,
    step_num: &str,
    name: &str,
    status: &str,
    is_active: bool,
    is_done: bool,
) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 38.0), egui::Sense::hover());
    let painter = ui.painter();
    let time = ui.input(|i| i.time) as f32;

    let (bg, high, shad, num_color) = if is_done {
        (
            Color32::from_rgb(22, 44, 30),
            Color32::from_rgb(85, 255, 85),
            Color32::from_rgb(10, 22, 14),
            Color32::from_rgb(85, 255, 85),
        )
    } else if is_active {
        let pulse = (time * 4.0).sin() * 0.5 + 0.5;
        let active_bg = lerp_color(Color32::from_rgb(22, 40, 58), Color32::from_rgb(30, 60, 85), pulse);
        (
            active_bg,
            Color32::from_rgb(85, 255, 255),
            Color32::from_rgb(12, 22, 32),
            Color32::from_rgb(85, 255, 255),
        )
    } else {
        (
            Color32::from_rgb(28, 30, 38),
            Color32::from_rgb(52, 56, 68),
            Color32::from_rgb(14, 15, 20),
            Color32::from_rgb(130, 138, 155),
        )
    };

    paint_mc_beveled_box(painter, rect, bg, high, shad, Color32::BLACK);

    // Number Slot
    let badge_rect = egui::Rect::from_min_size(
        egui::pos2(rect.min.x + 8.0, rect.center().y - 11.0),
        Vec2::new(22.0, 22.0),
    );
    paint_mc_beveled_box(
        painter,
        badge_rect,
        Color32::from_rgb(16, 18, 24),
        Color32::from_rgb(8, 10, 14),
        Color32::from_rgb(42, 46, 58),
        Color32::BLACK,
    );
    painter.text(
        badge_rect.center(),
        egui::Align2::CENTER_CENTER,
        step_num,
        egui::FontId::proportional(11.0),
        num_color,
    );

    // Name
    let text_pos = egui::pos2(rect.min.x + 38.0, rect.center().y);
    painter.text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        name,
        egui::FontId::proportional(13.0),
        Color32::from_rgb(240, 242, 250),
    );

    // Status on right
    let status_pos = egui::pos2(rect.max.x - 12.0, rect.center().y);
    painter.text(
        status_pos,
        egui::Align2::RIGHT_CENTER,
        status,
        egui::FontId::proportional(11.5),
        num_color,
    );
}

