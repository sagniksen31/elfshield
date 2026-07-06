use goblin::elf::Elf;
use goblin::elf::header::*;
use goblin::elf::program_header::*;
use goblin::elf64::dynamic::DT_NEEDED;
use goblin::elf64::dynamic::DT_RPATH;
use goblin::elf64::dynamic::DT_RUNPATH;
use crate::constants::FORTIFY_PAIRS;
#[derive(Debug, PartialEq)]
pub enum CheckStatus {
    Enabled,
    Disabled,
    Unknown(String), // carries an explanation of why it's unknown
}

#[derive(Debug, PartialEq)]
pub enum RelroStatus {
    None,
    Partial,
    Full,
}

#[derive(Debug)]
pub struct FortifyStatus {
    pub enabled: bool,
    pub fortified: usize,
    pub fortifiable: usize,
}
#[derive(Debug)]
pub struct RPathStatus {
    pub rpath: Option<String>,
    pub runpath: Option<String>,
}

#[derive(Debug)]
pub struct NeededLibraries {
    pub libraries: Vec<String>,
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

impl std::fmt::Display for RelroStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelroStatus::None      => write!(f, "None"),
            RelroStatus::Partial     => write!(f, "Partial"),
            RelroStatus::Full     => write!(f, "Full"),
        }
    }
}
//display status for fortify
impl std::fmt::Display for FortifyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.enabled {
            write!(
                f,
                "ENABLED (Fortified: {}, Fortifiable: {})",
                self.fortified,
                self.fortifiable
            )
        } else {
            write!(
                f,
                "DISABLED (Fortified: 0, Fortifiable: {})",
                self.fortifiable
            )
        }
    }
}
//runpath display
impl std::fmt::Display for RPathStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RPATH: {}\nRUNPATH: {}",
            self.rpath.as_deref().unwrap_or("None"),
            self.runpath.as_deref().unwrap_or("None")
        )
    }
}

impl std::fmt::Display for NeededLibraries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.libraries.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", self.libraries.join(", "))
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
            "No symbol information available".into(),
        );
    }
    if has_symbol(elf, "__stack_chk_fail") {
        CheckStatus::Enabled
    } else {
        CheckStatus::Disabled // we'll refine UNKNOWN later
    }
}

use goblin::elf::dynamic::{DF_1_NOW, DF_BIND_NOW};

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

pub fn check_fortify(elf: &Elf) -> FortifyStatus {
    let mut fortified = 0;
    let mut fortifiable = 0;

    for pair in FORTIFY_PAIRS {
        if has_symbol(elf, pair.fortified) {
            fortified += 1;
            fortifiable += 1;
        } else if has_symbol(elf, pair.base) {
            fortifiable += 1;
        }
    }

    FortifyStatus {
        enabled: fortified > 0,
        fortified,
        fortifiable,
    }
}

pub fn check_rpaths(elf: &Elf) -> RPathStatus {
    let mut rpath = None;
    let mut runpath = None;

    if let Some(dynamic) = &elf.dynamic {
        for dyn_entry in &dynamic.dyns {
            match dyn_entry.d_tag {
                DT_RPATH => {
                    if let Some(path) = elf.dynstrtab.get_at(dyn_entry.d_val as usize) {
                        rpath = Some(path.to_string());
                    }
                }
                DT_RUNPATH => {
                    if let Some(path) = elf.dynstrtab.get_at(dyn_entry.d_val as usize) {
                        runpath = Some(path.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    RPathStatus {
        rpath,
        runpath,
    }
}

pub fn check_stripped(elf: &Elf) -> CheckStatus {
    if elf.syms.is_empty() {
        CheckStatus::Enabled
    }
    else {
        CheckStatus::Disabled
    }
}

pub fn get_needed_libraries(elf: &Elf) -> NeededLibraries {
    let mut libraries = Vec::new();

    if let Some(dynamic) = &elf.dynamic {
        for dyn_entry in &dynamic.dyns {
            if dyn_entry.d_tag == DT_NEEDED {
                if let Some(lib) = elf.dynstrtab.get_at(dyn_entry.d_val as usize) {
                    libraries.push(lib.to_string());
                }
            }
        }
    }

    NeededLibraries { libraries }
}