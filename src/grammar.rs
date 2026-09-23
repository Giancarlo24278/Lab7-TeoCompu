use std::{collections::{BTreeMap, BTreeSet}, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    pub start: char,
    pub productions: BTreeMap<char, BTreeSet<String>>,
}

impl Grammar {
    pub fn new(start: char) -> Self {
        Self { start, productions: BTreeMap::new() }
    }

    pub fn add(&mut self, left: char, right: String) {
        self.productions.entry(left).or_default().insert(right);
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (left, rights) in &self.productions {
            let body = rights.iter().map(|r| if r.is_empty() { "ε" } else { r }).collect::<Vec<_>>().join(" | ");
            writeln!(f, "{left} → {body}")?;
        }
        Ok(())
    }
}
