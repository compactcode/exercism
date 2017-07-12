use std::collections::HashSet;
use std::iter::FromIterator;

pub fn is_pangram(sentence: &str) -> bool {
    let all:  HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());

    all.is_subset(&used)
}
