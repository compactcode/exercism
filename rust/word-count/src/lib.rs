use std::collections::HashMap;

fn invalid_char(c: char) -> bool {
    !c.is_alphanumeric() && !c.is_whitespace()
}

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .to_lowercase()
        .replace(invalid_char, "")
        .split_whitespace()
        .fold(HashMap::new(), |mut words, word| {
            *words.entry(word.to_string()).or_insert(0) += 1;
            words
        })
}
