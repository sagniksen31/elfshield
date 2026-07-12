# ELFShield

> **Fast, modern ELF binary hardening analysis written in Rust.**

ELFShield is a lightweight command-line utility for inspecting the security hardening of Linux ELF binaries. It performs static analysis directly on ELF metadata to determine which compiler and linker security mitigations are present, absent, or cannot be determined from the binary alone.

Designed for developers, reverse engineers, security researchers, malware analysts, and CTF players, ELFShield provides a quick overview of a binary's attack surface without executing it or relying on external tools such as `readelf`.

> **Current Status:** v0.1.0 (Initial Public Release)

---

## Features

- ⚡ Fast native Rust implementation
- 📦 Single lightweight executable
- 🔍 Direct ELF parsing using `goblin`
- 🛡️ Detects common compiler and linker hardening features
- 🔒 Safe static analysis — never executes the target binary
- 🧩 Modular architecture for easily adding new security checks

---

## Supported Checks

| Check | Description |
|--------|-------------|
| **PIE** | Position Independent Executable detection |
| **NX** | Non-Executable Stack detection |
| **Stack Canary** | Stack smashing protection detection |
| **RELRO** | None / Partial / Full RELRO detection |
| **FORTIFY_SOURCE** | Detects fortified libc wrappers and reports fortification coverage |
| **RPATH** | Detects embedded runtime library search paths |
| **RUNPATH** | Detects embedded runtime library search paths |
| **Stripped** | Determines whether debug symbol information has been removed |
| **Interpreter** | Displays the ELF interpreter (`PT_INTERP`) |
| **Needed Libraries** | Lists shared library dependencies (`DT_NEEDED`) |

---

## Example

```text
$ elfshield /bin/ls

ELFSHIELD v0.1.0
──────────────────────────────────────────────────
File           : ls
Path           : /bin/ls
Entry Point    : 0x6760
File Type      : PIE Executable
Architecture   : x86_64
Interpreter    : /lib64/ld-linux-x86-64.so.2

Security
--------
PIE            : ENABLED
NX             : ENABLED
Canary         : ENABLED
RELRO          : Full
FORTIFY        : ENABLED (6/10)

Metadata
--------
Stripped       : Yes

Search Paths
------------
RPATH          : None
RUNPATH        : None

Dependencies
------------
Needed Libraries: libselinux.so.1, libcap.so.2, libc.so.6
```

---

## Installation

### Install from crates.io

```bash
cargo install elfshield
```

Once installed, run:

```bash
elfshield /path/to/binary
```

---

### Build from Source

#### Requirements

- Rust (stable)
- Cargo

Clone the repository:

```bash
git clone https://github.com/sagniksen31/elfshield.git
cd elfshield
```

Build the release binary:

```bash
cargo build --release
```

Run:

```bash
./target/release/elfshield /path/to/binary
```

## Usage

Analyze a single ELF binary:

```bash
elfshield <path-to-elf>
```

Example:

```bash
elfshield /usr/bin/ssh
```

---

## How It Works

ELFShield performs **static analysis** by parsing ELF structures directly.

It does **not** execute the target binary.

Information is obtained from structures including:

- ELF Header
- Program Headers
- Dynamic Section
- Dynamic Symbol Table
- Static Symbol Table
- String Tables

Because the binary is never executed, ELFShield is safe to use on unknown or potentially malicious executables.

---

## Detection Philosophy

ELFShield aims to report only what can be inferred from the binary itself.

If a mitigation cannot be determined with confidence—for example due to symbol stripping or missing metadata—it reports **UNKNOWN** instead of making assumptions.

The goal is correctness and transparency rather than optimistic guesses.

---

## Supported Architectures

- x86
- x86_64
- ARM
- AArch64
- RISC-V

---

## Project Structure

```
src/
├── checks/
│   ├── canary.rs
│   ├── fortify.rs
│   ├── helpers.rs
│   ├── mod.rs
│   ├── needed_libs.rs
│   ├── nx.rs
│   ├── pie.rs
│   ├── relro.rs
│   ├── rpath.rs
│   └── stripped.rs
├── cli.rs
├── constants.rs
├── main.rs
└── utils.rs
```

---

## Why ELFShield?

Several existing tools already inspect ELF binaries, but many rely on shell scripts or external utilities.

ELFShield explores a different approach:

- Native Rust implementation
- Direct parsing of ELF structures
- Lightweight architecture
- Modular design for future security checks
- Easy integration into future automation and CI workflows

It was primarily built as a learning project to gain a deeper understanding of the ELF format, Linux program loading, compiler hardening techniques, and systems programming in Rust.

---

## Limitations

Current limitations include:

- ELF binaries only (PE and Mach-O are not supported)
- Static PIE detection can be ambiguous
- Detection is based solely on static ELF metadata
- JSON output is planned but not yet available
- Recursive directory analysis is not yet implemented

---

## Contributing

Contributions are welcome.

If you'd like to contribute:

```bash
cargo fmt
cargo clippy
cargo test
```

Please open an issue before making major architectural changes.

Bug reports, feature requests, and improvements to detection accuracy are always appreciated.

---

## License

Distributed under the MIT License.

See [LICENSE](LICENSE) for more information.

---

## Acknowledgements

ELFShield would not be possible without the excellent Rust ecosystem.

Special thanks to:

- **Goblin** for robust ELF parsing
- The Linux ELF specification
- Existing projects such as **checksec**, which inspired this project while taking a different implementation approach

---

## Author

**Sagnik Sen**

If you found this project useful, consider leaving a ⭐ on GitHub!
