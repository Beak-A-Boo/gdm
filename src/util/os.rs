use std::cmp::PartialEq;
use once_cell::sync::Lazy;

#[derive(Clone, Debug, PartialEq)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
    UNKNOWN(&'static str),
}

const CURRENT_OS: Lazy<OS> = Lazy::new(|| {
    let os = std::env::consts::OS;
    match os {
        "windows" => OS::Windows,
        "linux" => OS::Linux,
        "macos" => OS::MacOS,
        other => OS::UNKNOWN(other),
    }
});

impl Default for OS {

    fn default() -> Self {
        OS::current()
    }
}

impl OS {
    pub fn current() -> OS {
        CURRENT_OS.clone()
    }

    pub fn is_windows(&self) -> bool {
        self == &OS::Windows
    }
}