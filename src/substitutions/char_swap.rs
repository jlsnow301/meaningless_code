use lazy_static::lazy_static;
use regex::Regex;

const CHAR_REPLACEMENTS: &[(&str, &str)] = &[
    (r"th", "ð"),
    (r"Th", "Ð"),
    (r"ph", "f"),
    (r"ight", "it"),
    (r"aught", "ɒt"),
    (r"ought", "ɒt"),
    (r"ght", "t"),
    (r"ote", "ot"),
    (r"ein", "an"),
    (r"eigh", "a"),
    (r"cei", "se"),
    (r"^ci", "sɛ"),
    (r"ing", "ɪng"),
    (r"kno", "no"),
    (r"ck", "c"),
    (r"k", "c"),
    (r"le$", "ʊl"),
    (r"tion$", "shʊn"),
    (r"^wh", "w"),
    (r"ter$", "tɛr"),
    (r"eau", "u"),
    (r"ea", "e"),
    (r"ee", "e"),
];

lazy_static! {
    static ref CHAR_REGEX_REPLACEMENTS: Vec<(Regex, &'static str)> = CHAR_REPLACEMENTS
        .iter()
        .map(|&(pattern, replacement)| (Regex::new(pattern).unwrap(), replacement))
        .collect();
}

// Take one word at a time, then apply all the replacements
pub fn apply_replacements(word: &str) -> String {
    let mut result = word.to_string();

    for (pattern, replacement) in CHAR_REGEX_REPLACEMENTS.iter() {
        result = pattern.replace_all(&result, *replacement).to_string();
    }

    result
}
