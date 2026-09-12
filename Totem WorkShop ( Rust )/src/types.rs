#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPage {
    Dashboard,
    AnimatedTotem,
    SkinForge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotemMode {
    TwoD,
    ThreeD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasSize {
    Light64 = 64,
    Normal128 = 128,
    Heavy256 = 256,
}

impl CanvasSize {
    pub fn size(&self) -> u32 {
        *self as u32
    }
}

pub struct MinecraftVersion {
    pub display: &'static str,
    pub pack_format: u32,
}

pub const MC_VERSIONS: &[MinecraftVersion] = &[
    MinecraftVersion { display: "1.21 (1.21 - 1.21.11)", pack_format: 34 },
    MinecraftVersion { display: "26 (26.1+)", pack_format: 84 },
    MinecraftVersion { display: "1.20 (1.20 - 1.20.6)", pack_format: 15 },
    MinecraftVersion { display: "1.19 (1.19 - 1.19.4)", pack_format: 9 },
    MinecraftVersion { display: "1.18 (1.18 - 1.18.2)", pack_format: 8 },
    MinecraftVersion { display: "1.17 (1.17 - 1.17.1)", pack_format: 7 },
    MinecraftVersion { display: "1.16 (1.16 - 1.16.5)", pack_format: 6 },
    MinecraftVersion { display: "1.15 (1.15 - 1.15.2)", pack_format: 5 },
    MinecraftVersion { display: "1.14 (1.14 - 1.14.4)", pack_format: 4 },
    MinecraftVersion { display: "1.13 (1.13 - 1.13.2)", pack_format: 4 },
    MinecraftVersion { display: "1.12 (1.12 - 1.12.2)", pack_format: 3 },
];

pub struct MinecraftColor {
    pub code: &'static str,
    pub display_name: &'static str,
    pub rgb: [u8; 3],
    pub is_light: bool,
}

pub const MC_COLORS: &[MinecraftColor] = &[
    MinecraftColor { code: "§4", display_name: "§4", rgb: [170, 0, 0], is_light: false },
    MinecraftColor { code: "§c", display_name: "§c", rgb: [255, 85, 85], is_light: true },
    MinecraftColor { code: "§6", display_name: "§6", rgb: [255, 170, 0], is_light: true },
    MinecraftColor { code: "§e", display_name: "§e", rgb: [255, 255, 85], is_light: true },
    MinecraftColor { code: "§2", display_name: "§2", rgb: [0, 170, 0], is_light: false },
    MinecraftColor { code: "§a", display_name: "§a", rgb: [85, 255, 85], is_light: true },
    MinecraftColor { code: "§b", display_name: "§b", rgb: [85, 255, 255], is_light: true },
    MinecraftColor { code: "§3", display_name: "§3", rgb: [0, 170, 170], is_light: false },
    MinecraftColor { code: "§1", display_name: "§1", rgb: [0, 0, 170], is_light: false },
    MinecraftColor { code: "§9", display_name: "§9", rgb: [85, 85, 255], is_light: false },
    MinecraftColor { code: "§d", display_name: "§d", rgb: [255, 85, 255], is_light: true },
    MinecraftColor { code: "§5", display_name: "§5", rgb: [170, 0, 170], is_light: false },
    MinecraftColor { code: "§f", display_name: "§f", rgb: [255, 255, 255], is_light: true },
    MinecraftColor { code: "§7", display_name: "§7", rgb: [170, 170, 170], is_light: true },
    MinecraftColor { code: "§8", display_name: "§8", rgb: [85, 85, 85], is_light: false },
    MinecraftColor { code: "§0", display_name: "§0", rgb: [0, 0, 0], is_light: false },
    MinecraftColor { code: "§l", display_name: "Bold", rgb: [68, 68, 68], is_light: false },
    MinecraftColor { code: "§r", display_name: "Reset", rgb: [34, 34, 34], is_light: false },
];

#[derive(Debug, Clone)]
pub enum TaskUpdate {
    Step(String),
    Progress(f32, String),
    Success(String),
    Error(String),
}

#[derive(Debug, Clone, Default)]
pub struct GenerationProgress {
    pub is_running: bool,
    pub step: String,
    pub fraction: f32,
    pub progress_text: String,
    pub result_message: Option<Result<String, String>>,
}

#[derive(Debug, Clone)]
pub enum FileUploadEvent {
    Video(String, Vec<u8>),
    VideoMeta(f32, usize, f32),
    Icon(String, Vec<u8>),
    Sound(String, Vec<u8>),
    Skin(String, Vec<u8>),
}

