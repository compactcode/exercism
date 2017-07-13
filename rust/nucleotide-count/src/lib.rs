use std::collections::HashMap;
use std::iter::once;

const NUCLEOTIDES: &str = "ACGT";

fn is_nucleotide(c: char) -> bool {
    NUCLEOTIDES.contains(c)
}

pub fn count(c: char, dna: &str) -> Result<usize, ()> {
    if dna.chars().chain(once(c)).all(is_nucleotide) {
        Ok(dna.matches(c).count())
    } else {
        Err(())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let mut result = HashMap::new();

    for n in NUCLEOTIDES.chars() {
        result.insert(n, count(n, dna)?);
    }

    Ok(result)
}
