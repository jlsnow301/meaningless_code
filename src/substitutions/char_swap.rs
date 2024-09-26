use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref map: HashMap<&'static str, &'static str> = HashMap::from([
        ("th", "ð"),
        ("Th", "Ð"),
        ("ph", "f"),
        ("ight", "it"),
        ("aught", "ɒt"),
        ("ought", "ɒt"),
        ("ght", "t"),
        ("ote", "ot"),
        ("ein", "an"),
        ("eigh", "a"),
        ("cei", "se"),
        ("ei", "i"),
        ("ing", "ɪng"),
        ("kno", "no"),
        ("ck", "c"),
        ("k", "c"),
        (r"le$", "ʊl"),
        (r"tion$", "shʊn"),
        (r"^wh", "w"),
        (r"ter$", "tɛr"),
        ("eau", "u"),
        ("ea", "e"),
        ("ee", "ɛ"),
    ]);
}
