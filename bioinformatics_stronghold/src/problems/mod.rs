mod util;

use crate::dna::dna::DNA;
use crate::rna::rna::RNA;

/// https://rosalind.info/problems/dna/
pub fn dna(fname: &str) {
    match util::parse_single_input(fname) {
        Err(e) => eprintln!("{}", e),
        Ok(s) => {
            let d = DNA::new(&s);
            println!("{}", d.count_nucleotides());
        }
    }
}

/// https://rosalind.info/problems/rna/
pub fn rna(fname: &str) {
    match util::parse_single_input(fname) {
        Err(e) => eprintln!("{}", e),
        Ok(s) => {
            let d = DNA::new(&s);
            let r = RNA::from(d);
            println!("{}", r);
        }
    }
}

/// https://rosalind.info/problems/revc/
pub fn revc(fname: &str) {
    match util::parse_single_input(fname) {
        Err(e) => eprintln!("{}", e),
        Ok(s) => {
            let d = DNA::new(&s);
            println!("{}", d.reverse_complement());
        }
    }
}
