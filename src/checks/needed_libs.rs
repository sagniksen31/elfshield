use goblin::{elf::Elf, elf64::dynamic::DT_NEEDED};
use super::NeededLibraries;
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