use std::fmt::Display;

use crate::dna::dna::DNA;

#[derive(Debug)]
pub struct RNA {
    rna: String,
}

impl RNA {
    pub fn new(s: &str) -> Self {
        Self {
            rna: String::from(s),
        }
    }
}

impl Display for RNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rna)
    }
}

impl From<DNA> for RNA {
    fn from(value: DNA) -> Self {
        Self {
            rna: value.transcribe(),
        }
    }
}
