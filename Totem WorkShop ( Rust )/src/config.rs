use crate::core::i18n::Language;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(target_arch = "wasm32")]
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub output_dir: String,
    #[serde(default)]
    pub language: Language,
}

impl Default for AppConfig {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let default_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .to_string_lossy()
            .to_string();

        #[cfg(target_arch = "wasm32")]
        let default_dir = "Browser Download (ZIP)".to_string();

        Self {
            output_dir: default_dir,
            language: Language::English,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_config(path: &Path) -> AppConfig {
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                return config;
            }
        }
    }
    AppConfig::default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_config(path: &Path, config: &AppConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {e}"))?;
    std::fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {e}"))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn load_config(_path: &Path) -> AppConfig {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(val)) = storage.get_item("totem_workshop_config") {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&val) {
                    return config;
                }
            }
        }
    }
    AppConfig::default()
}

#[cfg(target_arch = "wasm32")]
pub fn save_config(_path: &Path, config: &AppConfig) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(val) = serde_json::to_string(config) {
                let _ = storage.set_item("totem_workshop_config", &val);
            }
        }
    }
    Ok(())
}
