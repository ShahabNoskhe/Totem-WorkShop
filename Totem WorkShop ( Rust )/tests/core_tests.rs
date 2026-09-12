use image::RgbaImage;

#[test]
fn test_skin_2d_and_3d_generation() {
    // Create a dummy 64x64 skin
    let mut skin = RgbaImage::new(64, 64);
    for pixel in skin.pixels_mut() {
        *pixel = image::Rgba([200, 150, 100, 255]);
    }

    // Generate 2D totem
    let totem = totem_workshop::core::skin_processor::generate_2d_totem(&skin);
    assert_eq!(totem.width(), 16);
    assert_eq!(totem.height(), 16);

    // Verify totem silhouette cleared pixels (e.g. (0, 10) and (15, 10))
    assert_eq!(totem.get_pixel(0, 10)[3], 0);
    assert_eq!(totem.get_pixel(15, 10)[3], 0);

    // Generate 3D preview
    let preview_3d = totem_workshop::core::skin_processor::generate_3d_body_preview(&skin);
    assert_eq!(preview_3d.width(), 16);
    assert_eq!(preview_3d.height(), 32);
}

#[test]
fn test_animation_mcmeta() {
    let mcmeta = totem_workshop::core::spritesheet::generate_animation_mcmeta(10, 2, false);
    assert_eq!(mcmeta["animation"]["frametime"], 2);
    assert_eq!(mcmeta["animation"]["interpolate"], false);
    let frames = mcmeta["animation"]["frames"].as_array().unwrap();
    assert_eq!(frames.len(), 10);
    assert_eq!(frames[0], 0);
    assert_eq!(frames[9], 9);
}

#[test]
fn test_3d_models_generation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let res = totem_workshop::core::model_generator::generate_totem_3d_models(temp_dir.path());
    assert!(res.is_ok());

    let models_dir = temp_dir.path().join("assets").join("minecraft").join("models").join("item");
    assert!(models_dir.join("totem_of_undying.json").exists());
    assert!(models_dir.join("skin.json").exists());

    // Verify valid JSON
    let totem_json_str = std::fs::read_to_string(models_dir.join("totem_of_undying.json")).unwrap();
    let totem_val: serde_json::Value = serde_json::from_str(&totem_json_str).unwrap();
    assert_eq!(totem_val["parent"], "minecraft:item/skin");

    let skin_json_str = std::fs::read_to_string(models_dir.join("skin.json")).unwrap();
    let skin_val: serde_json::Value = serde_json::from_str(&skin_json_str).unwrap();
    assert!(skin_val["elements"].is_array());
    assert_eq!(skin_val["elements"].as_array().unwrap().len(), 12);
}

#[test]
fn test_pack_builder_structure() {
    let temp_dir = tempfile::tempdir().unwrap();
    let opts = totem_workshop::core::pack_builder::PackMetaOptions {
        output_dir: temp_dir.path().to_path_buf(),
        pack_name: "Test Totem Pack".to_string(),
        description: "§aCustom Totem".to_string(),
        pack_format: 34,
        icon_path: None,
        sound_path: None,
    };

    let paths = totem_workshop::core::pack_builder::prepare_pack_structure(&opts).unwrap();
    assert!(paths.root.exists());
    assert!(paths.item_textures.exists());

    let mcmeta_content = std::fs::read_to_string(paths.root.join("pack.mcmeta")).unwrap();
    let mcmeta: serde_json::Value = serde_json::from_str(&mcmeta_content).unwrap();
    assert_eq!(mcmeta["pack"]["pack_format"], 34);
    assert_eq!(mcmeta["pack"]["description"], "§aCustom Totem");
}

#[test]
fn test_audio_conversion_to_ogg() {
    use totem_workshop::core::audio_converter::{encode_pcm_to_ogg, is_ogg};

    // Create 0.5s 44100Hz sine wave PCM samples
    let sample_rate = 44100;
    let num_samples = sample_rate / 2;
    let mut pcm = Vec::with_capacity(num_samples as usize);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.5;
        pcm.push(sample);
    }

    let ogg_bytes = encode_pcm_to_ogg(&pcm, 1, sample_rate).expect("Encoding PCM to Ogg should succeed");
    assert!(ogg_bytes.len() > 100);
    assert!(is_ogg(&ogg_bytes), "Resulting buffer must be a valid Ogg container starting with 'OggS'");
}

#[test]
fn test_universal_background_removal() {
    use totem_workshop::core::video_processor::universal_remove_background;
    use image::Rgba;

    // 1. Test Green Screen
    let mut green_img = RgbaImage::new(32, 32);
    for pixel in green_img.pixels_mut() {
        // Bright green background
        *pixel = Rgba([10, 240, 20, 255]);
    }
    // Put a red subject in the center
    for x in 12..20 {
        for y in 12..20 {
            green_img.put_pixel(x, y, Rgba([220, 30, 40, 255]));
        }
    }
    universal_remove_background(&mut green_img, 35);
    // Background corner must be transparent
    assert_eq!(green_img.get_pixel(0, 0)[3], 0);
    // Subject center must remain opaque
    assert!(green_img.get_pixel(15, 15)[3] > 200);

    // 2. Test Solid Black Background
    let mut black_img = RgbaImage::new(32, 32);
    for pixel in black_img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255]);
    }
    for x in 12..20 {
        for y in 12..20 {
            black_img.put_pixel(x, y, Rgba([240, 200, 50, 255]));
        }
    }
    universal_remove_background(&mut black_img, 35);
    assert_eq!(black_img.get_pixel(0, 0)[3], 0);
    assert!(black_img.get_pixel(15, 15)[3] > 200);
}
