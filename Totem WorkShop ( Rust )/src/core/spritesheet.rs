use image::{imageops, RgbaImage};
use serde_json::json;

pub fn build_spritesheet(frames: &[RgbaImage], target_size: u32) -> RgbaImage {
    let total_frames = frames.len() as u32;
    let height = total_frames * target_size;
    let mut spritesheet = RgbaImage::new(target_size, height);

    for (i, frame) in frames.iter().enumerate() {
        let y_offset = (i as u32) * target_size;
        if frame.width() == target_size && frame.height() == target_size {
            imageops::replace(&mut spritesheet, frame, 0, y_offset as i64);
        } else {
            let resized = imageops::resize(frame, target_size, target_size, imageops::FilterType::Lanczos3);
            imageops::replace(&mut spritesheet, &resized, 0, y_offset as i64);
        }
    }

    spritesheet
}

pub fn generate_animation_mcmeta(total_frames: usize, frametime: u32, interpolate: bool) -> serde_json::Value {
    let frames_indices: Vec<usize> = (0..total_frames).collect();
    json!({
        "animation": {
            "frametime": frametime,
            "interpolate": interpolate,
            "frames": frames_indices
        }
    })
}
