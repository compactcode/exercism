pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Err("length of arguments does not match");
    }

    Ok(a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count())
}
