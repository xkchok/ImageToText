#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ocr;
mod ui;

use app::ImageToTextApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image to Text OCR",
        options,
        Box::new(|_cc| Ok(Box::new(ImageToTextApp::default()))),
    )
}