// Hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
use eframe::egui::Vec2;
#[cfg(not(target_arch = "wasm32"))]
use totem_workshop::app::TotemWorkshopApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Totem Workshop Pro 🪽 | Created by @CanBeShahab")
            .with_inner_size(Vec2::new(1100.0, 800.0))
            .with_min_inner_size(Vec2::new(880.0, 650.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Totem Workshop Pro",
        options,
        Box::new(|cc| Ok(Box::new(TotemWorkshopApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        if let Some(loading) = document.get_element_by_id("loading") {
            loading.remove();
        }

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(totem_workshop::app::TotemWorkshopApp::new(cc)))),
            )
            .await;

        if let Err(e) = start_result {
            log::error!("Failed to start eframe: {e:?}");
        }
    });
}
