use goblin::elf::program_header::*;
use goblin::elf::Elf;
use super::CheckStatus;

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