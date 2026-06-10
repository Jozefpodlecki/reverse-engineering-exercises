use crate::{Spanned, encoder::{EncodingError, x86_64::{modrm::{Mod, ModRM}, registers::RegInfo, rex::RexPrefix, sib::SIB}}, parser::ast::{MemoryAddress, Operand}};
use crate::encoder::buffer::InstrBuf;

pub fn mov(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Immediate(imm)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let rex = RexPrefix::new().w(true).b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0xB8 + (dest_info.code & 0x7));
            buf.push_u64(*imm as u64);
            Ok(buf)
        }
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
            buf.push(0x89);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Register(reg), Operand::Memory(mem)) => {
            mov_reg_from_memory(reg, mem)
        }
        (Operand::Memory(mem), Operand::Register(reg)) => {
            mov_memory_to_reg(mem, reg)
        }
        (Operand::Memory(mem), Operand::Immediate(imm)) => {
            mov_memory_imm(mem, *imm)
        }
        _ => Err(EncodingError::UnsupportedOperand(dest.value.clone())),
    }
}

fn mov_reg_from_memory(reg: &crate::parser::Register, mem: &MemoryAddress) -> Result<InstrBuf, EncodingError> {
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
    buf.push(0x8B);
    
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

fn mov_memory_to_reg(mem: &MemoryAddress, reg: &crate::parser::Register) -> Result<InstrBuf, EncodingError> {
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
    buf.push(0x89);
    
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

fn mov_memory_imm(mem: &MemoryAddress, imm: i64) -> Result<InstrBuf, EncodingError> {
    let base_info = RegInfo::from_reg(&mem.base);
    let rex = RexPrefix::new().w(true).b(base_info.needs_rex);
    
    let mut buf = InstrBuf::new();
    if let Some(rex_byte) = rex.encode() {
        buf.push(rex_byte);
    }
    buf.push(0xC7);
    
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
    
    buf.push_u32(imm as u32);
    Ok(buf)
}

pub fn movsx(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).r(dest_info.needs_rex).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x0F).push(0xBE);
            
            let modrm = if mem.displacement == 0 {
                ModRM::new(Mod::Indirect, dest_info.code & 0x7, base_info.code & 0x7)
            } else {
                ModRM::new(Mod::Disp8, dest_info.code & 0x7, base_info.code & 0x7)
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
        _ => Err(EncodingError::UnsupportedOperand(dest.value.clone())),
    }
}

pub fn movzx(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).r(dest_info.needs_rex).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x0F).push(0xB6);
            
            let modrm = if mem.displacement == 0 {
                ModRM::new(Mod::Indirect, dest_info.code & 0x7, base_info.code & 0x7)
            } else {
                ModRM::new(Mod::Disp8, dest_info.code & 0x7, base_info.code & 0x7)
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
        _ => Err(EncodingError::UnsupportedOperand(dest.value.clone())),
    }
}

pub fn lea(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).r(dest_info.needs_rex).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x8D);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, dest_info.code & 0x7, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(dest.value.clone())),
    }
}

pub fn xchg(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let src_info = RegInfo::from_reg(src_reg);
            let rex = RexPrefix::new().w(true).r(src_info.needs_rex).b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x87);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Memory(mem), Operand::Register(reg)) => {
            let reg_info = RegInfo::from_reg(reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().w(true).r(reg_info.needs_rex).b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x87);
            
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
        _ => Err(EncodingError::UnsupportedOperand(dest.value.clone())),
    }
}