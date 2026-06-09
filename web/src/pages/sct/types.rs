use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Architecture {
    X86,
    X64,
}

impl Architecture {
    pub fn all() -> Vec<Architecture> {
        vec![
            Architecture::X86,
            Architecture::X64,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Architecture::X86 => "x86",
            Architecture::X64 => "x64",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SyscallType {
    Nt,
    Win32k,
}

impl SyscallType {
    pub fn all() -> Vec<SyscallType> {
        vec![
            SyscallType::Nt,
            SyscallType::Win32k,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            SyscallType::Nt => "NT Kernel Calls",
            SyscallType::Win32k => "Win32k GUI Calls",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum WindowsVersion {
    Nt3(u8), // 3.1, 3.5, 3.51
    Nt4(u8), // SP0-SP6, SP3 TSE
    Win2000(u8), // SP0-SP4
    Xp(u8), // SP0-SP3
    Server2003(u8, bool), // SP0-SP2, R2 flag
    Vista(u8), // SP0-SP2
    Win7(u8), // SP0-SP1
    Win8(u8), // 8.0, 8.1
    Win10(u16), // Build numbers 1507-22H2
    Win11(u16), // Build numbers
}

impl WindowsVersion {
    // pub fn all() -> Vec<WindowsVersion> {
    //     let mut versions = Vec::new();
        
    //     versions.push(WindowsVersion::Nt3(31));
    //     versions.push(WindowsVersion::Nt3(35));
    //     versions.push(WindowsVersion::Nt3(351));
        
    //     for sp in 0..=6 {
    //         versions.push(WindowsVersion::Nt4(sp));
    //     }
    //     versions.push(WindowsVersion::Nt4(99));
        
    //     for sp in 0..=4 {
    //         versions.push(WindowsVersion::Win2000(sp));
    //     }
        
    //     for sp in 0..=3 {
    //         versions.push(WindowsVersion::Xp(sp));
    //     }
        
    //     versions.push(WindowsVersion::Server2003(0, false));
    //     versions.push(WindowsVersion::Server2003(1, false));
    //     versions.push(WindowsVersion::Server2003(2, false));
    //     versions.push(WindowsVersion::Server2003(0, true));
    //     versions.push(WindowsVersion::Server2003(2, true));
        
    //     for sp in 0..=2 {
    //         versions.push(WindowsVersion::Vista(sp));
    //     }
        
    //     for sp in 0..=1 {
    //         versions.push(WindowsVersion::Win7(sp));
    //     }
        
    //     versions.push(WindowsVersion::Win8(0));
    //     versions.push(WindowsVersion::Win8(1));
        
    //     // let win10_builds = vec![1507, 1511, 1607, 1703, 1709, 1803, 1809, 1903, 1909, 2004, 20H2, 21H1, 21H2, 22H2];
    //     // for build in win10_builds {
    //     //     versions.push(WindowsVersion::Win10(build));
    //     // }
        
    //     let win11_builds = vec![22000, 22621, 22631, 26100];
    //     for build in win11_builds {
    //         versions.push(WindowsVersion::Win11(build));
    //     }
        
    //     versions
    // }
    
    // pub fn family(&self) -> &'static str {
    //     match self {
    //         WindowsVersion::Nt3(_) => "Windows NT 3.x",
    //         WindowsVersion::Nt4(_) => "Windows NT 4.0",
    //         WindowsVersion::Win2000(_) => "Windows 2000",
    //         WindowsVersion::Xp(_) => "Windows XP",
    //         WindowsVersion::Server2003(_, _) => "Windows Server 2003",
    //         WindowsVersion::Vista(_) => "Windows Vista",
    //         WindowsVersion::Win7(_) => "Windows 7",
    //         WindowsVersion::Win8(_) => "Windows 8",
    //         WindowsVersion::Win10(_) => "Windows 10",
    //         WindowsVersion::Win11(_) => "Windows 11",
    //     }
    // }
    
    // pub fn name(&self) -> String {
    //     match self {
    //         WindowsVersion::Nt3(v) => match v {
    //             31 => "NT 3.1".to_string(),
    //             35 => "NT 3.5".to_string(),
    //             351 => "NT 3.51".to_string(),
    //             _ => format!("NT 3.{}", v),
    //         },
    //         WindowsVersion::Nt4(sp) => match sp {
    //             99 => "NT 4.0 SP3 TSE".to_string(),
    //             _ => format!("NT 4.0 SP{}", sp),
    //         },
    //         WindowsVersion::Win2000(sp) => format!("2000 SP{}", sp),
    //         WindowsVersion::Xp(sp) => format!("XP SP{}", sp),
    //         WindowsVersion::Server2003(sp, r2) => {
    //             let base = format!("Server 2003 SP{}", sp);
    //             if *r2 {
    //                 format!("{} R2", base)
    //             } else {
    //                 base
    //             }
    //         },
    //         WindowsVersion::Vista(sp) => format!("Vista SP{}", sp),
    //         WindowsVersion::Win7(sp) => format!("7 SP{}", sp),
    //         WindowsVersion::Win8(v) => match v {
    //             0 => "8.0".to_string(),
    //             1 => "8.1".to_string(),
    //             _ => format!("8.{}", v),
    //         },
    //         WindowsVersion::Win10(build) => format!("10 ({})", build),
    //         WindowsVersion::Win11(build) => format!("11 ({})", build),
    //     }
    // }
    
    pub fn full_name(&self) -> String {
        match self {
            WindowsVersion::Nt3(v) => format!("Windows NT 3.{}", v),
            WindowsVersion::Nt4(sp) => match sp {
                99 => "Windows NT 4.0 SP3 TSE".to_string(),
                _ => format!("Windows NT 4.0 SP{}", sp),
            },
            WindowsVersion::Win2000(sp) => format!("Windows 2000 SP{}", sp),
            WindowsVersion::Xp(sp) => format!("Windows XP SP{}", sp),
            WindowsVersion::Server2003(sp, r2) => {
                let base = format!("Windows Server 2003 SP{}", sp);
                if *r2 {
                    format!("{} R2", base)
                } else {
                    base
                }
            },
            WindowsVersion::Vista(sp) => format!("Windows Vista SP{}", sp),
            WindowsVersion::Win7(sp) => format!("Windows 7 SP{}", sp),
            WindowsVersion::Win8(v) => match v {
                0 => "Windows 8.0".to_string(),
                1 => "Windows 8.1".to_string(),
                _ => format!("Windows 8.{}", v),
            },
            WindowsVersion::Win10(build) => format!("Windows 10 ({})", build),
            WindowsVersion::Win11(build) => format!("Windows 11 ({})", build),
        }
    }
    
    pub fn hidden(&self) -> bool {
        matches!(self, WindowsVersion::Nt3(_) | WindowsVersion::Nt4(_) | WindowsVersion::Win2000(_))
    }
}

#[derive(Clone, PartialEq)]
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