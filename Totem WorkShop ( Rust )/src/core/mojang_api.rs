use image::{ImageReader, RgbaImage};
use std::io::Cursor;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use base64::Engine;
#[cfg(not(target_arch = "wasm32"))]
use serde::Deserialize;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
struct MojangProfile {
    id: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
struct SessionProperty {
    name: String,
    value: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
struct SessionProfile {
    properties: Vec<SessionProperty>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
struct SkinTextureInfo {
    url: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
struct TexturesContainer {
    #[serde(rename = "SKIN")]
    skin: Option<SkinTextureInfo>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
struct TexturePayload {
    textures: TexturesContainer,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_skin_by_username(username: &str) -> Result<RgbaImage, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(8))
        .user_agent("TotemWorkshopPro/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    // Step 1: Query Mojang profile for UUID
    let profile_url = format!("https://api.mojang.com/users/profiles/minecraft/{username}");
    let resp = client.get(&profile_url).send();

    if let Ok(resp) = resp {
        if resp.status().is_success() {
            if let Ok(profile) = resp.json::<MojangProfile>() {
                // Step 2: Query session server for textures
                let session_url = format!(
                    "https://sessionserver.mojang.com/session/minecraft/profile/{}",
                    profile.id
                );
                if let Ok(s_resp) = client.get(&session_url).send() {
                    if s_resp.status().is_success() {
                        if let Ok(session) = s_resp.json::<SessionProfile>() {
                            if let Some(prop) = session.properties.iter().find(|p| p.name == "textures") {
                                if let Ok(decoded_bytes) =
                                    base64::engine::general_purpose::STANDARD.decode(&prop.value)
                                {
                                    if let Ok(payload) =
                                        serde_json::from_slice::<TexturePayload>(&decoded_bytes)
                                    {
                                        if let Some(skin_info) = payload.textures.skin {
                                            if let Ok(img_resp) = client.get(&skin_info.url).send() {
                                                if let Ok(bytes) = img_resp.bytes() {
                                                    if let Ok(img) = ImageReader::new(Cursor::new(bytes))
                                                        .with_guessed_format()
                                                        .map_err(|e| e.to_string())?
                                                        .decode()
                                                    {
                                                        return Ok(normalize_skin_format(img.to_rgba8()));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback 1: Minotar
    let fallback_url = format!("https://minotar.net/skin/{username}");
    if let Ok(resp) = client.get(&fallback_url).send() {
        if resp.status().is_success() {
            if let Ok(bytes) = resp.bytes() {
                if let Ok(img) = ImageReader::new(Cursor::new(bytes))
                    .with_guessed_format()
                    .map_err(|e| e.to_string())?
                    .decode()
                {
                    return Ok(normalize_skin_format(img.to_rgba8()));
                }
            }
        }
    }

    Err(format!("Could not fetch skin for user '{username}'. Please verify the username or internet connection."))
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_skin_by_username_wasm(username: &str) -> Result<RgbaImage, String> {
    // In browser, Minotar has open CORS headers allowing cross-origin skin fetches
    let fallback_url = format!("https://minotar.net/skin/{username}");
    let resp = reqwest::get(&fallback_url)
        .await
        .map_err(|e| format!("Network request failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Could not find player '{username}' on skin servers."));
    }

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    Ok(normalize_skin_format(img.to_rgba8()))
}

/// Ensures skin is 64x64 RGBA (converts classic 64x32 skins to 64x64)
pub fn normalize_skin_format(skin: RgbaImage) -> RgbaImage {
    let (w, h) = (skin.width(), skin.height());
    if w == 64 && h == 64 {
        return skin;
    }

    let mut normalized = RgbaImage::new(64, 64);
    image::imageops::overlay(&mut normalized, &skin, 0, 0);
    normalized
}
