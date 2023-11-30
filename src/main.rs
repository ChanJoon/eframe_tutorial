#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_min_inner_size([300.0, 220.0])
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "Demo",
        native_options,
        Box::new(|cc| Box::new(eframe_tutorial::TutorialApp::new(cc))),
    )
}
