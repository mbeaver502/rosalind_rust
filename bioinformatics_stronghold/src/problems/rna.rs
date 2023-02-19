use crate::dna::dna::DNA;
use crate::rna::rna::RNA;

use super::util;

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
