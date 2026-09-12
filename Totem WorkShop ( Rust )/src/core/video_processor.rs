use image::{Rgba, RgbaImage};
#[cfg(not(target_arch = "wasm32"))]
use image::imageops;
use std::collections::VecDeque;
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

#[derive(Debug, Clone)]
pub struct VideoMeta {
    pub filename: String,
    pub fps: Option<f32>,
    pub duration_secs: Option<f32>,
    pub total_frames: Option<usize>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn probe_video(video_path: &Path) -> Result<VideoMeta, String> {
    let filename = video_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "video".to_string());

    // Call ffmpeg -i to probe information
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .output();

    let mut fps = None;
    let mut duration_secs = None;

    if let Ok(out) = output {
        let stderr = String::from_utf8_lossy(&out.stderr);

        // Look for Duration: 00:00:05.12
        if let Some(dur_idx) = stderr.find("Duration: ") {
            let rest = &stderr[dur_idx + 10..];
            if let Some(end_idx) = rest.find(',') {
                let dur_str = &rest[..end_idx].trim();
                let parts: Vec<&str> = dur_str.split(':').collect();
                if parts.len() == 3 {
                    if let (Ok(h), Ok(m), Ok(s)) = (
                        parts[0].parse::<f32>(),
                        parts[1].parse::<f32>(),
                        parts[2].parse::<f32>(),
                    ) {
                        duration_secs = Some(h * 3600.0 + m * 60.0 + s);
                    }
                }
            }
        }

        // Look for fps e.g. "30 fps" or "29.97 fps"
        for part in stderr.split(',') {
            if part.contains("fps") {
                let trimmed = part.trim();
                let words: Vec<&str> = trimmed.split_whitespace().collect();
                if words.len() >= 2 && words[1] == "fps" {
                    if let Ok(val) = words[0].parse::<f32>() {
                        fps = Some(val);
                        break;
                    }
                }
            }
        }
    }

    let total_frames = match (fps, duration_secs) {
        (Some(f), Some(d)) => Some((f * d).round() as usize),
        _ => None,
    };

    Ok(VideoMeta {
        filename,
        fps,
        duration_secs,
        total_frames,
    })
}

#[cfg(target_arch = "wasm32")]
pub fn probe_video(_video_path: &Path) -> Result<VideoMeta, String> {
    Ok(VideoMeta {
        filename: "video".to_string(),
        fps: None,
        duration_secs: None,
        total_frames: None,
    })
}

/// Pure Rust fast MP4 box parser to extract sample count (frames) and duration/FPS
pub fn parse_mp4_metadata(data: &[u8]) -> Option<VideoMeta> {
    if data.len() < 16 {
        return None;
    }

    fn find_box(data: &[u8], target: &[u8; 4], mut start: usize, end: usize) -> Option<(usize, usize)> {
        while start + 8 <= end {
            let size = u32::from_be_bytes([data[start], data[start + 1], data[start + 2], data[start + 3]]) as usize;
            let tag = &data[start + 4..start + 8];
            let (hdr, actual_size) = if size == 1 {
                if start + 16 > end {
                    return None;
                }
                let s64 = u64::from_be_bytes(data[start + 8..start + 16].try_into().ok()?) as usize;
                (16, s64)
            } else if size == 0 {
                (8, end - start)
            } else {
                (8, size)
            };

            if actual_size < hdr || start + actual_size > end {
                return None;
            }

            if tag == target {
                return Some((start + hdr, start + actual_size));
            }
            start += actual_size;
        }
        None
    }

    let (moov_start, moov_end) = find_box(data, b"moov", 0, data.len())?;
    let mut pos = moov_start;
    while pos < moov_end {
        let (trak_start, trak_end) = find_box(data, b"trak", pos, moov_end)?;
        if let Some((mdia_start, mdia_end)) = find_box(data, b"mdia", trak_start, trak_end) {
            let mut timescale = 1000u32;
            let mut duration_ticks = 0u64;

            if let Some((mdhd_start, mdhd_end)) = find_box(data, b"mdhd", mdia_start, mdia_end) {
                if mdhd_start < mdhd_end && mdhd_start < data.len() {
                    let version = data[mdhd_start];
                    if version == 0 && mdhd_start + 20 <= mdhd_end {
                        timescale = u32::from_be_bytes(data[mdhd_start + 12..mdhd_start + 16].try_into().ok()?);
                        duration_ticks = u32::from_be_bytes(data[mdhd_start + 16..mdhd_start + 20].try_into().ok()?) as u64;
                    } else if version == 1 && mdhd_start + 28 <= mdhd_end {
                        timescale = u32::from_be_bytes(data[mdhd_start + 20..mdhd_start + 24].try_into().ok()?);
                        duration_ticks = u64::from_be_bytes(data[mdhd_start + 24..mdhd_start + 32].try_into().ok()?);
                    }
                }
            }

            if let Some((minf_start, minf_end)) = find_box(data, b"minf", mdia_start, mdia_end) {
                if let Some((stbl_start, stbl_end)) = find_box(data, b"stbl", minf_start, minf_end) {
                    if let Some((stsz_start, stsz_end)) = find_box(data, b"stsz", stbl_start, stbl_end) {
                        if stsz_start + 12 <= stsz_end {
                            let sample_count = u32::from_be_bytes(data[stsz_start + 8..stsz_start + 12].try_into().ok()?) as usize;
                            if sample_count > 0 && timescale > 0 {
                                let dur_secs = duration_ticks as f32 / timescale as f32;
                                let fps = if dur_secs > 0.01 {
                                    sample_count as f32 / dur_secs
                                } else {
                                    30.0
                                };
                                return Some(VideoMeta {
                                    filename: "video.mp4".to_string(),
                                    fps: Some(fps),
                                    duration_secs: Some(dur_secs),
                                    total_frames: Some(sample_count),
                                });
                            }
                        }
                    }
                }
            }
        }
        pos = trak_end;
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub fn extract_frames_with_ffmpeg(
    video_path: &Path,
    max_frames: usize,
    frame_skip: usize,
    target_size: u32,
    progress_callback: impl Fn(usize, usize),
) -> Result<Vec<RgbaImage>, String> {
    let temp_dir = tempfile::tempdir()
        .map_err(|e| format!("Failed to create temporary directory: {e}"))?;
    let out_pattern = temp_dir.path().join("frame_%05d.png");

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").arg(video_path);

    if frame_skip > 1 {
        cmd.arg("-vf").arg(format!("select='not(mod(n\\,{frame_skip}))'"));
        cmd.arg("-vsync").arg("vfr");
    }

    cmd.arg("-vframes").arg(max_frames.to_string());
    cmd.arg("-y").arg(out_pattern);

    let status = cmd
        .status()
        .map_err(|e| format!("Could not run ffmpeg. Is FFmpeg installed and in PATH? Error: {e}"))?;

    if !status.success() {
        return Err("FFmpeg exited with an error while extracting frames.".to_string());
    }

    let mut entries: Vec<PathBuf> = fs::read_dir(temp_dir.path())
        .map_err(|e| format!("Failed to read temp directory: {e}"))?
        .filter_map(|res| res.ok().map(|e| e.path()))
        .filter(|p| p.extension().map_or(false, |ext| ext == "png"))
        .collect();

    entries.sort();

    let total = entries.len().min(max_frames);
    let mut frames = Vec::with_capacity(total);

    for (i, frame_path) in entries.into_iter().take(max_frames).enumerate() {
        let loaded = image::open(&frame_path)
            .map_err(|e| format!("Failed to open extracted frame: {e}"))?
            .to_rgba8();

        let (w, h) = (loaded.width(), loaded.height());
        let min_dim = w.min(h);
        let crop_x = (w - min_dim) / 2;
        let crop_y = (h - min_dim) / 2;
        let cropped = imageops::crop_imm(&loaded, crop_x, crop_y, min_dim, min_dim).to_image();
        let resized = imageops::resize(&cropped, target_size, target_size, imageops::FilterType::Lanczos3);

        frames.push(resized);
        progress_callback(i + 1, total);
    }

    if frames.is_empty() {
        return Err("No frames were extracted from the video.".to_string());
    }

    Ok(frames)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn process_audio_file(source: &Path, dest_ogg: &Path) -> Result<(), String> {
    if let Some(parent) = dest_ogg.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create audio directory: {e}"))?;
    }

    // Check if the file is already a valid Ogg Vorbis file by magic bytes "OggS"
    if let Ok(data) = fs::read(source) {
        if crate::core::audio_converter::is_ogg(&data) {
            fs::copy(source, dest_ogg)
                .map_err(|e| format!("Failed to copy ogg sound: {e}"))?;
            return Ok(());
        }
    }

    // Convert via ffmpeg to Ogg Vorbis (-vn disables video stream, -c:a libvorbis encodes Vorbis)
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(source)
        .arg("-vn")
        .arg("-c:a")
        .arg("libvorbis")
        .arg("-q:a")
        .arg("4")
        .arg(dest_ogg)
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => {
            fs::copy(source, dest_ogg)
                .map_err(|e| format!("Failed to convert audio to OGG: {e}"))?;
            Ok(())
        }
    }
}

/// Universal background remover supporting Green Screen, Blue Screen,
/// Black/White/Gray backgrounds, and any uniform/edge-connected backgrounds.
pub fn apply_corner_chroma_key(img: &mut RgbaImage, tolerance: u8) {
    universal_remove_background(img, tolerance);
}

pub fn universal_remove_background(img: &mut RgbaImage, tolerance: u8) {
    let (width, height) = (img.width(), img.height());
    if width < 2 || height < 2 {
        return;
    }

    // Step 1: Check if already segmented/transparent (e.g. from MediaPipe)
    let mut transparent_edge_count = 0;
    let mut total_edge_samples = 0;
    let step_x = (width / 16).max(1);
    let step_y = (height / 16).max(1);

    for x in (0..width).step_by(step_x as usize) {
        for &y in &[0, height - 1] {
            total_edge_samples += 1;
            if img.get_pixel(x, y)[3] < 30 {
                transparent_edge_count += 1;
            }
        }
    }
    for y in (0..height).step_by(step_y as usize) {
        for &x in &[0, width - 1] {
            total_edge_samples += 1;
            if img.get_pixel(x, y)[3] < 30 {
                transparent_edge_count += 1;
            }
        }
    }

    if total_edge_samples > 0 && (transparent_edge_count as f32 / total_edge_samples as f32) > 0.20 {
        return;
    }

    // Step 2: Detect if image is dominated by Green Screen or Blue Screen (Chroma Key)
    let mut green_count = 0;
    let mut blue_count = 0;
    let mut total_checked = 0;

    for x in (0..width).step_by(step_x as usize) {
        for &y in &[0, height - 1] {
            let px = *img.get_pixel(x, y);
            if px[3] > 50 {
                total_checked += 1;
                if is_green_chroma(px) {
                    green_count += 1;
                } else if is_blue_chroma(px) {
                    blue_count += 1;
                }
            }
        }
    }
    for y in (0..height).step_by(step_y as usize) {
        for &x in &[0, width - 1] {
            let px = *img.get_pixel(x, y);
            if px[3] > 50 {
                total_checked += 1;
                if is_green_chroma(px) {
                    green_count += 1;
                } else if is_blue_chroma(px) {
                    blue_count += 1;
                }
            }
        }
    }

    let edge_total = total_checked.max(1) as f32;
    if (green_count as f32 / edge_total) > 0.20 {
        apply_green_screen_key(img);
        return;
    } else if (blue_count as f32 / edge_total) > 0.20 {
        apply_blue_screen_key(img);
        return;
    }

    // Step 3: Multi-Seed Boundary Adaptive Flood-Fill
    apply_multi_corner_adaptive_cutout(img, tolerance.max(42));
}

fn is_green_chroma(px: Rgba<u8>) -> bool {
    let r = px[0] as f32;
    let g = px[1] as f32;
    let b = px[2] as f32;
    let max_rb = r.max(b);
    let diff = g - max_rb;
    diff > 35.0 && g > 75.0
}

fn apply_green_screen_key(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let a = pixel[3];

        if a < 10 {
            continue;
        }

        let max_rb = r.max(b);
        let diff = g - max_rb;

        if diff > 35.0 && g > 70.0 {
            *pixel = Rgba([0, 0, 0, 0]);
        } else if diff > 15.0 && g > 60.0 {
            let alpha_factor = 1.0 - ((diff - 15.0) / 20.0).clamp(0.0, 1.0);
            let new_a = (a as f32 * alpha_factor) as u8;
            let new_g = max_rb.min(g) as u8;
            *pixel = Rgba([pixel[0], new_g, pixel[2], new_a]);
        }
    }
}

fn is_blue_chroma(px: Rgba<u8>) -> bool {
    let r = px[0] as f32;
    let g = px[1] as f32;
    let b = px[2] as f32;
    let max_rg = r.max(g);
    let diff = b - max_rg;
    diff > 35.0 && b > 75.0
}

fn apply_blue_screen_key(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let a = pixel[3];

        if a < 10 {
            continue;
        }

        let max_rg = r.max(g);
        let diff = b - max_rg;

        if diff > 35.0 && b > 70.0 {
            *pixel = Rgba([0, 0, 0, 0]);
        } else if diff > 15.0 && b > 60.0 {
            let alpha_factor = 1.0 - ((diff - 15.0) / 20.0).clamp(0.0, 1.0);
            let new_a = (a as f32 * alpha_factor) as u8;
            let new_b = max_rg.min(b) as u8;
            *pixel = Rgba([pixel[0], pixel[1], new_b, new_a]);
        }
    }
}

fn apply_multi_corner_adaptive_cutout(img: &mut RgbaImage, tolerance: u8) {
    let (width, height) = (img.width(), img.height());
    let mut visited = vec![false; (width * height) as usize];
    let mut queue = VecDeque::new();
    let tol = tolerance as f32;

    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let guard_radius_sq = (width as f32 * 0.28) * (width as f32 * 0.28);

    // Collect perimeter seeds: 4 corners + sampled border points
    let mut seeds = Vec::new();
    seeds.push((0, 0));
    seeds.push((width - 1, 0));
    seeds.push((0, height - 1));
    seeds.push((width - 1, height - 1));

    let step = (width / 32).max(1);
    for x in (0..width).step_by(step as usize) {
        seeds.push((x, 0));
        seeds.push((x, height - 1));
    }
    for y in (0..height).step_by(step as usize) {
        seeds.push((0, y));
        seeds.push((width - 1, y));
    }

    for (sx, sy) in seeds {
        let idx = (sy * width + sx) as usize;
        if visited[idx] {
            continue;
        }

        let seed_color = *img.get_pixel(sx, sy);
        if seed_color[3] < 10 {
            continue;
        }

        visited[idx] = true;
        queue.push_back((sx, sy, seed_color));

        while let Some((x, y, orig_seed)) = queue.pop_front() {
            let cur_px = *img.get_pixel(x, y);
            img.put_pixel(x, y, Rgba([0, 0, 0, 0]));

            let neighbors = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];

            for (nx, ny) in neighbors {
                if nx < width && ny < height {
                    let n_idx = (ny * width + nx) as usize;
                    if !visited[n_idx] {
                        let npx = *img.get_pixel(nx, ny);
                        if npx[3] < 10 {
                            visited[n_idx] = true;
                            continue;
                        }

                        let d_seed = color_dist(npx, orig_seed);
                        let d_step = color_dist(npx, cur_px);

                        let dx = nx as f32 - center_x;
                        let dy = ny as f32 - center_y;
                        let dist_to_center_sq = dx * dx + dy * dy;
                        let in_center = dist_to_center_sq < guard_radius_sq;

                        let can_clear = if in_center {
                            d_seed <= (tol * 0.65)
                        } else {
                            d_seed <= tol || (d_step <= 16.0 && d_seed <= tol * 1.35)
                        };

                        if can_clear {
                            visited[n_idx] = true;
                            queue.push_back((nx, ny, orig_seed));
                        }
                    }
                }
            }
        }
    }
}

fn color_dist(a: Rgba<u8>, b: Rgba<u8>) -> f32 {
    let dr = a[0] as f32 - b[0] as f32;
    let dg = a[1] as f32 - b[1] as f32;
    let db = a[2] as f32 - b[2] as f32;
    (dr * dr + dg * dg + db * db).sqrt()
}
