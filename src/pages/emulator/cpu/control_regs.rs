#[derive(Clone, PartialEq)]
pub struct ControlRegisters {
    pub cr0: u64,  // Control Register 0 (PE, MP, EM, TS, ET, NE, WP, AM, NW, CD, PG)
    pub cr2: u64,  // Page Fault Linear Address
    pub cr3: u64,  // Page Directory Base Register
    pub cr4: u64,  // Control Register 4 (VME, PVI, TSD, DE, PSE, PAE, MCE, PGE, PCIDE, OSFXSR, OSXMMEXCPT, UMIP, etc.)
    pub cr8: u64,  // Task Priority Register (TPR)
}

impl ControlRegisters {
    pub fn new() -> Self {
        Self {
            cr0: 0x80000001, // PE=1, PG=1, WP=1 (standard protected mode + paging)
            cr2: 0,
            cr3: 0,
            cr4: 0x2000,     // OSFXSR=1, OSXMMEXCPT=1 (enable SSE)
            cr8: 0,
        }
    }
    
    pub fn is_protected_mode(&self) -> bool {
        self.cr0 & 1 != 0
    }
    
    pub fn is_paging_enabled(&self) -> bool {
        self.cr0 & 0x80000000 != 0
    }
    
    pub fn is_write_protect(&self) -> bool {
        self.cr0 & 0x10000 != 0
    }
    
    pub fn is_pae_enabled(&self) -> bool {
        self.cr4 & 0x20 != 0
    }
    
    pub fn set_cr0(&mut self, value: u64) {
        // Only allow modifying certain bits
        let mask = 0x80050033; // PG, WP, NE, ET, TS, EM, MP, PE
        self.cr0 = (self.cr0 & !mask) | (value & mask);
    }
    
    pub fn set_cr4(&mut self, value: u64) {
        let mask = 0x40F0; // PAE, PSE, DE, TSD, PVI, VME
        self.cr4 = (self.cr4 & !mask) | (value & mask);
    }
}

impl Default for ControlRegisters {
    fn default() -> Self {
        Self::new()
    }
}