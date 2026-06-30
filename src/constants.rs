pub const FORTIFY_SYMBOLS: &[&str] = &[
    "__memcpy_chk",
    "__memmove_chk",
    "__mempcpy_chk",
    "__memset_chk",

    "__strcpy_chk",
    "__strncpy_chk",
    "__strcat_chk",
    "__strncat_chk",
    "__stpcpy_chk",
    "__stpncpy_chk",

    "__sprintf_chk",
    "__snprintf_chk",
    "__vsprintf_chk",
    "__vsnprintf_chk",

    "__printf_chk",
    "__fprintf_chk",
    "__vprintf_chk",
    "__vfprintf_chk",

    "__fwprintf_chk",
    "__vfwprintf_chk",
    "__swprintf_chk",
    "__vswprintf_chk",
    "__wprintf_chk",

    "__read_chk",
    "__pread_chk",
    "__pread64_chk",
    "__recv_chk",
    "__recvfrom_chk",

    "__getcwd_chk",
    "__readlink_chk",
    "__readlinkat_chk",
    "__realpath_chk",

    "__wcscpy_chk",
    "__wcsncpy_chk",
    "__wcscat_chk",
    "__wcsncat_chk",
    "__wmemcpy_chk",
    "__wmemmove_chk",
    "__wmempcpy_chk",
    "__wmemset_chk",

    "__fdelt_chk",

    "__open_2",
    "__open64_2",
    "__openat_2",
    "__openat64_2",
];