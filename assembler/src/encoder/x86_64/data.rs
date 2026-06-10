use crate::{Spanned, parser::ast::{Operand, MemoryAddress}};
use super::{RegInfo, RexPrefix, ModRM, Mod, SIB, EncodingError};

pub fn encode_mov(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Result<Vec<u8>, EncodingError> {
    match (&dest.value, &src.value) {
        (Operand::Register(dest_reg), Operand::Immediate(imm)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let rex = RexPrefix::new().w(true).b(dest_info.needs_rex);
            
            let mut bytes = Vec::new();
            if let Some(rex_byte) = rex.encode() {
                bytes.push(rex_byte);
            }
            bytes.push(0xB8 + (dest_info.code & 0x7));
            bytes.extend_from_slice(&imm.to_le_bytes());
            Ok(bytes)
        }
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_info = RegInfo::from_reg(dest_reg);
            let src_info = RegInfo::from_reg(src_reg);
            let rex = RexPrefix::new()
                .w(true)
                .r(src_info.needs_rex)
                .b(dest_info.needs_rex);
            
            let mut bytes = Vec::new();
            if let Some(rex_byte) = rex.encode() {
                bytes.push(rex_byte);
            }
            bytes.push(0x89);
            let modrm = ModRM::reg_reg(dest_info.code & 0x7, src_info.code & 0x7);
            bytes.push(modrm.encode());
            Ok(bytes)
        }
        (Operand::Register(reg), Operand::Memory(mem)) => {
            encode_mov_reg_from_memory(reg, mem)
        }
        (Operand::Memory(mem), Operand::Register(reg)) => {
            encode_mov_memory_to_reg(mem, reg)
        }
        (Operand::Memory(mem), Operand::Immediate(imm)) => {
            encode_mov_memory_imm(mem, *imm)
        }
        _ => Err(EncodingError::UnsupportedOperand),
    }
}