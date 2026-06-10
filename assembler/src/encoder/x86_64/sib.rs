#[derive(Debug, Clone, Copy)]
pub struct SIB {
    pub scale: u8,  // 0,1,2,3 (1,2,4,8)
    pub index: u8,  // register code (0-7)
    pub base: u8,   // register code (0-7)
}

impl SIB {
    pub fn new(scale: u8, index: u8, base: u8) -> Self {
        Self { scale, index, base }
    }
    
    pub fn encode(&self) -> u8 {
        ((self.scale & 0x3) << 6) | ((self.index & 0x7) << 3) | (self.base & 0x7)
    }
    
    pub fn for_rsp() -> Self {
        Self::new(0, 0x04, 0x04)
    }
}