use std::fmt::Display;

/// Represents a DNA strand.
#[derive(Debug)]
pub struct DNA {
    dna: String,
}

/// Represent the 'A', 'C', 'G', and 'T'
/// counts for a DNA strand.
#[derive(Debug)]
pub struct NucleotideCount {
    count_a: u64,
    count_c: u64,
    count_g: u64,
    count_t: u64,
}

impl DNA {
    /// Create a DNA strand from the given
    /// nucleotide sequence (`s`).
    pub fn new(s: &str) -> Self {
        Self {
            dna: String::from(s),
        }
    }

    /// Count the number of nucleotides in the DNA strand.
    /// Counts returned in (A, C, G, T) order.
    pub fn count_nucleotides(&self) -> NucleotideCount {
        let mut count_a = 0;
        let mut count_c = 0;
        let mut count_g = 0;
        let mut count_t = 0;

        self.dna.chars().for_each(|c| match c {
            'A' | 'a' => count_a += 1,
            'C' | 'c' => count_c += 1,
            'G' | 'g' => count_g += 1,
            'T' | 't' => count_t += 1,
            _ => {}
        });

        NucleotideCount {
            count_a,
            count_c,
            count_g,
            count_t,
        }
    }

    /// Transcribe a DNA strand into an RNA strand.
    pub fn transcribe(&self) -> String {
        self.dna
            .chars()
            .map(|n| match n {
                'A' | 'C' | 'G' => n,
                'T' => 'U',
                _ => n,
            })
            .collect::<String>()
    }

    /// Get the reverse complement of a DNA strand.
    pub fn reverse_complement(&self) -> Self {
        let dna = self
            .dna
            .chars()
            .rev()
            .map(|c| match c {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => c,
            })
            .collect::<String>();

        Self { dna }
    }
}

impl Display for NucleotideCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.count_a, self.count_c, self.count_g, self.count_t
        )
    }
}

impl Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dna)
    }
}
