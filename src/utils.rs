use goblin::elf::header::*;
pub fn get_arch(machine: u16) -> &'static str {
    match machine {
        EM_386 => "x86",
        EM_ARM => "ARM",
        EM_X86_64 => "x86_64",
        EM_AARCH64 => "AArch64",
        EM_RISCV => "RISC-V",
        _ => "Unknown",
    }
}

pub fn get_file_type(file_type: u16) -> &'static str {
    match file_type {
        ET_REL => "Relocatable",
        ET_EXEC => "Executable",
        ET_DYN => "Shared Object / PIE",
        ET_CORE => "Core Dump",
        _ => "Unknown",
    }
}
