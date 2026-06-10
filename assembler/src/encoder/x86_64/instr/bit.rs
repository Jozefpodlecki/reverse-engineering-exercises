use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn bt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_bit_op(dest, src, 0xA3)
}

pub fn bts(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_bit_op(dest, src, 0xAB)
}

pub fn btr(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_bit_op(dest, src, 0xB3)
}

pub fn btc(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_bit_op(dest, src, 0xBB)
}

fn encode_bit_op(
    dest: &Spanned<Operand>,
    src: &Spanned<Operand>,
    opcode: u8,
) -> Result<InstrBuf, EncodingError> {
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
            buf.push(0x0F);
            buf.push(opcode);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
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
            buf.push(0x0F);
            buf.push(opcode);
            
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
        (Operand::Register(_), Operand::Immediate(imm)) => {
            Err(EncodingError::ImmediateOutOfRange {
                value: *imm,
                min: 0,
                max: 255,
                size: crate::encoder::ImmediateSize::U8,
            })
        }
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone())),
    }
}

pub fn bsf(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_scan(dest, src, 0xBC)
}

pub fn bsr(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_scan(dest, src, 0xBD)
}

fn encode_scan(
    dest: &Spanned<Operand>,
    src: &Spanned<Operand>,
    opcode: u8,
) -> Result<InstrBuf, EncodingError> {
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
            buf.push(0x0F);
            buf.push(opcode);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new()
                .w(true)
                .r(dest_info.needs_rex)
                .b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x0F);
            buf.push(opcode);
            
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
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone())),
    }
}

pub fn popcnt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_popcnt_like(dest, src, 0xB8, 0xF3)
}

pub fn lzcnt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_popcnt_like(dest, src, 0xBD, 0xF3)
}

pub fn tzcnt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_popcnt_like(dest, src, 0xBC, 0xF3)
}

fn encode_popcnt_like(
    dest: &Spanned<Operand>,
    src: &Spanned<Operand>,
    opcode: u8,
    prefix: u8,
) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let src_info = RegInfo::from_reg(src_reg);
            let rex = RexPrefix::new()
                .w(true)
                .r(src_info.needs_rex)
                .b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            buf.push(prefix);
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x0F);
            buf.push(opcode);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new()
                .w(true)
                .r(dest_info.needs_rex)
                .b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            buf.push(prefix);
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x0F);
            buf.push(opcode);
            
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
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone())),
    }
}