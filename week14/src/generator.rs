// Week 14 — generator.rs

#![allow(dead_code, unused_imports)]
use rand::Rng;

pub fn generate_random(length: usize, use_symbols: bool) -> String {
    if length == 0 {
        panic!("length must be greater than 0");
    }

    let mut charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();

    if use_symbols {
        charset.push_str("!@#$%^&*");
    }

    let chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let mut rng = rand::thread_rng();

    let words: Vec<&str> = (0..word_count)
        .map(|_| {
            let idx = rng.gen_range(0..WORD_LIST.len());
            WORD_LIST[idx]
        })
        .collect();

    words.join(&separator.to_string())
}

pub fn generate_pin(length: usize) -> String {
    if length == 0 {
        panic!("length must be greater than 0");
    }

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let digit = rng.gen_range(0..10);
            char::from_digit(digit, 10).unwrap()
        })
        .collect()
}

// WORD LIST (unchanged)
pub const WORD_LIST: &[&str] = &[
    "apple", "river", "cloud", "stone", "flame", "ocean", "tiger", "maple", "storm", "frost",
    "eagle", "cedar", "brook", "ember", "coral", "prism", "solar", "lunar", "amber", "blaze",
    "cliff", "delta", "fable", "grove", "haven", "ivory", "jewel", "knoll", "lemon", "meadow",
    "north", "olive", "pearl", "quill", "ridge", "spark", "thorn", "umbra", "valor", "willow",
    "xenon", "yarrow", "zenith", "acorn", "birch", "crane", "drift", "elder", "flint", "glade",
    "hyena", "inlet", "junco", "kestrel",
];
