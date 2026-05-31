#[derive(Clone, PartialEq)]
pub struct DebugRegisters {
    pub dr0: u64,  // Breakpoint address 0
    pub dr1: u64,  // Breakpoint address 1
    pub dr2: u64,  // Breakpoint address 2
    pub dr3: u64,  // Breakpoint address 3
    pub dr6: u64,  // Debug status
    pub dr7: u64,  // Debug control
}

impl DebugRegisters {
    pub fn new() -> Self {
        Self {
            dr0: 0,
            dr1: 0,
            dr2: 0,
            dr3: 0,
            dr6: 0xFFFF0FF0, // All breakpoints inactive, B0-B3 = 0
            dr7: 0x400,      // GD = 0, LE/GE = 0, others 0
        }
    }
    
    pub fn read(&self, dr: u8) -> Result<u64, ()> {
        match dr {
            0 => Ok(self.dr0),
            1 => Ok(self.dr1),
            2 => Ok(self.dr2),
            3 => Ok(self.dr3),
            6 => Ok(self.dr6),
            7 => Ok(self.dr7),
            _ => Err(()), // #UD exception
        }
    }
    
    pub fn write(&mut self, dr: u8, value: u64) -> Result<(), ()> {
        match dr {
            0 => { self.dr0 = value; Ok(()) }
            1 => { self.dr1 = value; Ok(()) }
            2 => { self.dr2 = value; Ok(()) }
            3 => { self.dr3 = value; Ok(()) }
            6 => { self.dr6 = (self.dr6 & 0xFFFF0FF0) | (value & 0xFFFF0FF0); Ok(()) }
            7 => { self.dr7 = value & 0xFFFF7FFF; Ok(()) }
            _ => Err(()),
        }
    }
    
    pub fn is_breakpoint_enabled(&self, index: usize) -> bool {
        (self.dr7 >> (index * 2)) & 1 != 0
    }
    
    pub fn get_breakpoint_type(&self, index: usize) -> BreakpointType {
        let bits = (self.dr7 >> (16 + index * 4)) & 0x3;
        match bits {
            0 => BreakpointType::InstructionExecute,
            1 => BreakpointType::DataWrite,
            2 => BreakpointType::IoReadWrite,
            3 => BreakpointType::DataReadWrite,
            _ => BreakpointType::InstructionExecute,
        }
    }
    
    pub fn get_breakpoint_size(&self, index: usize) -> BreakpointSize {
        let bits = (self.dr7 >> (18 + index * 4)) & 0x3;
        match bits {
            0 => BreakpointSize::OneByte,
            1 => BreakpointSize::TwoBytes,
            2 => BreakpointSize::EightBytes,
            3 => BreakpointSize::FourBytes,
            _ => BreakpointSize::OneByte,
        }
    }
    
    pub fn check_breakpoint(&mut self, address: u64, access: BreakpointAccess) -> bool {
        let mut hit = false;
        
        for i in 0..4 {
            if !self.is_breakpoint_enabled(i) {
                continue;
            }
            
            let bp_type = self.get_breakpoint_type(i);
            let bp_size = self.get_breakpoint_size(i);
            let bp_address = match i {
                0 => self.dr0,
                1 => self.dr1,
                2 => self.dr2,
                3 => self.dr3,
                _ => 0,
            };
            
            let size_bytes = match bp_size {
                BreakpointSize::OneByte => 1,
                BreakpointSize::TwoBytes => 2,
                BreakpointSize::FourBytes => 4,
                BreakpointSize::EightBytes => 8,
            };
            
            let in_range = address >= bp_address && address < bp_address + size_bytes;
            
            let type_matches = match (bp_type, access) {
                (BreakpointType::InstructionExecute, BreakpointAccess::Execute) => true,
                (BreakpointType::DataWrite, BreakpointAccess::Write) => true,
                (BreakpointType::DataReadWrite, BreakpointAccess::Read) => true,
                (BreakpointType::DataReadWrite, BreakpointAccess::Write) => true,
                (BreakpointType::IoReadWrite, _) => false, // IO not implemented
                _ => false,
            };
            
            if in_range && type_matches {
                hit = true;
                self.dr6 |= 1 << i; // Set breakpoint status bit
            }
        }
        
        if hit {
            self.dr6 |= 1 << 13; // Set BD (debug exception) bit
        }
        
        hit
    }
    
    pub fn clear_status(&mut self) {
        self.dr6 &= 0xFFFF0FF0;
    }
}

#[derive(PartialEq)]
pub enum BreakpointType {
    InstructionExecute,
    DataWrite,
    IoReadWrite,
    DataReadWrite,
}

#[derive(PartialEq)]
pub enum BreakpointSize {
    OneByte,
    TwoBytes,
    FourBytes,
    EightBytes,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BreakpointAccess {
    Read,
    Write,
    Execute,
}

impl Default for DebugRegisters {
    fn default() -> Self {
        Self::new()
    }
}