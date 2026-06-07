use std::env;
use std::fs;

use goblin::elf::Elf;
use goblin::elf::header::*;
use goblin::elf64::program_header::*;

fn get_arch(machine: u16) -> &'static str {
    match machine {
        3 => "x86",
        40 => "ARM",
        62 => "x86_64",
        183 => "AArch64",
        243 => "RISC-V",
        _ => "Unknown",
    }
}

fn get_file_type(t: u16) -> &'static str {
    match t {
        ET_REL => "Relocatable",
        ET_EXEC => "Executable",
        ET_DYN => "Shared Object / PIE",
        ET_CORE => "Core Dump",
        _ => "Unknown",
    }
}

fn is_nx(t: &Elf) -> bool {
    for ph in &t.program_headers {
        if ph.p_type == PT_GNU_STACK {
            return (ph.p_flags & PF_X) == 0; //bitwise and to check if PF_X bit is set or not
        }
    }
    false
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <binary>");
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
    if elf.header.e_type == ET_DYN {
        println!("PIE: ENABLED");
    } else {
        println!("PIE: DISABLED");
    }
    if is_nx(&elf){
        println!("NX: ENABLED");
    } else {
        println!("NX: DISABLED");
    }
    
}
