use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_single_input(fname: &str) -> GenericResult<String> {
    let mut file = BufReader::new(File::open(fname)?);
    let mut buf = String::new();

    file.read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
