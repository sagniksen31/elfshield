pub mod canary;
pub mod fortify;
pub mod helpers;
pub mod needed_libs;
pub mod nx;
pub mod pie;
pub mod relro;
pub mod rpath;
pub mod stripped;

pub use canary::check_canary;
pub use fortify::check_fortify;
pub use needed_libs::get_needed_libraries;
pub use nx::is_nx;
pub use pie::is_pie;
pub use relro::check_relro;
pub use rpath::check_rpaths;
pub use stripped::check_stripped;

#[derive(Debug, PartialEq)]
pub enum CheckStatus {
    Enabled,
    Disabled,
    Unknown(String), // carries an explanation of why it's unknown
}

#[derive(Debug, PartialEq)]
pub enum RelroStatus {
    None,
    Partial,
    Full,
}

#[derive(Debug)]
pub struct FortifyStatus {
    pub enabled: bool,
    pub fortified: usize,
    pub fortifiable: usize,
}
#[derive(Debug)]
pub struct RPathStatus {
    pub rpath: Option<String>,
    pub runpath: Option<String>,
}

#[derive(Debug)]
pub struct NeededLibraries {
    pub libraries: Vec<String>,
}

// Implement Display so we can println!("{}", status) directly.
impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Enabled => write!(f, "ENABLED"),
            CheckStatus::Disabled => write!(f, "DISABLED"),
            CheckStatus::Unknown(msg) => write!(f, "UNKNOWN ({})", msg),
        }
    }
}

impl std::fmt::Display for RelroStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelroStatus::None => write!(f, "None"),
            RelroStatus::Partial => write!(f, "Partial"),
            RelroStatus::Full => write!(f, "Full"),
        }
    }
}
//display status for fortify
impl std::fmt::Display for FortifyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.enabled {
            write!(f, "ENABLED ({}/{})", self.fortified, self.fortifiable)
        } else {
            write!(f, "DISABLED (0/{})", self.fortifiable)
        }
    }
}

impl std::fmt::Display for NeededLibraries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.libraries.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", self.libraries.join(", "))
        }
    }
}
