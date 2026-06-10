use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn add(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
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
            buf.push(0x01);
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
                buf.push(0xC0 + (info.code & 0x7));
                buf.push(*imm as u8);
            } else {
                buf.push(0x81);
                buf.push(0xC0 + (info.code & 0x7));
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
            
            let modrm = ModRM::new(mod_val, 0x00, base_info.code & 0x7);
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
            buf.push(0x01);
            
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

pub fn sub(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
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
            buf.push(0x29);
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
                buf.push(0xE8 + (info.code & 0x7));
                buf.push(*imm as u8);
            } else {
                buf.push(0x81);
                buf.push(0xE8 + (info.code & 0x7));
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
            
            let modrm = ModRM::new(mod_val, 0x05, base_info.code & 0x7);
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
            buf.push(0x29);
            
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

pub fn mul(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            buf.push(0xE0 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x04, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn imul(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            buf.push(0xE8 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x05, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn div(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            buf.push(0xF0 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x06, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn idiv(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            buf.push(0xF8 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            
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
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn inc(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            buf.push(0xC0 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x00, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn dec(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            buf.push(0xC8 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xFF);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x01, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn neg(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            buf.push(0xD8 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x03, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn not(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().w(true).b(info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            buf.push(0xD0 + (info.code & 0x7));
            Ok(buf)
        }
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xF7);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, 0x02, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}