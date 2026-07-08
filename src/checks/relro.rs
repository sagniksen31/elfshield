use goblin::elf::Elf;
use goblin::elf::program_header::*;
use goblin::elf::dynamic::{DF_1_NOW, DF_BIND_NOW};
use super::RelroStatus;

pub fn check_relro(elf: &Elf) -> RelroStatus {
    // Step 1: Does the binary contain a PT_GNU_RELRO segment?
    let has_relro = elf
        .program_headers
        .iter()
        .any(|ph| ph.p_type == PT_GNU_RELRO);

    if !has_relro {
        return RelroStatus::None;
    }

    // Step 2: Check whether BIND_NOW/NOW is enabled.
    if let Some(dynamic) = &elf.dynamic {
        let bind_now = (dynamic.info.flags & DF_BIND_NOW) != 0
            || (dynamic.info.flags_1 & DF_1_NOW) != 0;

        if bind_now {
            RelroStatus::Full
        } else {
            RelroStatus::Partial
        }
    } else {
        // Static binaries may not have a .dynamic section.
        // They cannot express BIND_NOW, so conservatively report Partial.
        RelroStatus::Partial
    }
}