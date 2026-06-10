use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn movsd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_move(dest, src, 0xF2, 0x0F, 0x10)
}

pub fn movss(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_move(dest, src, 0xF3, 0x0F, 0x10)
}

pub fn movaps(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_move(dest, src, 0x00, 0x0F, 0x28)
}

pub fn movapd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_move(dest, src, 0x66, 0x0F, 0x28)
}

fn simd_move(
    dest: &Spanned<Operand>,
    src: &Spanned<Operand>,
    prefix: u8,
    opcode1: u8,
    opcode2: u8,
) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            if !dest_reg.is_xmm() || !src_reg.is_xmm() {
                return Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone()));
            }
            let dest_info = RegInfo::from_reg(dest_reg);
            let src_info = RegInfo::from_reg(src_reg);
            let rex = RexPrefix::new()
                .r(src_info.needs_rex)
                .b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if prefix != 0x00 {
                buf.push(prefix);
            }
            buf.push(opcode1);
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(opcode2);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            if !dest_reg.is_xmm() {
                return Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone()));
            }
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new()
                .r(dest_info.needs_rex)
                .b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if prefix != 0x00 {
                buf.push(prefix);
            }
            buf.push(opcode1);
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(opcode2);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, dest_info.code & 0x7, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone())),
    }
}

pub fn addpd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_arithmetic(dest, src, 0x66, 0x0F, 0x58)
}

pub fn addps(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_arithmetic(dest, src, 0x00, 0x0F, 0x58)
}

pub fn addsd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_arithmetic(dest, src, 0xF2, 0x0F, 0x58)
}

pub fn addss(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    simd_arithmetic(dest, src, 0xF3, 0x0F, 0x58)
}

fn simd_arithmetic(
    dest: &Spanned<Operand>,
    src: &Spanned<Operand>,
    prefix: u8,
    opcode1: u8,
    opcode2: u8,
) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            if !dest_reg.is_xmm() || !src_reg.is_xmm() {
                return Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone()));
            }
            let dest_info = RegInfo::from_reg(dest_reg);
            let src_info = RegInfo::from_reg(src_reg);
            let rex = RexPrefix::new()
                .r(src_info.needs_rex)
                .b(dest_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if prefix != 0x00 {
                buf.push(prefix);
            }
            buf.push(opcode1);
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(opcode2);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        (Operand::Register(dest_reg), Operand::Memory(mem)) => {
            if !dest_reg.is_xmm() {
                return Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone()));
            }
            let dest_info = RegInfo::from_reg(dest_reg);
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new()
                .r(dest_info.needs_rex)
                .b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if prefix != 0x00 {
                buf.push(prefix);
            }
            buf.push(opcode1);
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(opcode2);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, dest_info.code & 0x7, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src.value.clone())),
    }
}

pub fn vaddpd(
    dest: &Spanned<Operand>,
    src1: &Spanned<Operand>,
    src2: &Spanned<Operand>,
) -> Result<InstrBuf, EncodingError> {
    avx_arithmetic(dest, src1, src2, 0x66, 0x58)
}

pub fn vaddps(
    dest: &Spanned<Operand>,
    src1: &Spanned<Operand>,
    src2: &Spanned<Operand>,
) -> Result<InstrBuf, EncodingError> {
    avx_arithmetic(dest, src1, src2, 0x00, 0x58)
}

fn avx_arithmetic(
    dest: &Spanned<Operand>,
    src1: &Spanned<Operand>,
    src2: &Spanned<Operand>,
    prefix: u8,
    opcode: u8,
) -> Result<InstrBuf, EncodingError> {
    match (&dest.value, &src1.value, &src2.value) {
        (Operand::Register(dest_reg), Operand::Register(src1_reg), Operand::Register(src2_reg)) => {
            if !dest_reg.is_ymm() || !src1_reg.is_ymm() || !src2_reg.is_ymm() {
                return Err(EncodingError::InvalidOperandCombination(dest.value.clone(), src1.value.clone()));
            }
            let dest_info = RegInfo::from_reg(dest_reg);
            let src1_info = RegInfo::from_reg(src1_reg);
            let src2_info = RegInfo::from_reg(src2_reg);
            
            let mut buf = InstrBuf::new();
            if prefix != 0x00 {
                buf.push(prefix);
            }
            buf.push(0xC5);
            let vex = 0xF0 | (src1_info.needs_rex as u8) << 2 | (dest_info.needs_rex as u8);
            buf.push(vex);
            buf.push(opcode);
            
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src2_info.code & 0x7);
            buf.push(modrm.encode());
            Ok(buf)
        }
        _ => Err(EncodingError::InvalidOperandCombination(
            dest.value.clone(),
            src1.value.clone(),
        )),
    }
}