mod input;
mod output;
mod reader;
mod substitutions;
mod translation;

use input::file_input::get_filepath;
use output::writer::write_output;
use reader::chapter::read_chapter;
use substitutions::{char_swap, word_swap};
use translation::translate::translate;

fn main() {
    let filepath = get_filepath().expect("No file selected");

    let random_chapter = read_chapter(&filepath).expect("Failed to read chapter");

    let word_swap = word_swap::map.clone();
    let char_swap = char_swap::map.clone();

    let translated = translate(random_chapter, &word_swap, &char_swap);

    write_output(&translated).expect("Failed to write output");

    println!("Translation complete");
}
