use crate::substitutions::{char_swap::apply_replacements, word_swap::REPLACEMENTS};

pub fn translate(content: String) -> String {
    let mut result = content;

    for (word, replacement) in REPLACEMENTS.iter() {
        result = result.replace(word, replacement);
    }

    let mut char_edited = String::new();
    let mut last_end = 0;

    for (start, part) in result.match_indices(char::is_whitespace) {
        char_edited.push_str(&apply_replacements(&result[last_end..start]));
        char_edited.push_str(part);
        last_end = start + part.len();
    }

    char_edited.push_str(&apply_replacements(&result[last_end..]));

    char_edited
}
