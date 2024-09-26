use lazy_static::lazy_static;
use regex::Regex;

const REPLACEMENTS: &[(&str, &str)] = &[
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
    (r"ate$", "at"),
    (r"oo", "u"),
];

lazy_static! {
    static ref CHAR_REPLACEMENTS: Vec<(Regex, &'static str)> = REPLACEMENTS
        .iter()
        .map(|&(pattern, replacement)| (Regex::new(pattern).unwrap(), replacement))
        .collect();
}

pub fn apply_replacements(word: &str) -> String {
    let mut result = word.to_string();

    for (pattern, replacement) in CHAR_REPLACEMENTS.iter() {
        result = pattern.replace_all(&result, *replacement).to_string();
    }

    result
}
