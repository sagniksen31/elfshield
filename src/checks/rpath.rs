use super::RPathStatus;
use goblin::elf::Elf;
use goblin::elf64::dynamic::DT_RPATH;
use goblin::elf64::dynamic::DT_RUNPATH;
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

    RPathStatus { rpath, runpath }
}
