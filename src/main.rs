mod checks;
mod cli;
mod constants;
mod utils;

use std::env;
use std::fs;

use crate::cli::print_report;
use goblin::elf::Elf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: elfshield <path-to-elf>");
        std::process::exit(1);
    }
    let path = &args[1];
    let data = fs::read(path)?;
    let elf = Elf::parse(&data)?;

    print_report(path, &elf);
    Ok(())
}
