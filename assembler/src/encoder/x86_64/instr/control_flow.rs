use std::collections::HashMap;

use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::ast::ConditionCode;
use crate::Spanned;

pub fn jmp(target: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &target.value {
        Operand::Label(_) => {
            let mut buf = InstrBuf::new();
            buf.push(0xE9).push_u32(0);
            Ok(buf)
        }
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().b(info.needs_rex);
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            buf.push(0xE0 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().b(base_info.needs_rex);
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            
            let (mod_val, disp_size) = if mem.displacement == 0 {
                (Mod::Indirect, None)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, Some(mem.displacement as u8))
            } else {
                (Mod::Disp32, None)
            };
            
            let modrm = ModRM::new(mod_val, 0x04, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if let Some(disp) = disp_size {
                buf.push(disp);
            }
            
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(target.value.clone())),
    }
}

pub fn jmp_with_label(target: &str, symbols: &HashMap<String, usize>, offset: u64) -> InstrBuf {
    let mut buf = InstrBuf::new();
    buf.push(0xE9);
    if let Some(&addr) = symbols.get(target) {
        let rel = (addr as i64) - (offset as i64 + 5);
        buf.push_u32(rel as u32);
    } else {
        buf.push_u32(0);
    }
    buf
}

pub fn call(target: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &target.value {
        Operand::Label(_) => {
            let mut buf = InstrBuf::new();
            buf.push(0xE8).push_u32(0);
            Ok(buf)
        }
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().b(info.needs_rex);
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            buf.push(0xD0 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().b(base_info.needs_rex);
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            
            let modrm = if mem.displacement == 0 {
                ModRM::new(Mod::Indirect, 0x02, base_info.code & 0x7)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                ModRM::new(Mod::Disp8, 0x02, base_info.code & 0x7)
            } else {
                ModRM::new(Mod::Disp32, 0x02, base_info.code & 0x7)
            };
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if mem.displacement != 0 {
                buf.push(mem.displacement as u8);
            }
            
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(target.value.clone())),
    }
}

pub fn jcc(cc: &ConditionCode, target: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    let opcode = match cc {
        ConditionCode::E | ConditionCode::Z => 0x84,
        ConditionCode::NE | ConditionCode::NZ => 0x85,
        ConditionCode::G => 0x8F,
        ConditionCode::GE => 0x8D,
        ConditionCode::L => 0x8C,
        ConditionCode::LE => 0x8E,
        ConditionCode::A => 0x87,
        ConditionCode::AE => 0x83,
        ConditionCode::B => 0x82,
        ConditionCode::BE => 0x86,
        _ => 0x84,
    };
    let mut buf = InstrBuf::new();
    buf.push(0x0F).push(opcode).push_u32(0);
    Ok(buf)
}