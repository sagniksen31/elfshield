use super::CheckStatus;
use goblin::elf::Elf;
pub fn check_stripped(elf: &Elf) -> CheckStatus {
    if elf.syms.is_empty() {
        CheckStatus::Enabled
    } else {
        CheckStatus::Disabled
    }
}
