pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = vec![];

    let end_index = digits.len() + 1 - len;

    for x in 0..end_index {
        result.push(digits[x..x + len].to_string());
    }

    return result;
}
