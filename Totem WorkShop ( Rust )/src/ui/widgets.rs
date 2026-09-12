use crate::types::MC_COLORS;
use egui::{Color32, CornerRadius, Response, RichText, Stroke, StrokeKind, Ui, Vec2};

pub fn render_section_header(ui: &mut Ui, icon: &str, title: &str, color: Color32) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same(8), Color32::from_black_alpha(60));
        painter.rect_stroke(
            rect,
            CornerRadius::same(8),
            Stroke::new(1.0_f32, color),
            StrokeKind::Inside,
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(15.0),
            color,
        );

        ui.add_space(6.0);
        ui.label(RichText::new(title).size(18.0).strong().color(Color32::WHITE));
    });
    ui.add_space(4.0);
}

pub fn render_clean_progress_bar(
    ui: &mut Ui,
    fraction: f32,
    desired_width: f32,
    desired_height: f32,
    fill_color: Color32,
) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(desired_width, desired_height), egui::Sense::hover());
    let painter = ui.painter();

    let radius = CornerRadius::same((desired_height / 2.0).round() as u8);
    let bg = Color32::from_rgb(16, 18, 26);
    let stroke_color = Color32::from_rgb(38, 42, 58);

    // Track background
    painter.rect_filled(rect, radius, bg);
    painter.rect_stroke(rect, radius, Stroke::new(1.0_f32, stroke_color), StrokeKind::Inside);

    let clamped_frac = fraction.clamp(0.0, 1.0);

    // ONLY draw filled foreground if fraction > 0.005 (0.5%)
    // This completely eliminates the circle/pill overlap artifact at 0%!
    if clamped_frac > 0.005 {
        let fill_w = (rect.width() * clamped_frac).max(desired_height).min(rect.width());
        let fill_rect = egui::Rect::from_min_size(rect.min, Vec2::new(fill_w, desired_height));

        let clipped_painter = painter.with_clip_rect(rect);
        clipped_painter.rect_filled(fill_rect, radius, fill_color);
    }

    // Centered percentage text
    let pct_str = format!("{:.0}%", clamped_frac * 100.0);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        pct_str,
        egui::FontId::proportional(11.0),
        Color32::from_rgb(230, 235, 245),
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
    let card_h = 50.0;

    let (rect, _) = ui.allocate_exact_size(Vec2::new(available_w, card_h), egui::Sense::hover());
    let painter = ui.painter();

    let bg = if is_selected {
        Color32::from_rgb(26, 32, 44)
    } else {
        Color32::from_rgb(20, 22, 28)
    };
    let border_color = if is_selected {
        Color32::from_rgb(56, 189, 248)
    } else {
        Color32::from_rgb(38, 42, 54)
    };

    painter.rect_filled(rect, CornerRadius::same(10), bg);
    painter.rect_stroke(
        rect,
        CornerRadius::same(10),
        Stroke::new(1.0_f32, border_color),
        StrokeKind::Inside,
    );

    // Left Icon
    let icon_pos = egui::pos2(rect.min.x + 22.0, rect.center().y);
    painter.text(
        icon_pos,
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(18.0),
        Color32::WHITE,
    );

    // Title & Info
    let title_pos = egui::pos2(rect.min.x + 44.0, rect.min.y + 14.0);
    painter.text(
        title_pos,
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(13.0),
        Color32::from_rgb(220, 225, 235),
    );

    let info_pos = egui::pos2(rect.min.x + 44.0, rect.min.y + 34.0);
    let info_color = if is_selected {
        Color32::from_rgb(52, 211, 153)
    } else {
        Color32::from_rgb(130, 138, 155)
    };
    let display_info = if info_text.len() > 36 {
        format!("...{}", &info_text[info_text.len().saturating_sub(33)..])
    } else {
        info_text.to_string()
    };
    painter.text(
        info_pos,
        egui::Align2::LEFT_CENTER,
        display_info,
        egui::FontId::proportional(11.0),
        info_color,
    );

    // Right Action Button
    let btn_w = 80.0;
    let btn_h = 30.0;
    let btn_rect = egui::Rect::from_min_size(
        egui::pos2(rect.max.x - btn_w - 10.0, rect.center().y - btn_h / 2.0),
        Vec2::new(btn_w, btn_h),
    );

    let btn_resp = ui.interact(btn_rect, ui.auto_id_with(title), egui::Sense::click());
    
    let (btn_bg, btn_stroke_color, text_color) = if is_selected {
        if btn_resp.is_pointer_button_down_on() {
            (Color32::from_rgb(18, 32, 46), Color32::from_rgb(56, 189, 248), Color32::from_rgb(186, 230, 253))
        } else if btn_resp.hovered() {
            (Color32::from_rgb(28, 48, 68), Color32::from_rgb(125, 211, 252), Color32::WHITE)
        } else {
            (Color32::from_rgb(22, 38, 54), Color32::from_rgb(56, 189, 248), Color32::from_rgb(125, 211, 252))
        }
    } else {
        if btn_resp.is_pointer_button_down_on() {
            (Color32::from_rgb(20, 24, 32), Color32::from_rgb(55, 62, 78), Color32::from_rgb(180, 185, 195))
        } else if btn_resp.hovered() {
            (Color32::from_rgb(36, 42, 56), Color32::from_rgb(70, 80, 102), Color32::WHITE)
        } else {
            (Color32::from_rgb(26, 30, 40), Color32::from_rgb(48, 54, 70), Color32::from_rgb(200, 205, 215))
        }
    };

    painter.rect_filled(btn_rect, CornerRadius::same(6), btn_bg);
    painter.rect_stroke(
        btn_rect,
        CornerRadius::same(6),
        Stroke::new(1.0_f32, btn_stroke_color),
        StrokeKind::Inside,
    );
    
    let display_btn_text = if is_selected { "Change" } else { btn_text };
    painter.text(
        btn_rect.center(),
        egui::Align2::CENTER_CENTER,
        display_btn_text,
        egui::FontId::proportional(12.0),
        text_color,
    );

    if btn_resp.clicked() {
        clicked = true;
        btn_resp.surrender_focus();
    }

    clicked
}

pub fn render_mc_color_palette(ui: &mut Ui, target_text: &mut String) {
    ui.label(RichText::new("🎨 Minecraft Color Palette:").size(12.0).strong().color(Color32::from_rgb(200, 205, 215)));

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
                .min_size(Vec2::new(44.0, 26.0))
                .corner_radius(CornerRadius::same(6));

                if ui.add(btn).clicked() {
                    target_text.push_str(c.code);
                }

                if (i + 1) % cols == 0 {
                    ui.end_row();
                }
            }
        });
}

pub fn render_signature(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        ui.label(
            RichText::new("⚡ Totem Workshop Pro • Crafted by @SirZigo")
                .italics()
                .size(11.0)
                .color(Color32::from_rgb(110, 118, 135)),
        );
    });
}

pub fn render_hero_card(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    subtitle: &str,
    tags: &[&str],
    accent_color: Color32,
    size: Vec2,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    let painter = ui.painter();
    let is_hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    let bg_fill = if is_down {
        Color32::from_rgb(16, 18, 26)
    } else if is_hovered {
        Color32::from_rgb(26, 30, 42)
    } else {
        Color32::from_rgb(20, 23, 32)
    };

    let border_stroke = if is_hovered {
        Stroke::new(1.8_f32, accent_color)
    } else {
        Stroke::new(1.0_f32, Color32::from_rgb(45, 50, 68))
    };

    // Main Card background
    painter.rect_filled(rect, CornerRadius::same(18), bg_fill);
    painter.rect_stroke(rect, CornerRadius::same(18), border_stroke, StrokeKind::Inside);

    // Glowing top accent bar
    let top_bar = egui::Rect::from_min_size(
        rect.min + Vec2::new(20.0, 0.0),
        Vec2::new(rect.width() - 40.0, 3.0),
    );
    if is_hovered {
        painter.rect_filled(top_bar, CornerRadius::same(2), accent_color);
    }

    // Icon Circle
    let circle_center = rect.center() - Vec2::new(0.0, 52.0);
    let circle_radius = 28.0;
    let circle_fill = if is_hovered {
        Color32::from_rgb(accent_color.r() / 4, accent_color.g() / 4, accent_color.b() / 4)
    } else {
        Color32::from_rgb(28, 32, 45)
    };
    painter.circle_filled(circle_center, circle_radius, circle_fill);
    painter.circle_stroke(
        circle_center,
        circle_radius,
        Stroke::new(1.0_f32, if is_hovered { accent_color } else { Color32::from_rgb(60, 68, 90) }),
    );
    painter.text(
        circle_center,
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(26.0),
        Color32::WHITE,
    );

    // Title
    let title_pos = rect.center() - Vec2::new(0.0, 4.0);
    painter.text(
        title_pos,
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(22.0),
        if is_hovered { Color32::WHITE } else { Color32::from_rgb(230, 235, 245) },
    );

    // Subtitle
    let sub_pos = rect.center() + Vec2::new(0.0, 25.0);
    painter.text(
        sub_pos,
        egui::Align2::CENTER_CENTER,
        subtitle,
        egui::FontId::proportional(12.5),
        Color32::from_rgb(148, 163, 184),
    );

    // Feature tags pills at bottom
    let tag_y = rect.max.y - 32.0;
    let mut current_x = rect.center().x - (tags.len() as f32 * 55.0) / 2.0;
    for tag in tags {
        let tag_rect = egui::Rect::from_min_size(egui::pos2(current_x, tag_y), Vec2::new(50.0, 18.0));
        painter.rect_filled(tag_rect, CornerRadius::same(9), Color32::from_rgb(30, 35, 48));
        painter.text(
            tag_rect.center(),
            egui::Align2::CENTER_CENTER,
            *tag,
            egui::FontId::proportional(10.0),
            Color32::from_rgb(148, 163, 184),
        );
        current_x += 56.0;
    }

    response
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

    let (bg, border, num_color) = if is_done {
        (Color32::from_rgb(18, 38, 28), Color32::from_rgb(34, 197, 94), Color32::from_rgb(34, 197, 94))
    } else if is_active {
        (Color32::from_rgb(20, 35, 55), Color32::from_rgb(56, 189, 248), Color32::from_rgb(56, 189, 248))
    } else {
        (Color32::from_rgb(22, 25, 33), Color32::from_rgb(38, 42, 54), Color32::from_rgb(100, 108, 125))
    };

    painter.rect_filled(rect, CornerRadius::same(8), bg);
    painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0_f32, border), StrokeKind::Inside);

    // Number Badge
    let badge_rect = egui::Rect::from_min_size(
        egui::pos2(rect.min.x + 10.0, rect.center().y - 10.0),
        Vec2::new(20.0, 20.0),
    );
    painter.circle_filled(badge_rect.center(), 10.0, Color32::from_black_alpha(80));
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
        Color32::from_rgb(220, 225, 235),
    );

    // Status on right
    let status_pos = egui::pos2(rect.max.x - 12.0, rect.center().y);
    painter.text(
        status_pos,
        egui::Align2::RIGHT_CENTER,
        status,
        egui::FontId::proportional(11.0),
        num_color,
    );
}
