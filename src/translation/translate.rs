use crate::substitutions::{char_swap, word_swap};
use regex::Regex;

fn apply_all(word: &str, edited: &mut String) {
    let replaced = if word_swap::has_replacement(word) {
        word_swap::apply_replacements(word)
    } else {
        char_swap::apply_replacements(word)
    };

    edited.push_str(&replaced);
}

pub fn translate(content: String) -> String {
    let mut edited = String::new();
    let re = Regex::new(r"(\w+|\W+)").unwrap();

    for cap in re.captures_iter(&content) {
        let part = &cap[0];
        if part.chars().all(char::is_alphabetic) {
            apply_all(part, &mut edited);
        } else {
            edited.push_str(part);
        }
    }

    edited
}
