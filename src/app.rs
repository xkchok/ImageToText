use std::path::PathBuf;
use std::sync::mpsc;
use arboard::Clipboard;
use tempfile::NamedTempFile;
use crate::ocr::extract_text_from_image;

pub struct ImageToTextApp {
    selected_image_path: Option<PathBuf>,
    original_extracted_text: String,
    processing: bool,
    receiver: Option<mpsc::Receiver<Result<String, String>>>,
    clipboard: Option<Clipboard>,
    remove_newlines: bool,
}

impl Default for ImageToTextApp {
    fn default() -> Self {
        let clipboard = Clipboard::new().ok();
        Self {
            selected_image_path: None,
            original_extracted_text: String::new(),
            processing: false,
            receiver: None,
            clipboard,
            remove_newlines: false,
        }
    }
}

impl ImageToTextApp {
    pub fn selected_image_path(&self) -> &Option<PathBuf> {
        &self.selected_image_path
    }


    pub fn get_displayed_text(&self) -> String {
        if self.remove_newlines {
            self.original_extracted_text.replace("\r\n", " ").replace('\n', " ").replace('\r', " ")
        } else {
            self.original_extracted_text.clone()
        }
    }

    pub fn set_displayed_text(&mut self, text: String) {
        self.original_extracted_text = text;
    }

    pub fn is_processing(&self) -> bool {
        self.processing
    }

    pub fn remove_newlines(&self) -> bool {
        self.remove_newlines
    }

    pub fn set_remove_newlines(&mut self, remove: bool) {
        self.remove_newlines = remove;
    }

    pub fn copy_text_to_clipboard(&mut self) -> Result<(), String> {
        let text_to_copy = self.get_displayed_text();
        if let Some(clipboard) = &mut self.clipboard {
            clipboard.set_text(&text_to_copy)
                .map_err(|e| format!("Failed to copy to clipboard: {}", e))
        } else {
            Err("Clipboard not available".to_string())
        }
    }

    pub fn set_selected_image(&mut self, path: PathBuf) {
        self.selected_image_path = Some(path);
        self.original_extracted_text.clear();
    }

    pub fn check_processing_result(&mut self) {
        if let Some(receiver) = &self.receiver {
            if let Ok(result) = receiver.try_recv() {
                self.processing = false;
                match result {
                    Ok(text) => {
                        self.original_extracted_text = text;
                    },
                    Err(error) => self.original_extracted_text = format!("Error: {}", error),
                }
                self.receiver = None;
            }
        }
    }

    pub fn start_ocr_from_file(&mut self) {
        if let Some(path) = &self.selected_image_path {
            self.start_ocr_processing(path.clone());
        }
    }

    pub fn paste_and_process_image(&mut self) -> Result<(), String> {
        if let Some(clipboard) = &mut self.clipboard {
            match clipboard.get_image() {
                Ok(img_data) => {
                    let temp_file = NamedTempFile::with_suffix(".png")
                        .map_err(|e| format!("Failed to create temp file: {}", e))?;
                    
                    let image = image::RgbaImage::from_raw(
                        img_data.width as u32,
                        img_data.height as u32,
                        img_data.bytes.into_owned()
                    ).ok_or("Failed to create image from clipboard data")?;
                    
                    image.save(temp_file.path())
                        .map_err(|e| format!("Failed to save image: {}", e))?;
                    
                    let path = temp_file.path().to_path_buf();
                    self.selected_image_path = Some(path.clone());
                    self.start_ocr_processing(path);
                    
                    std::mem::forget(temp_file);
                    Ok(())
                },
                Err(_) => Err("No image found in clipboard".to_string())
            }
        } else {
            Err("Clipboard not available".to_string())
        }
    }

    fn start_ocr_processing(&mut self, path: PathBuf) {
        self.processing = true;
        self.original_extracted_text = "Processing...".to_string();
        
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        
        std::thread::spawn(move || {
            let result = extract_text_from_image(&path)
                .map_err(|e| e.to_string());
            let _ = sender.send(result);
        });
    }
}