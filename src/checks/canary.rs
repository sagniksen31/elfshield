use goblin::elf::Elf;
use super::CheckStatus;
use super::helpers::has_symbol;
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