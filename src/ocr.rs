use std::path::PathBuf;
use std::process::Command;

pub fn extract_text_from_image(image_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let tesseract_exe = r"C:\Program Files\Tesseract-OCR\tesseract.exe";
    
    let output = Command::new(tesseract_exe)
        .arg(image_path.to_str().unwrap())
        .arg("stdout")
        .arg("-c")
        .arg("preserve_interword_spaces=1")
        .arg("--psm")
        .arg("6")
        .output()?;
    
    if output.status.success() {
        let text = String::from_utf8(output.stdout)?;
        Ok(text.trim_end().to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Tesseract failed: {}", error).into())
    }
}