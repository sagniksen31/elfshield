use goblin::elf::Elf;
use goblin::elf::header::*;
use goblin::elf::program_header::*;

#[derive(Debug, PartialEq)]
pub enum CheckStatus {
    Enabled,
    Disabled,
    Unknown(String), // carries an explanation of why it's unknown
}

// Implement Display so we can println!("{}", status) directly.
impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Enabled      => write!(f, "ENABLED"),
            CheckStatus::Disabled     => write!(f, "DISABLED"),
            CheckStatus::Unknown(msg) => write!(f, "UNKNOWN ({})", msg),
        }
    }
}

pub fn is_nx(elf: &Elf) -> CheckStatus {
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

pub fn is_pie(elf: &Elf) -> CheckStatus {
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

fn has_symbol(elf: &Elf, target: &str) -> bool {
    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if name == target {
                return true;
            }
        }
    }
    for sym in &elf.syms {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if name == target {
                return true;
            }
        }
    }
    false
}

pub fn check_canary(elf: &Elf) -> CheckStatus {
    if elf.syms.is_empty() && elf.dynsyms.is_empty() {
        return CheckStatus::Unknown(
            "Canary couldn't be determined, might be stripped and static".into(),
        );
    }
    if has_symbol(elf, "__stack_chk_fail") {
        CheckStatus::Enabled
    } else {
        CheckStatus::Disabled // we'll refine UNKNOWN later
    }
}