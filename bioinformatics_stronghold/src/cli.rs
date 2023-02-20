use crate::problems;

use clap::{Arg, Command};

type CLIResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CLIArgs {
    problem: String,
    filename: String,
}

pub fn run(args: CLIArgs) -> CLIResult<()> {
    match args.problem.to_lowercase().as_str() {
        "dna" => problems::dna(&args.filename),
        "rna" => problems::rna(&args.filename),
        "revc" => problems::revc(&args.filename),
        _ => return Err(From::from(format!("Invalid option {}", args.problem))),
    }

    Ok(())
}

pub fn get_args() -> CLIResult<CLIArgs> {
    let matches = Command::new("biostrong")
        .version("0.1.0")
        .author("M Beaver")
        .about("Rosalind solutions written (poorly) in Rust.")
        .arg(
            Arg::new("problem")
                .required(true)
                .help("The Rosalind problem to process")
                .value_name("PROBLEM"),
        )
        .arg(
            Arg::new("filename")
                .required(true)
                .help("Input file")
                .value_name("FILE"),
        )
        .get_matches();

    // I'm sure there's a better way to do this...
    // We can safely unwrap here since both these args are required.
    let problem = String::from(matches.get_one::<String>("problem").unwrap());
    let filename = String::from(matches.get_one::<String>("filename").unwrap());

    Ok(CLIArgs { problem, filename })
}
