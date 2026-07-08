use goblin::elf::Elf;
pub(super) fn has_symbol(elf: &Elf, target: &str) -> bool {
    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name)
            && name == target
        {
            return true;
        }
    }
    for sym in &elf.syms {
        if let Some(name) = elf.strtab.get_at(sym.st_name)
            && name == target
        {
            return true;
        }
    }
    false
}
