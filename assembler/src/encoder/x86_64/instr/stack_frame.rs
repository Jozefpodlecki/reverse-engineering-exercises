use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn enter(imm16: &Spanned<Operand>, imm8: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    match (&imm16.value, &imm8.value) {
        (Operand::Immediate(alloc_bytes), Operand::Immediate(nest_level)) => {
            if *alloc_bytes < 0 || *alloc_bytes > 65535 {
                return Err(EncodingError::ImmediateOutOfRange {
                    value: *alloc_bytes,
                    min: 0,
                    max: 65535,
                    size: crate::encoder::ImmediateSize::U16,
                });
            }
            if *nest_level < 0 || *nest_level > 255 {
                return Err(EncodingError::ImmediateOutOfRange {
                    value: *nest_level,
                    min: 0,
                    max: 255,
                    size: crate::encoder::ImmediateSize::U8,
                });
            }
            
            let mut buf = InstrBuf::new();
            buf.push(0xC8);
            buf.push_u16(*alloc_bytes as u16);
            buf.push(*nest_level as u8);
            Ok(buf)
        }
        _ => Err(EncodingError::InvalidOperandCombination(imm16.value.clone(), imm8.value.clone())),
    }
}