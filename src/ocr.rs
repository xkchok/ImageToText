use std::path::PathBuf;
use rusty_tesseract::{Image, Args};
use image::ImageReader;
use std::collections::HashMap;

pub fn extract_text_from_image(image_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let dynamic_image = ImageReader::open(image_path)?
        .decode()?;
    
    let img = Image::from_dynamic_image(&dynamic_image)?;
    
    let args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from([
            ("preserve_interword_spaces".into(), "1".into()),
        ]),
        psm: Some(6),
        ..Args::default()
    };
    
    let text = rusty_tesseract::image_to_string(&img, &args)?;
    Ok(text.trim_end().to_string())
}