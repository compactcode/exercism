pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];

    let mut candidate = 2;

    while n > 1 {
      while n % candidate == 0 {
	  prime_factors.push(candidate);
	  n /= candidate;
      }
      candidate += 1;
    }

    return prime_factors
}
