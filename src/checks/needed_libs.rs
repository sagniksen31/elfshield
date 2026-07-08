use super::NeededLibraries;
use goblin::{elf::Elf, elf::dynamic::DT_NEEDED};
pub fn get_needed_libraries(elf: &Elf) -> NeededLibraries {
    let mut libraries = Vec::new();

    if let Some(dynamic) = &elf.dynamic {
        for dyn_entry in &dynamic.dyns {
            if dyn_entry.d_tag == DT_NEEDED
                && let Some(lib) = elf.dynstrtab.get_at(dyn_entry.d_val as usize)
            {
                libraries.push(lib.to_string());
            }
        }
    }

    NeededLibraries { libraries }
}
