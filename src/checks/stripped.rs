use goblin::elf::Elf;
use super::CheckStatus;
pub fn check_stripped(elf: &Elf) -> CheckStatus {
    if elf.syms.is_empty() {
        CheckStatus::Enabled
    }
    else {
        CheckStatus::Disabled
    }
}