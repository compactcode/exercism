use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACGT";

fn is_nucleotide(c: char) -> bool {
    NUCLEOTIDES.contains(c)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, ()> {
    if is_nucleotide(nucleotide) && dna.chars().all(is_nucleotide) {
        Ok(dna.matches(nucleotide).count())
    } else {
        Err(())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let counts = "ACGT".chars().map(|nucleotide| (nucleotide, count(nucleotide, dna)));
    let mut result = HashMap::new();
    for (c, r) in counts {
        if r.is_ok() {
            result.insert(c, r.unwrap());
        }
    }
    if result.is_empty() {
        Err(())
    } else {
        Ok(result)
    }
}
