#[derive(Clone, PartialEq, Eq, Hash)]
pub enum WindowsVersion {
    Xp,
    Vista,
    Win7,
    Win8,
    Win81,
    Win10,
    Win11,
}

impl WindowsVersion {
    pub fn all() -> Vec<WindowsVersion> {
        vec![
            WindowsVersion::Xp,
            WindowsVersion::Vista,
            WindowsVersion::Win7,
            WindowsVersion::Win8,
            WindowsVersion::Win81,
            WindowsVersion::Win10,
            WindowsVersion::Win11,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            WindowsVersion::Xp => "XP",
            WindowsVersion::Vista => "Vista",
            WindowsVersion::Win7 => "7",
            WindowsVersion::Win8 => "8",
            WindowsVersion::Win81 => "8.1",
            WindowsVersion::Win10 => "10",
            WindowsVersion::Win11 => "11",
        }
    }
    
    pub fn full_name(&self) -> &'static str {
        match self {
            WindowsVersion::Xp => "Windows XP",
            WindowsVersion::Vista => "Windows Vista",
            WindowsVersion::Win7 => "Windows 7",
            WindowsVersion::Win8 => "Windows 8",
            WindowsVersion::Win81 => "Windows 8.1",
            WindowsVersion::Win10 => "Windows 10",
            WindowsVersion::Win11 => "Windows 11",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum OsFamily {
    Windows,
    Linux,
    MacOs,
}

impl OsFamily {
    pub fn all() -> Vec<OsFamily> {
        vec![
            OsFamily::Windows,
            OsFamily::Linux,
            OsFamily::MacOs,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            OsFamily::Windows => "Windows",
            OsFamily::Linux => "Linux",
            OsFamily::MacOs => "macOS",
        }
    }
}