#[derive(Debug, Clone, Copy, Default)]
pub struct RexPrefix {
    pub w: bool,  // 64-bit operand size
    pub r: bool,  // extended register (MODRM.reg)
    pub x: bool,  // extended index (SIB.index)
    pub b: bool,  // extended base (MODRM.rm / SIB.base)
}

impl RexPrefix {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn w(mut self, w: bool) -> Self {
        self.w = w;
        self
    }
    
    pub fn r(mut self, r: bool) -> Self {
        self.r = r;
        self
    }
    
    pub fn x(mut self, x: bool) -> Self {
        self.x = x;
        self
    }
    
    pub fn b(mut self, b: bool) -> Self {
        self.b = b;
        self
    }
    
    pub fn encode(&self) -> Option<u8> {
        if !self.w && !self.r && !self.x && !self.b {
            return None;
        }
        Some(0x40 | (self.w as u8) << 3 | (self.r as u8) << 2 | (self.x as u8) << 1 | (self.b as u8))
    }
}