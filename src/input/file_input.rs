use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn get_filepath() -> Option<String> {
    let data_dir = Path::new("data");

    if !data_dir.exists() || !data_dir.is_dir() {
        eprintln!("Error: 'data' directory does not exist.");
        return None;
    }

    let entries: Vec<_> = fs::read_dir(data_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "txt"))
        .collect();

    match entries.len() {
        0 => {
            println!("No .txt files found in the 'data' directory.");
            None
        }
        1 => {
            let filepath = entries[0].path();
            println!("Using the only available file: {:?}", filepath);
            Some(filepath.to_string_lossy().into_owned())
        }
        _ => {
            println!("Multiple files found. Please choose one:");
            for (i, entry) in entries.iter().enumerate() {
                println!("{}. {:?}", i + 1, entry.path().file_name().unwrap());
            }

            print!("Enter the number of the file you want to use: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice: usize = choice.trim().parse().unwrap_or(0);

            if choice > 0 && choice <= entries.len() {
                let filepath = entries[choice - 1].path();
                Some(filepath.to_string_lossy().into_owned())
            } else {
                println!("Invalid choice. No file selected.");
                None
            }
        }
    }
}
