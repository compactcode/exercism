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
    let mut result = HashMap::new();

    for nucleotide in NUCLEOTIDES.chars() {
        result.insert(nucleotide, try!(count(nucleotide, dna)));
    }

    Ok(result)
}
