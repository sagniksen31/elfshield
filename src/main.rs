use std::env;
use std::fs;

use goblin::elf::Elf;
use goblin::elf::header::*;
use goblin::elf::program_header::*;

#[derive(Debug, PartialEq)]
enum CheckStatus {
    Enabled,
    Disabled,
    Unknown(String), // carries an explanation of why it's unknown
}

// Implement Display so we can println!("{}", status) directly.
impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Enabled       => write!(f, "ENABLED"),
            CheckStatus::Disabled      => write!(f, "DISABLED"),
            CheckStatus::Unknown(msg)  => write!(f, "UNKNOWN ({})", msg),
        }
    }
}

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

fn is_nx(elf: &Elf) -> CheckStatus {
    for ph in &elf.program_headers {
        if ph.p_type == PT_GNU_STACK {
            return if (ph.p_flags & PF_X) == 0 {
                CheckStatus::Enabled
            } else {
                CheckStatus::Disabled
            };
        }
    }
    // Absent PT_GNU_STACK does NOT mean NX is off.
    // Modern kernels default to NX-enabled, but we cannot guarantee it.
    CheckStatus::Unknown("PT_GNU_STACK segment absent — kernel default applies".to_string())
}

fn is_pie(elf: &Elf) -> CheckStatus {
    match elf.header.e_type {
        ET_EXEC => CheckStatus::Disabled,

        ET_DYN => {
            // ET_DYN alone does not mean PIE — shared libraries are also ET_DYN.
            // PT_INTERP (the dynamic linker path) is only present in executables.
            let has_interp = elf
                .program_headers
                .iter()
                .any(|ph| ph.p_type == PT_INTERP);

            if has_interp {
                CheckStatus::Enabled
            } else {
                CheckStatus::Unknown("shared library — PIE not applicable".to_string())
            }
        }

        _ => CheckStatus::Unknown("not an executable ELF type".to_string()),
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
    //print!("{:?}", elf.program_headers);
}
