use std::process::Command;

pub mod tsrt {
    use super::*;

    pub fn read_text_from_image(image_path: &str) -> String {
        let output = Command::new("tesseract")
            .arg(image_path)
            .arg("stdout")
            .arg("-l")
            .arg("rus+eng") // язык
            .output();

        match output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            Ok(output) => {
                format!(
                    "Tesseract error: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            }
            Err(e) => {
                format!("Ошибка при запуске Tesseract: {}", e)
            }
        }
    }
}
