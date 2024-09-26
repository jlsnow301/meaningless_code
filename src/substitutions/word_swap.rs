use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref REPLACEMENTS: HashMap<&'static str, &'static str> = HashMap::from([
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
