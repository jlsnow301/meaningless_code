use rand::seq::SliceRandom;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub fn read_chapter<P: AsRef<Path>>(filepath: P) -> io::Result<String> {
    let content = read_to_string(filepath)?;

    let chapters: Vec<String> = content
        .split("CHAPTER ")
        .skip(1)
        .map(|chapter| format!("CHAPTER {}", chapter.trim()))
        .collect();

    let random_chapter = chapters
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No chapters found"))?;

    Ok(random_chapter.to_string())
}
