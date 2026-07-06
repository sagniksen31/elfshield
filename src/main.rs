mod checks;
mod constants;

use std::env;
use std::fs;

use goblin::elf::Elf;
use goblin::elf::header::*;

use checks::*;

fn get_arch(machine: u16) -> &'static str {
    match machine {
        EM_386 => "x86",
        EM_ARM => "ARM",
        EM_X86_64 => "x86_64",
        EM_AARCH64 => "AArch64",
        EM_RISCV => "RISC-V",
        _ => "Unknown",
    }
}

fn get_file_type(file_type: u16) -> &'static str {
    match file_type {
        ET_REL => "Relocatable",
        ET_EXEC => "Executable",
        ET_DYN => "Shared Object / PIE",
        ET_CORE => "Core Dump",
        _ => "Unknown",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <binary>");
        return;
    }

    let data = fs::read(&args[1]).expect("Couldn't read file");

    let elf: Elf<'_> = Elf::parse(&data).expect("Not valid ELF");

    println!("Entry Point: 0x{:x}", elf.entry);

    println!("Sections: {}", elf.section_headers.len());

    println!("Architecture: {}", get_arch(elf.header.e_machine));

    println!("Type: {}", get_file_type(elf.header.e_type));

    println!("Sections:");

    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            println!("  {}", name);
        }
    }

    println!("  PIE:  {}", is_pie(&elf));
    println!("  NX:   {}", is_nx(&elf));
    println!("Canary: {}", check_canary(&elf));
    println!("Relro: {}", check_relro(&elf));
    println!("Fortify: {}", check_fortify(&elf));
    println!("{}", check_rpaths(&elf)); //runpath
}