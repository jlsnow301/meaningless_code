use regex::Regex;
use std::collections::HashMap;

pub fn translate(
    content: String,
    word_swap: &HashMap<&str, &str>,
    char_swap: &HashMap<&str, &str>,
) -> String {
    let mut result = content;

    for (pattern, replacement) in word_swap.iter() {
        let re = Regex::new(&format!(r"\b{}\b", regex::escape(pattern))).unwrap();
        result = re.replace_all(&result, *replacement).to_string();
    }

    for (pattern, replacement) in char_swap.iter() {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, *replacement).to_string();
    }

    result
}
