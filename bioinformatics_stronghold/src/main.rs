use problems::dna::dna;

pub mod dna;
pub mod problems;

fn main() {
    dna("./tests/inputs/rosalind_dna_1_dataset.txt");
}
