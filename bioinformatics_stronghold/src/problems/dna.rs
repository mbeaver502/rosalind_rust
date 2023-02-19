use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::dna::dna::DNA;

pub fn dna(fname: &str) {
    match parse_single_input(fname) {
        Err(e) => eprintln!("{}", e),
        Ok(s) => {
            let d = DNA::new(&s);
            println!("{}", d.count_nucleotides());
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
