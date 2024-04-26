use std::cmp::PartialEq;

use anyhow::bail;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Arch {
    X86, // 32-bit
    X64, // 64-bit
    ARM32, // ARM 32-bit
    ARM64, // ARM 64-bit
}


#[cfg(target_os = "windows")]
const CURRENT_OS: OS = OS::Windows;
#[cfg(target_os = "linux")]
const CURRENT_OS: OS = OS::Linux;
#[cfg(target_os = "macos")]
const CURRENT_OS: OS = OS::MacOS;

#[cfg(target_arch = "x86")]
const CURRENT_ARCH: Arch = Arch::X86;
#[cfg(target_arch = "x86_64")]
const CURRENT_ARCH: Arch = Arch::X64;
#[cfg(target_arch = "aarch32")]
const CURRENT_ARCH: Arch = Arch::ARM32;
#[cfg(target_arch = "aarch64")]
const CURRENT_ARCH: Arch = Arch::ARM64;

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

    pub fn architecture(&self) -> Arch {
        CURRENT_ARCH.clone()
    }

    pub fn get_os_string(&self, mono: bool) -> anyhow::Result<&str> {
        match (self, self.architecture(), mono) {
            (OS::Windows, Arch::X86, false) => Ok("win32"),
            (OS::Windows, Arch::X86, true) => Ok("mono_win32"),
            (OS::Windows, Arch::X64, false) => Ok("win64"),
            (OS::Windows, Arch::X64, true) => Ok("mono_win64"),
            (OS::Windows, Arch::ARM32, _) => bail!("ARM32 is not supported on Windows"),
            (OS::Windows, Arch::ARM64, _) => bail!("ARM64 is not supported on Windows"),
            (OS::Linux, Arch::X86, false) => Ok("linux.x86_32"),
            (OS::Linux, Arch::X86, true) => Ok("mono_linux_x86_32"),
            (OS::Linux, Arch::X64, false) => Ok("linux.x86_64"),
            (OS::Linux, Arch::X64, true) => Ok("mono_linux_x86_64"),
            (OS::Linux, Arch::ARM32, false) => Ok("linux.arm32"),
            (OS::Linux, Arch::ARM32, true) => Ok("mono_linux_arm32"),
            (OS::Linux, Arch::ARM64, false) => Ok("linux.arm64"),
            (OS::Linux, Arch::ARM64, true) => Ok("mono_linux_arm64"),
            (OS::MacOS, _, false) => Ok("macos.universal"),
            (OS::MacOS, _, true) => Ok("mono_macos.universal"),
        }
    }
}