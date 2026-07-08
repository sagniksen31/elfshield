pub struct FortifyPair {
    pub base: &'static str,
    pub fortified: &'static str,
}

pub const FORTIFY_PAIRS: &[FortifyPair] = &[
    FortifyPair {
        base: "memcpy",
        fortified: "__memcpy_chk",
    },
    FortifyPair {
        base: "memmove",
        fortified: "__memmove_chk",
    },
    FortifyPair {
        base: "mempcpy",
        fortified: "__mempcpy_chk",
    },
    FortifyPair {
        base: "memset",
        fortified: "__memset_chk",
    },
    FortifyPair {
        base: "stpcpy",
        fortified: "__stpcpy_chk",
    },
    FortifyPair {
        base: "stpncpy",
        fortified: "__stpncpy_chk",
    },
    FortifyPair {
        base: "strcat",
        fortified: "__strcat_chk",
    },
    FortifyPair {
        base: "strcpy",
        fortified: "__strcpy_chk",
    },
    FortifyPair {
        base: "strncat",
        fortified: "__strncat_chk",
    },
    FortifyPair {
        base: "strncpy",
        fortified: "__strncpy_chk",
    },
    FortifyPair {
        base: "printf",
        fortified: "__printf_chk",
    },
    FortifyPair {
        base: "fprintf",
        fortified: "__fprintf_chk",
    },
    FortifyPair {
        base: "vprintf",
        fortified: "__vprintf_chk",
    },
    FortifyPair {
        base: "vfprintf",
        fortified: "__vfprintf_chk",
    },
    FortifyPair {
        base: "sprintf",
        fortified: "__sprintf_chk",
    },
    FortifyPair {
        base: "snprintf",
        fortified: "__snprintf_chk",
    },
    FortifyPair {
        base: "vsprintf",
        fortified: "__vsprintf_chk",
    },
    FortifyPair {
        base: "vsnprintf",
        fortified: "__vsnprintf_chk",
    },
];
