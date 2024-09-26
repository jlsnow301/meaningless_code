use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn write_output(content: &str) -> std::io::Result<String> {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("translation_{}.txt", timestamp);
    let filepath = output_dir.join(filename);

    let mut file = File::create(&filepath)?;
    file.write_all(content.as_bytes())?;

    Ok(filepath.to_string_lossy().into_owned())
}
