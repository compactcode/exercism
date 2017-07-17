use std::collections::HashMap;

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .to_lowercase()
        .chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect::<String>()
        .split_whitespace()
        .fold(HashMap::new(), |mut words, word| {
            *words.entry(word.to_string()).or_insert(0) += 1;
            words
        })
}
