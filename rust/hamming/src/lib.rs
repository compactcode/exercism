pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Err("length of arguments does not match");
    }

    let mut distance = 0;

    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            distance += 1
        }
    }

    Ok(distance)
}
