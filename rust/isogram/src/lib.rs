use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for x in candidate.to_lowercase().chars().filter(|x| x.is_alphabetic()) {
        char_counts.entry(x).and_modify(|x| { *x += 1 }).or_insert(1);
    }

    return char_counts.values().all(|x| *x == 1);
}
