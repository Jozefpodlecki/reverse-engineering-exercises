#[derive(Debug, Clone, Copy)]
pub enum Mod {
    Indirect = 0b00,
    Disp8 = 0b01,
    Disp32 = 0b10,
    Reg = 0b11,
}

#[derive(Debug, Clone, Copy)]
pub struct ModRM {
    pub r#mod: Mod,
    pub reg: u8,  // opcode extension or register
    pub rm: u8,   // register or memory
}

impl ModRM {
    pub fn new(r#mod: Mod, reg: u8, rm: u8) -> Self {
        Self { r#mod, reg, rm }
    }
    
    pub fn encode(&self) -> u8 {
        ((self.r#mod as u8) << 6) | ((self.reg & 0x7) << 3) | (self.rm & 0x7)
    }
    
    pub fn reg_reg(reg_dest: u8, reg_src: u8) -> Self {
        Self::new(Mod::Reg, reg_src, reg_dest)
    }
    
    pub fn mem(rm: u8, disp_size: DispSize) -> Self {
        let r#mod = match disp_size {
            DispSize::None => Mod::Indirect,
            DispSize::Byte => Mod::Disp8,
            DispSize::Dword => Mod::Disp32,
        };
        Self::new(r#mod, 0, rm)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DispSize {
    None,
    Byte,
    Dword,
}