use super::CheckStatus;
use super::helpers::has_symbol;
use goblin::elf::Elf;
pub fn check_canary(elf: &Elf) -> CheckStatus {
    if elf.syms.is_empty() && elf.dynsyms.is_empty() {
        return CheckStatus::Unknown("No symbol information available".into());
    }
    if has_symbol(elf, "__stack_chk_fail") {
        CheckStatus::Enabled
    } else {
        CheckStatus::Disabled
    }
}
