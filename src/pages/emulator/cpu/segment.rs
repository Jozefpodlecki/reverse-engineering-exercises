#[derive(Clone, PartialEq)]
pub struct SegmentRegisters {
    pub cs: Segment,
    pub ds: Segment,
    pub ss: Segment,
    pub es: Segment,
    pub fs: Segment,
    pub gs: Segment,
}

#[derive(Clone, PartialEq)]
pub struct Segment {
    pub selector: u16,
    pub base: u64,
    pub limit: u64,
    pub flags: SegmentFlags,
}

#[derive(Clone, PartialEq)]
pub struct SegmentFlags {
    pub present: bool,
    pub dpl: u8,           // Descriptor privilege level (0-3)
    pub long_mode: bool,   // L bit (64-bit code segment)
    pub default_size: bool, // D/B bit (32-bit vs 16-bit)
    pub granularity: bool, // G bit (4KB granularity vs byte)
    pub rw: bool,          // Readable (code) / Writable (data)
    pub accessed: bool,
    pub executable: bool,   // Code vs data segment
    pub expand_down: bool,  // For data segments
    pub conforming: bool,   // For code segments
}

impl Segment {
    pub fn null() -> Self {
        Self {
            selector: 0,
            base: 0,
            limit: 0,
            flags: SegmentFlags {
                present: false,
                dpl: 0,
                long_mode: false,
                default_size: false,
                granularity: false,
                rw: false,
                accessed: false,
                executable: false,
                expand_down: false,
                conforming: false,
            },
        }
    }
    
    pub fn code_segment(selector: u16, dpl: u8, long_mode: bool) -> Self {
        Self {
            selector,
            base: 0,
            limit: 0xFFFFFFFFF,
            flags: SegmentFlags {
                present: true,
                dpl,
                long_mode,
                default_size: !long_mode,
                granularity: true,
                rw: true,  // readable
                accessed: false,
                executable: true,
                expand_down: false,
                conforming: false,
            },
        }
    }
    
    pub fn data_segment(selector: u16, dpl: u8, writable: bool) -> Self {
        Self {
            selector,
            base: 0,
            limit: 0xFFFFFFFFF,
            flags: SegmentFlags {
                present: true,
                dpl,
                long_mode: false,
                default_size: true,
                granularity: true,
                rw: writable,
                accessed: false,
                executable: false,
                expand_down: false,
                conforming: false,
            },
        }
    }
    
    pub fn fs_segment(base: u64) -> Self {
        Self {
            selector: 0,
            base,
            limit: 0xFFFFFFFFF,
            flags: SegmentFlags {
                present: true,
                dpl: 0,
                long_mode: false,
                default_size: true,
                granularity: true,
                rw: true,
                accessed: false,
                executable: false,
                expand_down: false,
                conforming: false,
            },
        }
    }
    
    pub fn gs_segment(base: u64) -> Self {
        Self {
            selector: 0,
            base,
            limit: 0xFFFFFFFFF,
            flags: SegmentFlags {
                present: true,
                dpl: 0,
                long_mode: false,
                default_size: true,
                granularity: true,
                rw: true,
                accessed: false,
                executable: false,
                expand_down: false,
                conforming: false,
            },
        }
    }
    
    pub fn check_access(&self, address: u64) -> bool {
        if !self.flags.present {
            return false;
        }
        
        let limit = if self.flags.granularity {
            (self.limit as u64) * 4096 + 4095
        } else {
            self.limit as u64
        };
        
        address >= self.base && address <= self.base + limit
    }
}

impl SegmentRegisters {
    pub fn new() -> Self {
        Self {
            cs: Segment::code_segment(0x33, 0, true),   // 64-bit code segment
            ds: Segment::data_segment(0x2B, 0, true),   // 64-bit data segment (writable)
            ss: Segment::data_segment(0x2B, 0, true),   // 64-bit stack segment
            es: Segment::data_segment(0x2B, 0, true),   // Extra segment
            fs: Segment::null(),                         // Set via WRFSBASE or MSR
            gs: Segment::null(),                         // Set via WRGSBASE or MSR
        }
    }
    
    pub fn read(&self, reg: iced_x86::Register) -> u16 {
        match reg {
            iced_x86::Register::CS => self.cs.selector,
            iced_x86::Register::DS => self.ds.selector,
            iced_x86::Register::SS => self.ss.selector,
            iced_x86::Register::ES => self.es.selector,
            iced_x86::Register::FS => self.fs.selector,
            iced_x86::Register::GS => self.gs.selector,
            _ => 0,
        }
    }
    
    pub fn write(&mut self, reg: iced_x86::Register, selector: u16) {
        match reg {
            iced_x86::Register::CS => self.cs.selector = selector,
            iced_x86::Register::DS => self.ds.selector = selector,
            iced_x86::Register::SS => self.ss.selector = selector,
            iced_x86::Register::ES => self.es.selector = selector,
            iced_x86::Register::FS => self.fs.selector = selector,
            iced_x86::Register::GS => self.gs.selector = selector,
            _ => {}
        }
    }
    
    pub fn get_base(&self, reg: iced_x86::Register) -> u64 {
        match reg {
            iced_x86::Register::FS => self.fs.base,
            iced_x86::Register::GS => self.gs.base,
            iced_x86::Register::CS => self.cs.base,
            iced_x86::Register::DS => self.ds.base,
            iced_x86::Register::SS => self.ss.base,
            iced_x86::Register::ES => self.es.base,
            _ => 0,
        }
    }
    
    pub fn set_fs_base(&mut self, base: u64) {
        self.fs = Segment::fs_segment(base);
    }
    
    pub fn set_gs_base(&mut self, base: u64) {
        self.gs = Segment::gs_segment(base);
    }
}

impl Default for SegmentRegisters {
    fn default() -> Self {
        Self::new()
    }
}