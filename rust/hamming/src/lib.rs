pub fn hamming_distance(a: &str, b: &str) -> Result<usize, ()> {
    if a.len() == b.len() {
        Ok(
            a.chars()
            .zip(b.chars())
            .filter(|&(a, b)| a != b)
            .count()
       )
    } else {
        Err(())
    }
}
