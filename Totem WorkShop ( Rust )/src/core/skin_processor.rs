use image::{imageops, Rgba, RgbaImage};

/// Generates the standard 2D Totem of Undying texture (16x16) from a Minecraft skin.
pub fn generate_2d_totem(skin: &RgbaImage) -> RgbaImage {
    let mut totem = RgbaImage::new(16, 16);

    // Crops from skin
    let head = imageops::crop_imm(skin, 8, 8, 8, 8).to_image();
    let body = imageops::crop_imm(skin, 20, 20, 8, 7).to_image();
    let arm_r = imageops::crop_imm(skin, 44, 20, 4, 4).to_image();
    let arm_l = imageops::crop_imm(skin, 36, 52, 4, 4).to_image();

    // Overlays
    let head_ov = imageops::crop_imm(skin, 40, 8, 8, 8).to_image();
    let body_ov = imageops::crop_imm(skin, 20, 36, 8, 7).to_image();
    let arm_r_ov = imageops::crop_imm(skin, 44, 36, 4, 4).to_image();
    let arm_l_ov = imageops::crop_imm(skin, 52, 52, 4, 4).to_image();

    // Base body layers
    imageops::overlay(&mut totem, &arm_r, 0, 9);
    imageops::overlay(&mut totem, &body, 4, 9);
    imageops::overlay(&mut totem, &arm_l, 12, 9);
    imageops::overlay(&mut totem, &head, 4, 1);

    // Overlay layers (blended)
    imageops::overlay(&mut totem, &head_ov, 4, 1);
    imageops::overlay(&mut totem, &arm_r_ov, 0, 9);
    imageops::overlay(&mut totem, &body_ov, 4, 9);
    imageops::overlay(&mut totem, &arm_l_ov, 12, 9);

    // Totem contour mask: clearing pixels to create iconic Minecraft Totem silhouette
    let pixels_to_clear: &[(u32, u32)] = &[
        (0, 10), (15, 10),
        (0, 11), (1, 11), (14, 11), (15, 11),
        (0, 12), (1, 12), (2, 12), (3, 12), (12, 12), (13, 12), (14, 12), (15, 12),
        (4, 13), (11, 13),
        (4, 14), (5, 14), (10, 14), (11, 14),
        (4, 15), (5, 15), (10, 15), (11, 15),
    ];

    for &(x, y) in pixels_to_clear {
        if x < 16 && y < 16 {
            totem.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

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
