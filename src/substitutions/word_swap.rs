use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref REPLACEMENTS: HashMap<&'static str, &'static str> = HashMap::from([
        ("cat", "cæt"),
        ("not", "nɒt"),
        ("cut", "cʌt"),
        ("seize", "sez"),
        ("put", "pʊt"),
        ("hot", "hɒt"),
        ("bed", "bɛd"),
        ("kite", "kit"),
        ("weird", "werd"),
        ("been", "bɛn"),
        ("her", "hɛr"),
        ("there", "ðɛr"),
        ("here", "her"),
        ("like", "lik"),
        ("their", "ðɛr"),
    ]);
}

pub fn has_replacement(word: &str) -> bool {
    REPLACEMENTS.contains_key(word)
}

pub fn apply_replacements(word: &str) -> String {
    if let Some(&replacement) = REPLACEMENTS.get(word) {
        replacement.to_string()
    } else {
        word.to_string()
    }
}
