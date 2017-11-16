fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }

    if n == 3 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    if n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let mut w = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += w;
        w = 6 - w;
    }

    return true;
}

struct PrimeIter { state: u64 }

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.state += 1;

            if is_prime(self.state) {
                return Some(self.state);
            }
        }
    }
}

pub fn nth(n: usize) -> Result<u64, ()> {
    match n {
        0 => Err(()),
        _ => Ok(PrimeIter { state: 0 }.nth(n).unwrap())
    }
}
