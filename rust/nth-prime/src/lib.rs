// https://stackoverflow.com/questions/1801391/what-is-the-best-algorithm-for-checking-if-a-number-is-prime#1801446
fn is_prime(n: u64) -> bool {
    return match n {
        2 => true,
        3 => true,
        x if x % 2 == 0 => false,
        x if x % 3 == 0 => false,
        _ => {
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
    };
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
    if n < 1 {
        return Err(());
    }

    Ok(PrimeIter { state: 0 }.nth(n).unwrap())
}
