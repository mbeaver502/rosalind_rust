use crate::dna::dna::DNA;
use crate::rna::rna::RNA;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn rna(fname: &str) {
    match parse_single_input(fname) {
        Err(e) => eprintln!("{}", e),
        Ok(s) => {
            let d = DNA::new(&s);
            let r = RNA::from(d);
            println!("{}", r);
        }
    }
}

type GenericResult<T> = Result<T, Box<dyn Error>>;

fn parse_single_input(fname: &str) -> GenericResult<String> {
    let mut file = BufReader::new(File::open(fname)?);
    let mut buf = String::new();

    file.read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
