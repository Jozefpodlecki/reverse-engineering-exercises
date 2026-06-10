use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn cmove(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x44)
}

pub fn cmovne(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x45)
}

pub fn cmovg(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x4F)
}

pub fn cmovge(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x4D)
}

pub fn cmovl(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x4C)
}

pub fn cmovle(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x4E)
}

pub fn cmova(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x47)
}

pub fn cmovae(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x43)
}

pub fn cmovb(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x42)
}

pub fn cmovbe(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x46)
}

pub fn cmovs(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x48)
}

pub fn cmovns(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    encode_cmov(dest, src, 0x49)
}

fn encode_cmov(
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