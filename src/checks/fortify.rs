use super::FortifyStatus;
use super::helpers::has_symbol;
use crate::constants::FORTIFY_PAIRS;
use goblin::elf::Elf;

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
