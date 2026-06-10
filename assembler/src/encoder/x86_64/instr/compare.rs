use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn cmp(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let src_info = RegInfo::from_reg(src_reg);
            let rex = RexPrefix::new()
                .w(true)
                .r(src_info.needs_rex)
                .b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x39);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Register(reg), Operand::Immediate(imm)) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            
            if *imm >= -128 && *imm <= 127 {
                buf.push(0x83);
                buf.push(0xF8 + (info.code & 0x7));
                buf.push(*imm as u8);
            } else {
                buf.push(0x81);
                buf.push(0xF8 + (info.code & 0x7));
                buf.push_u32(*imm as u32);
            }
            Ok(buf)
        }
        (Operand::Memory(mem), Operand::Immediate(imm)) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            
            if *imm >= -128 && *imm <= 127 {
                buf.push(0x83);
            } else {
                buf.push(0x81);
            }
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x07, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            
            if *imm >= -128 && *imm <= 127 {
                buf.push(*imm as u8);
            } else {
                buf.push_u32(*imm as u32);
            }
            Ok(buf)
        }
        (Operand::Memory(mem), Operand::Register(reg)) => {
            let reg_info = RegInfo::from_reg(reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new()
                .w(true)
                .r(reg_info.needs_rex)
                .b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x39);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, reg_info.code & 0x7, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone())),
    }
}