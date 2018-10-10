use std::collections::BTreeMap;

pub fn transform(old: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (score, letters) in old.iter() {
        for letter in letters.iter() {
            result.insert(letter.to_ascii_lowercase(), *score);
        }
    }

    return result;
}
