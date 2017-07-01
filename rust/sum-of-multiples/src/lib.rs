pub fn sum_of_multiples(n: usize, multiples: &Vec<usize>) -> usize {
    (0..n).filter(|x| multiples.iter().any(|m| x % m == 0)).sum::<usize>()
}
