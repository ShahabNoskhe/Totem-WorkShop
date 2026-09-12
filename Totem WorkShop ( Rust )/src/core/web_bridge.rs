#[cfg(target_arch = "wasm32")]
use image::RgbaImage;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = totemWeb, js_name = pickFile)]
    fn js_pick_file(accept: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = totemWeb, js_name = extractVideoFrames)]
    fn js_extract_video_frames(
        video_bytes: &[u8],
        target_frames: u32,
        target_size: u32,
        remove_background: bool,
        on_progress: &JsValue,
    ) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = totemWeb, js_name = decodeAudioToPcm)]
    fn js_decode_audio_to_pcm(audio_bytes: &[u8]) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = totemWeb, js_name = probeVideo)]
    fn js_probe_video(video_bytes: &[u8]) -> js_sys::Promise;
}

#[cfg(target_arch = "wasm32")]
pub async fn pick_file(accept: &str) -> Option<(String, Vec<u8>)> {
    let promise = js_pick_file(accept);
    let res = JsFuture::from(promise).await.ok()?;
    if res.is_null() || res.is_undefined() {
        return None;
    }

    let name = js_sys::Reflect::get(&res, &JsValue::from_str("name"))
        .ok()?
        .as_string()?;
    let bytes_val = js_sys::Reflect::get(&res, &JsValue::from_str("bytes")).ok()?;
    let uint8 = js_sys::Uint8Array::new(&bytes_val);
    let bytes = uint8.to_vec();

    Some((name, bytes))
}

#[cfg(target_arch = "wasm32")]
pub async fn convert_audio_to_ogg(name: &str, bytes: &[u8]) -> (String, Vec<u8>) {
    use crate::core::audio_converter::{is_ogg, encode_pcm_to_ogg};

    if is_ogg(bytes) {
        let base_name = if name.to_lowercase().ends_with(".ogg") {
            name.to_string()
        } else {
            format!("{name}.ogg")
        };
        return (base_name, bytes.to_vec());
    }

    let promise = js_decode_audio_to_pcm(bytes);
    if let Ok(res) = JsFuture::from(promise).await {
        if !res.is_null() && !res.is_undefined() {
            if let (Ok(ch_val), Ok(sr_val), Ok(pcm_val)) = (
                js_sys::Reflect::get(&res, &JsValue::from_str("channels")),
                js_sys::Reflect::get(&res, &JsValue::from_str("sampleRate")),
                js_sys::Reflect::get(&res, &JsValue::from_str("pcm")),
            ) {
                let channels = ch_val.as_f64().unwrap_or(2.0) as u16;
                let sample_rate = sr_val.as_f64().unwrap_or(44100.0) as u32;
                let float_arr = js_sys::Float32Array::new(&pcm_val);
                let mut pcm = vec![0.0f32; float_arr.length() as usize];
                float_arr.copy_to(&mut pcm);

                if let Ok(ogg_data) = encode_pcm_to_ogg(&pcm, channels, sample_rate) {
                    let mut base_name = name.to_string();
                    if let Some(dot_idx) = base_name.rfind('.') {
                        base_name.truncate(dot_idx);
                    }
                    base_name.push_str(".ogg");
                    return (base_name, ogg_data);
                }
            }
        }
    }

    (name.to_string(), bytes.to_vec())
}

#[cfg(target_arch = "wasm32")]
pub async fn extract_video_frames<F>(
    video_bytes: &[u8],
    target_frames: u32,
    target_size: u32,
    remove_background: bool,
    mut on_progress: F,
) -> Result<Vec<RgbaImage>, String>
where
    F: FnMut(f32, String) + 'static,
{
    let cb = Closure::wrap(Box::new(move |pct: f64, step: String| {
        on_progress(pct as f32, step);
    }) as Box<dyn FnMut(f64, String)>);

    let promise = js_extract_video_frames(
        video_bytes,
        target_frames,
        target_size,
        remove_background,
        cb.as_ref(),
    );

    let res = JsFuture::from(promise)
        .await
        .map_err(|e| format!("{e:?}"))?;

    drop(cb);

    let array: js_sys::Array = res
        .dyn_into()
        .map_err(|_| "Failed to cast frames to JS Array".to_string())?;

    let mut frames = Vec::new();
    for i in 0..array.length() {
        let item = array.get(i);
        let uint8 = js_sys::Uint8Array::new(&item);
        let raw_bytes = uint8.to_vec();
        let img = RgbaImage::from_raw(target_size, target_size, raw_bytes)
            .ok_or_else(|| format!("Invalid frame data dimensions at index {i}"))?;
        frames.push(img);
    }

    Ok(frames)
}

#[cfg(target_arch = "wasm32")]
pub async fn probe_video_web(video_bytes: &[u8]) -> Option<(f32, usize, f32)> {
    let promise = js_probe_video(video_bytes);
    let res = JsFuture::from(promise).await.ok()?;
    if res.is_null() || res.is_undefined() {
        return None;
    }
    let duration = js_sys::Reflect::get(&res, &JsValue::from_str("duration")).ok()?.as_f64()? as f32;
    let fps = js_sys::Reflect::get(&res, &JsValue::from_str("fps")).ok()?.as_f64()? as f32;
    let total_frames = js_sys::Reflect::get(&res, &JsValue::from_str("totalFrames")).ok()?.as_f64()? as usize;
    Some((fps, total_frames, duration))
}
