use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Msr {
    pub efer: u64,      // Extended Feature Enable Register
    pub star: u64,      // Syscall Target Address Register
    pub lstar: u64,     // Long Mode Syscall Target Address Register
    pub cstar: u64,     // Compatible Mode Syscall Target Address Register
    pub sfmask: u64,    // Syscall Flag Mask
    pub fs_base: u64,   // FS Segment Base
    pub gs_base: u64,   // GS Segment Base
    pub kernel_gs_base: u64, // Kernel GS Base
    pub tsc: u64,       // Timestamp Counter
    pub aperf: u64,     // Actual Performance Counter
    pub mperf: u64,     // Maximum Performance Counter
    pub extended: HashMap<u32, u64>, // For other MSRs
}

impl Msr {
    pub fn new() -> Self {
        Self {
            efer: 0,
            star: 0,
            lstar: 0,
            cstar: 0,
            sfmask: 0,
            fs_base: 0,
            gs_base: 0,
            kernel_gs_base: 0,
            tsc: 0,
            aperf: 0,
            mperf: 0,
            extended: HashMap::new(),
        }
    }
    
    pub fn read(&self, msr: u32) -> u64 {
        match msr {
            0xC0000080 => self.efer,      // EFER
            0xC0000081 => self.star,      // STAR
            0xC0000082 => self.lstar,     // LSTAR
            0xC0000083 => self.cstar,     // CSTAR
            0xC0000084 => self.sfmask,    // SFMASK
            0xC0000100 => self.fs_base,   // FS_BASE
            0xC0000101 => self.gs_base,   // GS_BASE
            0xC0000102 => self.kernel_gs_base, // Kernel GS_BASE
            0x10 => self.tsc,             // TSC
            0xE8 => self.aperf,           // APERF
            0xE7 => self.mperf,           // MPERF
            _ => *self.extended.get(&msr).unwrap_or(&0),
        }
    }
    
    pub fn write(&mut self, msr: u32, value: u64) {
        match msr {
            0xC0000080 => self.efer = value,
            0xC0000081 => self.star = value,
            0xC0000082 => self.lstar = value,
            0xC0000083 => self.cstar = value,
            0xC0000084 => self.sfmask = value,
            0xC0000100 => self.fs_base = value,
            0xC0000101 => self.gs_base = value,
            0xC0000102 => self.kernel_gs_base = value,
            0x10 => self.tsc = value,
            0xE8 => self.aperf = value,
            0xE7 => self.mperf = value,
            _ => {
                self.extended.insert(msr, value);
            }
        }
    }
}

impl Default for Msr {
    fn default() -> Self {
        Self::new()
    }
}