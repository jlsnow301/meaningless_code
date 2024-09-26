mod input;
mod output;
mod reader;
mod substitutions;
mod translation;

use input::file_input::get_filepath;
use output::writer::write_output;
use reader::chapter::read_chapter;

use translation::translate::translate;

fn main() {
    let filepath = get_filepath().expect("No file selected");

    let random_chapter = read_chapter(&filepath).expect("Failed to read chapter");

    let translated = translate(random_chapter);

    write_output(&translated).expect("Failed to write output");

    println!("Translation complete");
}
