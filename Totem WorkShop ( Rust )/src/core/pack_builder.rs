use crate::types::TotemMode;
#[cfg(not(target_arch = "wasm32"))]
use image::imageops;
use image::RgbaImage;
use serde_json::json;
use std::io::Cursor;
use std::path::PathBuf;

pub struct PackMetaOptions {
    pub output_dir: PathBuf,
    pub pack_name: String,
    pub description: String,
    pub pack_format: u32,
    pub icon_path: Option<PathBuf>,
    pub sound_path: Option<PathBuf>,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct PackPaths {
    pub root: PathBuf,
    pub item_textures: PathBuf,
    #[allow(dead_code)]
    pub item_sounds: PathBuf,
}

pub fn sanitize_folder_name(name: &str) -> String {
    let forbidden = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let cleaned: String = name.chars().filter(|c| !forbidden.contains(c)).collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "TotemPack".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn prepare_pack_structure(opts: &PackMetaOptions) -> Result<PackPaths, String> {
    use std::fs;

    let folder_name = sanitize_folder_name(&opts.pack_name);
    let root = opts.output_dir.join(folder_name);

    if root.exists() {
        let _ = fs::remove_dir_all(&root);
    }

    let item_textures = root.join("assets").join("minecraft").join("textures").join("item");
    let item_sounds = root.join("assets").join("minecraft").join("sounds").join("item").join("totem");

    fs::create_dir_all(&item_textures)
        .map_err(|e| format!("Failed to create texture directory: {e}"))?;

    // Write pack.mcmeta
    let mcmeta = json!({
        "pack": {
            "pack_format": opts.pack_format,
            "description": if opts.description.trim().is_empty() {
                "Totem Workshop Pack"
            } else {
                opts.description.trim()
            }
        }
    });

    let mcmeta_str = serde_json::to_string_pretty(&mcmeta)
        .map_err(|e| format!("Failed to serialize pack.mcmeta: {e}"))?;
    fs::write(root.join("pack.mcmeta"), mcmeta_str)
        .map_err(|e| format!("Failed to write pack.mcmeta: {e}"))?;

    // Handle pack icon
    if let Some(ref icon_p) = opts.icon_path {
        if icon_p.exists() {
            if let Ok(img) = image::open(icon_p) {
                let resized = imageops::resize(&img.to_rgba8(), 64, 64, imageops::FilterType::Lanczos3);
                let _ = resized.save(root.join("pack.png"));
            }
        }
    }

    // Handle sound & sounds.json
    if let Some(ref sound_p) = opts.sound_path {
        if sound_p.exists() {
            let dest_sound = item_sounds.join("use.ogg");
            super::video_processor::process_audio_file(sound_p, &dest_sound)?;

            let sounds_json = json!({
                "item.totem.use": {
                    "sounds": ["item/totem/use"]
                }
            });
            let sounds_json_str = serde_json::to_string_pretty(&sounds_json)
                .map_err(|e| format!("Failed to serialize sounds.json: {e}"))?;
            let sounds_json_path = root.join("assets").join("minecraft").join("sounds.json");
            let _ = fs::write(sounds_json_path, sounds_json_str);
        }
    }

    Ok(PackPaths {
        root,
        item_textures,
        item_sounds,
    })
}

/// Builds an in-memory ZIP archive of the resource pack (compatible with Minecraft drag & drop)
pub fn build_pack_zip_buffer(
    opts: &PackMetaOptions,
    texture: &RgbaImage,
    mode: TotemMode,
    animation_mcmeta: Option<&str>,
    icon_bytes: Option<&[u8]>,
    sound_bytes: Option<(&str, &[u8])>,
) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let mut buf = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buf));
        let file_opts = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // 1. pack.mcmeta
        let mcmeta = json!({
            "pack": {
                "pack_format": opts.pack_format,
                "description": if opts.description.trim().is_empty() {
                    "Totem Workshop Pack"
                } else {
                    opts.description.trim()
                }
            }
        });
        zip.start_file("pack.mcmeta", file_opts).map_err(|e| e.to_string())?;
        zip.write_all(serde_json::to_string_pretty(&mcmeta).unwrap().as_bytes())
            .map_err(|e| e.to_string())?;

        // 2. pack.png if icon provided
        if let Some(ibytes) = icon_bytes {
            if let Ok(img) = image::load_from_memory(ibytes) {
                let resized = img.resize_exact(64, 64, image::imageops::FilterType::Nearest);
                let mut icon_png = Vec::new();
                if resized.write_to(&mut Cursor::new(&mut icon_png), image::ImageFormat::Png).is_ok() {
                    let _ = zip.start_file("pack.png", file_opts);
                    let _ = zip.write_all(&icon_png);
                }
            }
        }

        // 3. totem_of_undying.png
        zip.start_file("assets/minecraft/textures/item/totem_of_undying.png", file_opts)
            .map_err(|e| e.to_string())?;
        let mut png_bytes = Vec::new();
        texture.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .map_err(|e| e.to_string())?;
        zip.write_all(&png_bytes).map_err(|e| e.to_string())?;

        // 4. 3D models if in 3D mode
        if mode == TotemMode::ThreeD {
            zip.start_file("assets/minecraft/models/item/totem_of_undying.json", file_opts)
                .map_err(|e| e.to_string())?;
            let totem_m = crate::core::model_generator::get_totem_model_json();
            zip.write_all(serde_json::to_string_pretty(&totem_m).unwrap().as_bytes())
                .map_err(|e| e.to_string())?;

            zip.start_file("assets/minecraft/models/item/skin.json", file_opts)
                .map_err(|e| e.to_string())?;
            let skin_m = crate::core::model_generator::get_skin_model_json();
            zip.write_all(serde_json::to_string_pretty(&skin_m).unwrap().as_bytes())
                .map_err(|e| e.to_string())?;
        }

        // 5. Animation mcmeta if present
        if let Some(mcmeta_text) = animation_mcmeta {
            zip.start_file("assets/minecraft/textures/item/totem_of_undying.png.mcmeta", file_opts)
                .map_err(|e| e.to_string())?;
            zip.write_all(mcmeta_text.as_bytes()).map_err(|e| e.to_string())?;
        }

        // 6. Sound file and sounds.json if sound provided
        if let Some((_sound_name, sbytes)) = sound_bytes {
            let _ = zip.start_file("assets/minecraft/sounds/item/totem/use.ogg", file_opts);
            let _ = zip.write_all(sbytes);

            let sounds_json = json!({
                "item.totem.use": {
                    "sounds": [
                        { "name": "item/totem/use", "stream": false }
                    ]
                }
            });
            let _ = zip.start_file("assets/minecraft/sounds.json", file_opts);
            let _ = zip.write_all(serde_json::to_string_pretty(&sounds_json).unwrap().as_bytes());
        }

        zip.finish().map_err(|e| e.to_string())?;
    }

    Ok(buf)
}

#[cfg(target_arch = "wasm32")]
pub fn download_bytes_in_browser(filename: &str, bytes: &[u8]) -> Result<(), String> {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let window = web_sys::window().ok_or("No global window")?;
    let document = window.document().ok_or("No document")?;

    let uint8_array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    uint8_array.copy_from(bytes);

    let parts = js_sys::Array::new();
    parts.push(&uint8_array.buffer());

    let bag = BlobPropertyBag::new();
    bag.set_type("application/zip");

    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &bag)
        .map_err(|e| format!("{e:?}"))?;

    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("{e:?}"))?;

    let anchor = document.create_element("a")
        .map_err(|e| format!("{e:?}"))?
        .dyn_into::<HtmlAnchorElement>()
        .map_err(|e| format!("{e:?}"))?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    let _ = Url::revoke_object_url(&url);
    Ok(())
}
