use eframe::egui;
use rfd::FileDialog;
use crate::app::ImageToTextApp;

impl eframe::App for ImageToTextApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_processing_result();
        
        ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image to Text OCR");
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("Select Image").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "tiff", "gif"])
                        .pick_file()
                    {
                        self.set_selected_image(path);
                    }
                }
                
                if ui.button("Paste Image").clicked() {
                    if let Err(error) = self.paste_and_process_image() {
                        self.set_displayed_text(format!("Paste Error: {}", error));
                    }
                }
                
                if let Some(path) = self.selected_image_path() {
                    ui.label(format!("Selected: {}", 
                        path.file_name().unwrap_or_default().to_string_lossy()));
                }
            });
            
            ui.separator();
            
            ui.horizontal(|ui| {
                let button_enabled = self.selected_image_path().is_some() && !self.is_processing();
                if ui.add_enabled(button_enabled, egui::Button::new("Extract Text")).clicked() {
                    self.start_ocr_from_file();
                }
                
                if ui.button("Copy Text").clicked() {
                    if let Err(error) = self.copy_text_to_clipboard() {
                        eprintln!("Copy failed: {}", error);
                    }
                }
                
                if self.is_processing() {
                    ui.spinner();
                    ui.label("Processing...");
                }
            });
            
            ui.horizontal(|ui| {
                let mut remove_newlines = self.remove_newlines();
                if ui.checkbox(&mut remove_newlines, "Remove newlines").changed() {
                    self.set_remove_newlines(remove_newlines);
                }
            });
            
            ui.separator();
            
            ui.label("Extracted Text:");
            
            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 20.0)
                .show(ui, |ui| {
                    let mut displayed_text = self.get_displayed_text();
                    let response = ui.add_sized(
                        [ui.available_width(), ui.available_height()],
                        egui::TextEdit::multiline(&mut displayed_text)
                            .desired_width(f32::INFINITY)
                            .code_editor()
                    );
                    
                    if response.changed() {
                        self.set_displayed_text(displayed_text);
                    }
                });
        });
    }
}