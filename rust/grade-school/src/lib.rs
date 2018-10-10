use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.entry(grade).or_insert(BTreeSet::new()).insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students.get(&grade).map(|students| students.iter().map(|x| x.to_string()).collect())
    }
}
