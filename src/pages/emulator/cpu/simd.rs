use std::ops::{Deref, DerefMut};

use iced_x86::Register;

#[derive(Clone, PartialEq)]
pub struct Zmm([u8; 64]);

impl Default for Zmm {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

impl Deref for Zmm {
    type Target = [u8; 64];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Zmm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, PartialEq)]
pub struct Ymm([u8; 32]);

impl Default for Ymm {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl Deref for Ymm {
    type Target = [u8; 32];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ymm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, PartialEq)]
pub struct Xmm([u8; 16]);

impl Default for Xmm {
    fn default() -> Self {
        Self([0u8; 16])
    }
}

impl Deref for Xmm {
    type Target = [u8; 16];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Xmm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct SimdRegisters {
    pub xmm0: Xmm,
    pub xmm1: Xmm,
    pub xmm2: Xmm,
    pub xmm3: Xmm,
    pub xmm4: Xmm,
    pub xmm5: Xmm,
    pub xmm6: Xmm,
    pub xmm7: Xmm,
    pub xmm8: Xmm,
    pub xmm9: Xmm,
    pub xmm10: Xmm,
    pub xmm11: Xmm,
    pub xmm12: Xmm,
    pub xmm13: Xmm,
    pub xmm14: Xmm,
    pub xmm15: Xmm,
    pub ymm0: Ymm,
    pub ymm1: Ymm,
    pub ymm2: Ymm,
    pub ymm3: Ymm,
    pub ymm4: Ymm,
    pub ymm5: Ymm,
    pub ymm6: Ymm,
    pub ymm7: Ymm,
    pub ymm8: Ymm,
    pub ymm9: Ymm,
    pub ymm10: Ymm,
    pub ymm11: Ymm,
    pub ymm12: Ymm,
    pub ymm13: Ymm,
    pub ymm14: Ymm,
    pub ymm15: Ymm,
    pub zmm0: Zmm,
    pub zmm1: Zmm,
    pub zmm2: Zmm,
    pub zmm3: Zmm,
    pub zmm4: Zmm,
    pub zmm5: Zmm,
    pub zmm6: Zmm,
    pub zmm7: Zmm,
    pub zmm8: Zmm,
    pub zmm9: Zmm,
    pub zmm10: Zmm,
    pub zmm11: Zmm,
    pub zmm12: Zmm,
    pub zmm13: Zmm,
    pub zmm14: Zmm,
    pub zmm15: Zmm,
}


impl SimdRegisters {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn read_xmm(&self, reg: Register) -> &[u8; 16] {
        match reg {
            Register::XMM0 => &self.xmm0,
            Register::XMM1 => &self.xmm1,
            Register::XMM2 => &self.xmm2,
            Register::XMM3 => &self.xmm3,
            Register::XMM4 => &self.xmm4,
            Register::XMM5 => &self.xmm5,
            Register::XMM6 => &self.xmm6,
            Register::XMM7 => &self.xmm7,
            Register::XMM8 => &self.xmm8,
            Register::XMM9 => &self.xmm9,
            Register::XMM10 => &self.xmm10,
            Register::XMM11 => &self.xmm11,
            Register::XMM12 => &self.xmm12,
            Register::XMM13 => &self.xmm13,
            Register::XMM14 => &self.xmm14,
            Register::XMM15 => &self.xmm15,
            _ => &self.xmm0,
        }
    }
    
    pub fn write_xmm(&mut self, reg: Register, value: [u8; 16]) {
        match reg {
            Register::XMM0 => self.xmm0 = Xmm(value),
            Register::XMM1 => self.xmm1 = Xmm(value),
            Register::XMM2 => self.xmm2 = Xmm(value),
            Register::XMM3 => self.xmm3 = Xmm(value),
            Register::XMM4 => self.xmm4 = Xmm(value),
            Register::XMM5 => self.xmm5 = Xmm(value),
            Register::XMM6 => self.xmm6 = Xmm(value),
            Register::XMM7 => self.xmm7 = Xmm(value),
            Register::XMM8 => self.xmm8 = Xmm(value),
            Register::XMM9 => self.xmm9 = Xmm(value),
            Register::XMM10 => self.xmm10 = Xmm(value),
            Register::XMM11 => self.xmm11 = Xmm(value),
            Register::XMM12 => self.xmm12 = Xmm(value),
            Register::XMM13 => self.xmm13 = Xmm(value),
            Register::XMM14 => self.xmm14 = Xmm(value),
            Register::XMM15 => self.xmm15 = Xmm(value),
            _ => {}
        }
    }
    
    pub fn read_ymm(&self, reg: Register) -> &[u8; 32] {
        match reg {
            Register::YMM0 => &self.ymm0,
            Register::YMM1 => &self.ymm1,
            Register::YMM2 => &self.ymm2,
            Register::YMM3 => &self.ymm3,
            Register::YMM4 => &self.ymm4,
            Register::YMM5 => &self.ymm5,
            Register::YMM6 => &self.ymm6,
            Register::YMM7 => &self.ymm7,
            Register::YMM8 => &self.ymm8,
            Register::YMM9 => &self.ymm9,
            Register::YMM10 => &self.ymm10,
            Register::YMM11 => &self.ymm11,
            Register::YMM12 => &self.ymm12,
            Register::YMM13 => &self.ymm13,
            Register::YMM14 => &self.ymm14,
            Register::YMM15 => &self.ymm15,
            _ => &self.ymm0,
        }
    }
    
    pub fn write_ymm(&mut self, reg: Register, value: [u8; 32]) {
        match reg {
            Register::YMM0 => self.ymm0 = Ymm(value),
            Register::YMM1 => self.ymm1 = Ymm(value),
            Register::YMM2 => self.ymm2 = Ymm(value),
            Register::YMM3 => self.ymm3 = Ymm(value),
            Register::YMM4 => self.ymm4 = Ymm(value),
            Register::YMM5 => self.ymm5 = Ymm(value),
            Register::YMM6 => self.ymm6 = Ymm(value),
            Register::YMM7 => self.ymm7 = Ymm(value),
            Register::YMM8 => self.ymm8 = Ymm(value),
            Register::YMM9 => self.ymm9 = Ymm(value),
            Register::YMM10 => self.ymm10 = Ymm(value),
            Register::YMM11 => self.ymm11 = Ymm(value),
            Register::YMM12 => self.ymm12 = Ymm(value),
            Register::YMM13 => self.ymm13 = Ymm(value),
            Register::YMM14 => self.ymm14 = Ymm(value),
            Register::YMM15 => self.ymm15 = Ymm(value),
            _ => {}
        }
    }
    
    pub fn read_zmm(&self, reg: Register) -> &[u8; 64] {
        match reg {
            Register::ZMM0 => &self.zmm0,
            Register::ZMM1 => &self.zmm1,
            Register::ZMM2 => &self.zmm2,
            Register::ZMM3 => &self.zmm3,
            Register::ZMM4 => &self.zmm4,
            Register::ZMM5 => &self.zmm5,
            Register::ZMM6 => &self.zmm6,
            Register::ZMM7 => &self.zmm7,
            Register::ZMM8 => &self.zmm8,
            Register::ZMM9 => &self.zmm9,
            Register::ZMM10 => &self.zmm10,
            Register::ZMM11 => &self.zmm11,
            Register::ZMM12 => &self.zmm12,
            Register::ZMM13 => &self.zmm13,
            Register::ZMM14 => &self.zmm14,
            Register::ZMM15 => &self.zmm15,
            _ => &self.zmm0,
        }
    }
    
    pub fn write_zmm(&mut self, reg: Register, value: [u8; 64]) {
        match reg {
            Register::ZMM0 => self.zmm0 = Zmm(value),
            Register::ZMM1 => self.zmm1 = Zmm(value),
            Register::ZMM2 => self.zmm2 = Zmm(value),
            Register::ZMM3 => self.zmm3 = Zmm(value),
            Register::ZMM4 => self.zmm4 = Zmm(value),
            Register::ZMM5 => self.zmm5 = Zmm(value),
            Register::ZMM6 => self.zmm6 = Zmm(value),
            Register::ZMM7 => self.zmm7 = Zmm(value),
            Register::ZMM8 => self.zmm8 = Zmm(value),
            Register::ZMM9 => self.zmm9 = Zmm(value),
            Register::ZMM10 => self.zmm10 = Zmm(value),
            Register::ZMM11 => self.zmm11 = Zmm(value),
            Register::ZMM12 => self.zmm12 = Zmm(value),
            Register::ZMM13 => self.zmm13 = Zmm(value),
            Register::ZMM14 => self.zmm14 = Zmm(value),
            Register::ZMM15 => self.zmm15 = Zmm(value),
            _ => {}
        }
    }
    
    pub fn read_u8(&self, reg: Register, byte_index: usize) -> u8 {
        match reg.size() {
            16 => self.read_xmm(reg)[byte_index],
            32 => self.read_ymm(reg)[byte_index],
            64 => self.read_zmm(reg)[byte_index],
            _ => 0,
        }
    }
    
    pub fn write_u8(&mut self, reg: Register, byte_index: usize, value: u8) {
        match reg.size() {
            16 => {
                let mut data = *self.read_xmm(reg);
                data[byte_index] = value;
                self.write_xmm(reg, data);
            }
            32 => {
                let mut data = *self.read_ymm(reg);
                data[byte_index] = value;
                self.write_ymm(reg, data);
            }
            64 => {
                let mut data = *self.read_zmm(reg);
                data[byte_index] = value;
                self.write_zmm(reg, data);
            }
            _ => {}
        }
    }
    
    pub fn read_u32(&self, reg: Register, dword_index: usize) -> u32 {
        let start = dword_index * 4;
        
        match reg.size() {
            16 => {
                let bytes = self.read_xmm(reg);
                u32::from_le_bytes([bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3]])
            }
            32 => {
                let bytes = self.read_ymm(reg);
                u32::from_le_bytes([bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3]])
            }
            64 => {
                let bytes = self.read_zmm(reg);
                u32::from_le_bytes([bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3]])
            }
            _ => 0,
        }
    }
    
    pub fn write_u32(&mut self, reg: Register, dword_index: usize, value: u32) {
        let bytes = value.to_le_bytes();
        
        match reg.size() {
            16 => {
                let mut data = *self.read_xmm(reg);
                let start = dword_index * 4;
                data[start] = bytes[0];
                data[start + 1] = bytes[1];
                data[start + 2] = bytes[2];
                data[start + 3] = bytes[3];
                self.write_xmm(reg, data);
            }
            32 => {
                let mut data = *self.read_ymm(reg);
                let start = dword_index * 4;
                data[start] = bytes[0];
                data[start + 1] = bytes[1];
                data[start + 2] = bytes[2];
                data[start + 3] = bytes[3];
                self.write_ymm(reg, data);
            }
            64 => {
                let mut data = *self.read_zmm(reg);
                let start = dword_index * 4;
                data[start] = bytes[0];
                data[start + 1] = bytes[1];
                data[start + 2] = bytes[2];
                data[start + 3] = bytes[3];
                self.write_zmm(reg, data);
            }
            _ => {}
        }
    }
    
    pub fn read_u64(&self, reg: Register, qword_index: usize) -> u64 {
        let start = qword_index * 8;
        
        match reg.size() {
            16 => {
                let bytes = self.read_xmm(reg);
                u64::from_le_bytes([
                    bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3],
                    bytes[start + 4], bytes[start + 5], bytes[start + 6], bytes[start + 7],
                ])
            }
            32 => {
                let bytes = self.read_ymm(reg);
                u64::from_le_bytes([
                    bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3],
                    bytes[start + 4], bytes[start + 5], bytes[start + 6], bytes[start + 7],
                ])
            }
            64 => {
                let bytes = self.read_zmm(reg);
                u64::from_le_bytes([
                    bytes[start], bytes[start + 1], bytes[start + 2], bytes[start + 3],
                    bytes[start + 4], bytes[start + 5], bytes[start + 6], bytes[start + 7],
                ])
            }
            _ => 0,
        }
    }
    
    pub fn write_u64(&mut self, reg: Register, qword_index: usize, value: u64) {
        let bytes = value.to_le_bytes();
        
        match reg.size() {
            16 => {
                let mut data = *self.read_xmm(reg);
                let start = qword_index * 8;
                data[start] = bytes[0];
                data[start + 1] = bytes[1];
                data[start + 2] = bytes[2];
                data[start + 3] = bytes[3];
                data[start + 4] = bytes[4];
                data[start + 5] = bytes[5];
                data[start + 6] = bytes[6];
                data[start + 7] = bytes[7];
                self.write_xmm(reg, data);
            }
            32 => {
                let mut data = *self.read_ymm(reg);
                let start = qword_index * 8;
                data[start] = bytes[0];
                data[start + 1] = bytes[1];
                data[start + 2] = bytes[2];
                data[start + 3] = bytes[3];
                data[start + 4] = bytes[4];
                data[start + 5] = bytes[5];
                data[start + 6] = bytes[6];
                data[start + 7] = bytes[7];
                self.write_ymm(reg, data);
            }
            64 => {
                let mut data = *self.read_zmm(reg);
                let start = qword_index * 8;
                data[start] = bytes[0];
                data[start + 1] = bytes[1];
                data[start + 2] = bytes[2];
                data[start + 3] = bytes[3];
                data[start + 4] = bytes[4];
                data[start + 5] = bytes[5];
                data[start + 6] = bytes[6];
                data[start + 7] = bytes[7];
                self.write_zmm(reg, data);
            }
            _ => {}
        }
    }
    
    pub fn read_u128(&self, reg: Register) -> u128 {
        let bytes = self.read_xmm(reg);
        u128::from_le_bytes(*bytes)
    }
    
    pub fn write_u128(&mut self, reg: Register, value: u128) {
        self.write_xmm(reg, value.to_le_bytes());
    }
}