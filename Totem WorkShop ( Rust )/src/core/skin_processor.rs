use image::{imageops, RgbaImage};

/// Generates the iconic cute 2D Totem of Undying texture (16x16) from a Minecraft skin (spea.cc style)
pub fn generate_2d_totem(skin: &RgbaImage) -> RgbaImage {
    let mut totem = RgbaImage::new(16, 16);

    // 1. Head (8x8 at x=4, y=1)
    let head = imageops::crop_imm(skin, 8, 8, 8, 8).to_image();
    let head_ov = imageops::crop_imm(skin, 40, 8, 8, 8).to_image();

    // 2. Torso (8x4 at x=4, y=9)
    let body = imageops::crop_imm(skin, 20, 20, 8, 4).to_image();
    let body_ov = imageops::crop_imm(skin, 20, 36, 8, 4).to_image();

    // 3. Right Arm (on left side of totem at x=1, y=7, width=3, height=2)
    // - Outer tip (x=1..3, y=7..9): Hand (2x2)
    // - Inner sleeve (x=3..4, y=7..9): Shoulder (1x2)
    let arm_r_hand = imageops::crop_imm(skin, 44, 28, 2, 2).to_image();
    let arm_r_sleeve = imageops::crop_imm(skin, 44, 20, 1, 2).to_image();
    let arm_r_hand_ov = imageops::crop_imm(skin, 44, 44, 2, 2).to_image();
    let arm_r_sleeve_ov = imageops::crop_imm(skin, 44, 36, 1, 2).to_image();

    // 4. Left Arm (on right side of totem at x=12, y=7, width=3, height=2)
    // - Inner sleeve (x=12..13, y=7..9): Shoulder (1x2)
    // - Outer tip (x=13..15, y=7..9): Hand (2x2)
    let arm_l_sleeve = imageops::crop_imm(skin, 36, 52, 1, 2).to_image();
    let arm_l_hand = imageops::crop_imm(skin, 36, 60, 2, 2).to_image();
    let arm_l_sleeve_ov = imageops::crop_imm(skin, 52, 52, 1, 2).to_image();
    let arm_l_hand_ov = imageops::crop_imm(skin, 52, 60, 2, 2).to_image();

    // 5. Pants / Legs (6x2 at x=5, y=13)
    // - Right leg top (3x2 at x=5, y=13)
    // - Left leg top (3x2 at x=8, y=13)
    let leg_r_pants = imageops::crop_imm(skin, 4, 20, 3, 2).to_image();
    let leg_l_pants = imageops::crop_imm(skin, 20, 52, 3, 2).to_image();
    let leg_r_pants_ov = imageops::crop_imm(skin, 4, 36, 3, 2).to_image();
    let leg_l_pants_ov = imageops::crop_imm(skin, 4, 52, 3, 2).to_image();

    // 6. Feet / Shoes bottom (4x1 at x=6, y=15)
    // - Right foot (2x1 at x=6, y=15)
    // - Left foot (2x1 at x=8, y=15)
    let foot_r = imageops::crop_imm(skin, 4, 31, 2, 1).to_image();
    let foot_l = imageops::crop_imm(skin, 20, 63, 2, 1).to_image();
    let foot_r_ov = imageops::crop_imm(skin, 4, 47, 2, 1).to_image();
    let foot_l_ov = imageops::crop_imm(skin, 4, 63, 2, 1).to_image();

    // Composite Base Layers:
    // Arms (rendered at sides)
    imageops::overlay(&mut totem, &arm_r_hand, 1, 7);
    imageops::overlay(&mut totem, &arm_r_sleeve, 3, 7);
    imageops::overlay(&mut totem, &arm_l_sleeve, 12, 7);
    imageops::overlay(&mut totem, &arm_l_hand, 13, 7);

    // Torso & Legs
    imageops::overlay(&mut totem, &body, 4, 9);
    imageops::overlay(&mut totem, &leg_r_pants, 5, 13);
    imageops::overlay(&mut totem, &leg_l_pants, 8, 13);
    imageops::overlay(&mut totem, &foot_r, 6, 15);
    imageops::overlay(&mut totem, &foot_l, 8, 15);

    // Head
    imageops::overlay(&mut totem, &head, 4, 1);

    // Composite Overlay / 2nd Layer (with alpha blending):
    imageops::overlay(&mut totem, &arm_r_hand_ov, 1, 7);
    imageops::overlay(&mut totem, &arm_r_sleeve_ov, 3, 7);
    imageops::overlay(&mut totem, &arm_l_sleeve_ov, 12, 7);
    imageops::overlay(&mut totem, &arm_l_hand_ov, 13, 7);

    imageops::overlay(&mut totem, &body_ov, 4, 9);
    imageops::overlay(&mut totem, &leg_r_pants_ov, 5, 13);
    imageops::overlay(&mut totem, &leg_l_pants_ov, 8, 13);
    imageops::overlay(&mut totem, &foot_r_ov, 6, 15);
    imageops::overlay(&mut totem, &foot_l_ov, 8, 15);

    imageops::overlay(&mut totem, &head_ov, 4, 1);

    totem
}

/// Generates a full 3D body standing preview (16x32) from the skin
pub fn generate_3d_body_preview(skin: &RgbaImage) -> RgbaImage {
    let mut preview = RgbaImage::new(16, 32);

    let head = imageops::crop_imm(skin, 8, 8, 8, 8).to_image();
    let body = imageops::crop_imm(skin, 20, 20, 8, 12).to_image();
    let arm_r = imageops::crop_imm(skin, 44, 20, 4, 12).to_image();
    let leg_r = imageops::crop_imm(skin, 4, 20, 4, 12).to_image();
    let arm_l = imageops::crop_imm(skin, 36, 52, 4, 12).to_image();
    let leg_l = imageops::crop_imm(skin, 20, 52, 4, 12).to_image();

    // Base parts
    imageops::overlay(&mut preview, &arm_r, 0, 8);
    imageops::overlay(&mut preview, &body, 4, 8);
    imageops::overlay(&mut preview, &arm_l, 12, 8);
    imageops::overlay(&mut preview, &leg_r, 4, 20);
    imageops::overlay(&mut preview, &leg_l, 8, 20);
    imageops::overlay(&mut preview, &head, 4, 0);

    // Overlays
    let head_ov = imageops::crop_imm(skin, 40, 8, 8, 8).to_image();
    let body_ov = imageops::crop_imm(skin, 20, 36, 8, 12).to_image();
    let arm_r_ov = imageops::crop_imm(skin, 44, 36, 4, 12).to_image();
    let arm_l_ov = imageops::crop_imm(skin, 52, 52, 4, 12).to_image();
    let leg_r_ov = imageops::crop_imm(skin, 4, 36, 4, 12).to_image();
    let leg_l_ov = imageops::crop_imm(skin, 4, 52, 4, 12).to_image();

    imageops::overlay(&mut preview, &head_ov, 4, 0);
    imageops::overlay(&mut preview, &arm_r_ov, 0, 8);
    imageops::overlay(&mut preview, &body_ov, 4, 8);
    imageops::overlay(&mut preview, &arm_l_ov, 12, 8);
    imageops::overlay(&mut preview, &leg_r_ov, 4, 20);
    imageops::overlay(&mut preview, &leg_l_ov, 8, 20);

    preview
}

/// Rescales pixel art with Nearest Neighbor to retain crisp retro Minecraft pixels
pub fn upscale_pixel_art(img: &RgbaImage, target_width: u32, target_height: u32) -> RgbaImage {
    imageops::resize(img, target_width, target_height, imageops::FilterType::Nearest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_totem_dimensions() {
        let dummy_skin = RgbaImage::new(64, 64);
        let totem = generate_2d_totem(&dummy_skin);
        assert_eq!(totem.width(), 16);
        assert_eq!(totem.height(), 16);
    }
}
