pub mod dna;
pub mod problems;
pub mod rna;

mod cli;

fn main() {
    if let Err(e) = cli::get_args().and_then(cli::run) {
        eprintln!("Encountered error: {}", e);
        std::process::exit(1);
    }
}
