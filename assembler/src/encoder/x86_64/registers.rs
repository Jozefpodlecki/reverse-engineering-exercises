use crate::Register;

#[derive(Debug, Clone, Copy)]
pub struct RegInfo {
    pub code: u8,
    pub needs_rex: bool,
    pub rex_r: bool,
}

impl RegInfo {
    pub fn from_reg(reg: &Register) -> Self {
        match reg {
            // 64-bit
            Register::RAX => Self { code: 0, needs_rex: false, rex_r: false },
            Register::RCX => Self { code: 1, needs_rex: false, rex_r: false },
            Register::RDX => Self { code: 2, needs_rex: false, rex_r: false },
            Register::RBX => Self { code: 3, needs_rex: false, rex_r: false },
            Register::RSP => Self { code: 4, needs_rex: false, rex_r: false },
            Register::RBP => Self { code: 5, needs_rex: false, rex_r: false },
            Register::RSI => Self { code: 6, needs_rex: false, rex_r: false },
            Register::RDI => Self { code: 7, needs_rex: false, rex_r: false },
            
            // Extended 64-bit
            Register::R8 => Self { code: 0, needs_rex: true, rex_r: true },
            Register::R9 => Self { code: 1, needs_rex: true, rex_r: true },
            Register::R10 => Self { code: 2, needs_rex: true, rex_r: true },
            Register::R11 => Self { code: 3, needs_rex: true, rex_r: true },
            Register::R12 => Self { code: 4, needs_rex: true, rex_r: true },
            Register::R13 => Self { code: 5, needs_rex: true, rex_r: true },
            Register::R14 => Self { code: 6, needs_rex: true, rex_r: true },
            Register::R15 => Self { code: 7, needs_rex: true, rex_r: true },
            
            // 32-bit (same codes, no REX)
            Register::EAX => Self { code: 0, needs_rex: false, rex_r: false },
            Register::ECX => Self { code: 1, needs_rex: false, rex_r: false },
            Register::EDX => Self { code: 2, needs_rex: false, rex_r: false },
            Register::EBX => Self { code: 3, needs_rex: false, rex_r: false },
            Register::ESP => Self { code: 4, needs_rex: false, rex_r: false },
            Register::EBP => Self { code: 5, needs_rex: false, rex_r: false },
            Register::ESI => Self { code: 6, needs_rex: false, rex_r: false },
            Register::EDI => Self { code: 7, needs_rex: false, rex_r: false },
            
            // 16-bit
            Register::AX => Self { code: 0, needs_rex: false, rex_r: false },
            Register::CX => Self { code: 1, needs_rex: false, rex_r: false },
            Register::DX => Self { code: 2, needs_rex: false, rex_r: false },
            Register::BX => Self { code: 3, needs_rex: false, rex_r: false },
            Register::SP => Self { code: 4, needs_rex: false, rex_r: false },
            Register::BP => Self { code: 5, needs_rex: false, rex_r: false },
            Register::SI => Self { code: 6, needs_rex: false, rex_r: false },
            Register::DI => Self { code: 7, needs_rex: false, rex_r: false },
            
            // 8-bit
            Register::AL => Self { code: 0, needs_rex: false, rex_r: false },
            Register::CL => Self { code: 1, needs_rex: false, rex_r: false },
            Register::DL => Self { code: 2, needs_rex: false, rex_r: false },
            Register::BL => Self { code: 3, needs_rex: false, rex_r: false },
            Register::SPL => Self { code: 4, needs_rex: false, rex_r: false },
            Register::BPL => Self { code: 5, needs_rex: false, rex_r: false },
            Register::SIL => Self { code: 6, needs_rex: false, rex_r: false },
            Register::DIL => Self { code: 7, needs_rex: false, rex_r: false },
            
            // SIMD - codes 0-15, may need REX for 8-15
            Register::XMM0 | Register::YMM0 | Register::ZMM0 => Self { code: 0, needs_rex: false, rex_r: false },
            Register::XMM1 | Register::YMM1 | Register::ZMM1 => Self { code: 1, needs_rex: false, rex_r: false },
            Register::XMM2 | Register::YMM2 | Register::ZMM2 => Self { code: 2, needs_rex: false, rex_r: false },
            Register::XMM3 | Register::YMM3 | Register::ZMM3 => Self { code: 3, needs_rex: false, rex_r: false },
            Register::XMM4 | Register::YMM4 | Register::ZMM4 => Self { code: 4, needs_rex: false, rex_r: false },
            Register::XMM5 | Register::YMM5 | Register::ZMM5 => Self { code: 5, needs_rex: false, rex_r: false },
            Register::XMM6 | Register::YMM6 | Register::ZMM6 => Self { code: 6, needs_rex: false, rex_r: false },
            Register::XMM7 | Register::YMM7 | Register::ZMM7 => Self { code: 7, needs_rex: false, rex_r: false },
            Register::XMM8 | Register::YMM8 | Register::ZMM8 => Self { code: 8, needs_rex: true, rex_r: true },
            Register::XMM9 | Register::YMM9 | Register::ZMM9 => Self { code: 9, needs_rex: true, rex_r: true },
            Register::XMM10 | Register::YMM10 | Register::ZMM10 => Self { code: 10, needs_rex: true, rex_r: true },
            Register::XMM11 | Register::YMM11 | Register::ZMM11 => Self { code: 11, needs_rex: true, rex_r: true },
            Register::XMM12 | Register::YMM12 | Register::ZMM12 => Self { code: 12, needs_rex: true, rex_r: true },
            Register::XMM13 | Register::YMM13 | Register::ZMM13 => Self { code: 13, needs_rex: true, rex_r: true },
            Register::XMM14 | Register::YMM14 | Register::ZMM14 => Self { code: 14, needs_rex: true, rex_r: true },
            Register::XMM15 | Register::YMM15 | Register::ZMM15 => Self { code: 15, needs_rex: true, rex_r: true },
            
            _ => Self { code: 0, needs_rex: false, rex_r: false },
        }
    }
}