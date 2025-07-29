#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ocr;
mod ui;

use app::ImageToTextApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Image to Text OCR",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.25);
            Ok(Box::new(ImageToTextApp::default()))
        }),
    )
}