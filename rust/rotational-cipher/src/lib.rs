use std::ascii::AsciiExt;

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn rotate_char(c: char, key: usize) -> char {
    let indexed_chars: Vec<char> = ALPHABET.chars().collect();

    let rotated_char = match ALPHABET.find(|a| a == c.to_ascii_lowercase()) {
        Some(index) => indexed_chars[(key + index) % 26],
        _ => c
    };

    match c.is_lowercase() {
        true  => rotated_char,
        false => rotated_char.to_ascii_uppercase()
    }
}

pub fn rotate(input: &str, key: usize) -> String {
    input
        .chars()
        .map(|c| rotate_char(c, key))
        .collect()
}
