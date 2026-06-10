use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn push(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().b(info.needs_rex);
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            if info.code >= 8 {
                buf.push(0x50 + (info.code - 8));
            } else {
                buf.push(0x50 + info.code);
            }
            Ok(buf)
        }
        Operand::Immediate(imm) => {
            let mut buf = InstrBuf::new();
            buf.push(0x68).push_u32(*imm as u32);
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}

pub fn pop(op: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match &op.value {
        Operand::Register(reg) => {
            let info = RegInfo::from_reg(reg);
            let rex = RexPrefix::new().b(info.needs_rex);
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            if info.code >= 8 {
                buf.push(0x58 + (info.code - 8));
            } else {
                buf.push(0x58 + info.code);
            }
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(op.value.clone())),
    }
}