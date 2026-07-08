mod checks;
mod cli;
mod constants;
mod utils;

use std::env;
use std::fs;

use crate::cli::print_report;
use goblin::elf::Elf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <binary>");
        return;
    }
    let path = &args[1];
    let data = fs::read(path).expect("Couldn't read file");

    let elf: Elf<'_> = Elf::parse(&data).expect("Not valid ELF");

    print_report(path, &elf);
}
