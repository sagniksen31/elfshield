use super::CheckStatus;
use goblin::elf::Elf;
use goblin::elf::header::*;
use goblin::elf::program_header::*;

pub fn is_pie(elf: &Elf) -> CheckStatus {
    match elf.header.e_type {
        ET_EXEC => CheckStatus::Disabled,

        ET_DYN => {
            // ET_DYN alone does not mean PIE — shared libraries are also ET_DYN.
            // PT_INTERP (the dynamic linker path) is only present in executables.
            let has_interp = elf.program_headers.iter().any(|ph| ph.p_type == PT_INTERP);

            if has_interp {
                CheckStatus::Enabled
            } else {
                CheckStatus::Unknown("shared library — PIE not applicable".to_string())
            }
        }

        _ => CheckStatus::Unknown("not an executable ELF type".to_string()),
    }
}
