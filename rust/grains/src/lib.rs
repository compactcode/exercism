pub fn square(n: u32) -> u64 {
    if n < 1 || n > 64 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(n - 1)
}

pub fn total() -> u64 {
    (1..64 + 1).map(square).sum()
}
