use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::ast::ConditionCode;
use crate::Spanned;
use crate::symbol::SymbolResolver;

pub fn shl(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xE0, 4)
}

pub fn shr(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xE8, 5)
}

pub fn sar(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xF8, 7)
}

pub fn rol(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xC0, 0)
}

pub fn ror(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xC8, 1)
}

pub fn rcl(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xD0, 2)
}

pub fn rcr(dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_shift(dest, count, 0xD8, 3)
}

fn encode_shift(
    dest: &Spanned<Operand>,
    count: &Spanned<Operand>,
    base_opcode: u8,
    reg_code: u8,
) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &count.value) {
        (Operand::Register(dest_reg), Operand::Immediate(1)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let rex = RexPrefix::new().w(true).b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xD1);
            buf.push(base_opcode + (dest_info.code & 0x7));
            Ok(buf)
        }
        (Operand::Register(dest_reg), Operand::Immediate(imm)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let rex = RexPrefix::new().w(true).b(dest_info.needs_rex);
            
            if *imm < 0 || *imm > 255 {
                return Err(EncodingError::ImmediateOutOfRange {
                    value: *imm,
                    min: 0,
                    max: 255,
                    size: crate::encoder::ImmediateSize::U8,
                });
            }
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xC1);
            buf.push(base_opcode + (dest_info.code & 0x7));
            buf.push(*imm as u8);
            Ok(buf)
        }
        (Operand::Register(dest_reg), Operand::Register(count_reg)) => {
            if count_reg != &crate::parser::Register::CL {
                return Err(EncodingError::InvalidRegister(count_reg.clone()));
            }
            
            let dest_info = RegInfo::from_reg(dest_reg);
            let rex = RexPrefix::new().w(true).b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xD3);
            buf.push(base_opcode + (dest_info.code & 0x7));
            Ok(buf)
        }
        (Operand::Memory(mem), Operand::Immediate(1)) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xD1);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, reg_code, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        (Operand::Memory(mem), Operand::Immediate(imm)) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            if *imm < 0 || *imm > 255 {
                return Err(EncodingError::ImmediateOutOfRange {
                    value: *imm,
                    min: 0,
                    max: 255,
                    size: crate::encoder::ImmediateSize::U8,
                });
            }
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xC1);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, reg_code, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            buf.push(*imm as u8);
            Ok(buf)
        }
        (Operand::Memory(mem), Operand::Register(count_reg)) => {
            if count_reg != &crate::parser::Register::CL {
                return Err(EncodingError::InvalidRegister(count_reg.clone()));
            }
            
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xD3);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, reg_code, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), count.value.clone())),
    }
}