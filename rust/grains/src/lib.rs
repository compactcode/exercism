pub fn square(n: u32) -> u64 {
    if n < 1 || n > 64 {
        panic!("Square must be between 1 and 64")
    }
    (1..n).fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    (1..64 + 1).map(0, |acc, n| acc + square(n))
}
