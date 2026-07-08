use crate::checks::*;
use crate::utils::{get_arch, get_file_type};
use goblin::elf::Elf;
use std::path::Path;
pub fn print_report(path: &str, elf: &Elf) {
    print_header(path, elf);

    print_security(elf);

    print_metadata(elf);

    print_search_paths(elf);

    print_dependencies(elf);
}

fn print_header(path: &str, elf: &Elf) {
    let filename = Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    println!("ELFSHIELD v{}", env!("CARGO_PKG_VERSION"));
    println!("{}", "─".repeat(50));

    print_field("File", filename);
    print_field("Path", path);
    print_field("Entry Point", format!("{:#x}", elf.header.e_entry));
    print_field("File Type", get_file_type(elf.header.e_type));
    print_field("Architecture", get_arch(elf.header.e_machine));
    print_field("Interpreter", elf.interpreter.unwrap_or("None (Static)"));
}

fn print_security(elf: &Elf) {
    print_section("Security");

    print_field("PIE", is_pie(elf));
    print_field("NX", is_nx(elf));
    print_field("Canary", check_canary(elf));
    print_field("RELRO", check_relro(elf));
    print_field("FORTIFY", check_fortify(elf));
}

fn print_metadata(elf: &Elf) {
    print_section("Metadata");
    let stripped = check_stripped(elf);

    match stripped {
        CheckStatus::Enabled => print_field("Stripped", "Yes"),
        CheckStatus::Disabled => print_field("Stripped", "No"),
        CheckStatus::Unknown(msg) => print_field("Stripped", format!("Unknown ({})", msg)),
    }
}

fn print_search_paths(elf: &Elf) {
    print_section("Search Paths");
    let paths = check_rpaths(elf);
    print_field("RPATH", paths.rpath.as_deref().unwrap_or("None"));
    print_field("RUNPATH", paths.runpath.as_deref().unwrap_or("None"));
}

fn print_dependencies(elf: &Elf) {
    print_section("Dependencies");
    print_field("Needed Libraries", get_needed_libraries(elf));
}

fn print_field<T: std::fmt::Display>(name: &str, value: T) {
    println!("{:<15}: {}", name, value);
}

fn print_section(title: &str) {
    println!("\n{}", title);
    println!("{}", "-".repeat(title.len()));
}
