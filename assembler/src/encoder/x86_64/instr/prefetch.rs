use crate::encoder::EncodingError;
use crate::encoder::buffer::InstrBuf;
use crate::encoder::x86_64::modrm::{Mod, ModRM};
use crate::encoder::x86_64::registers::RegInfo;
use crate::encoder::x86_64::rex::RexPrefix;
use crate::encoder::x86_64::sib::SIB;
use crate::parser::ast::Operand;
use crate::Spanned;

pub fn prefetch(addr: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    prefetch_common(addr, 0x00)
}

pub fn prefetchnta(addr: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    prefetch_common(addr, 0x00)
}

pub fn prefetcht0(addr: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    prefetch_common(addr, 0x01)
}

pub fn prefetcht1(addr: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    prefetch_common(addr, 0x02)
}

pub fn prefetcht2(addr: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    prefetch_common(addr, 0x03)
}

pub fn prefetchw(addr: &Spanned<Operand>) -> Result<InstrBuf, EncodingError> {
    prefetch_common(addr, 0x04)
}

fn prefetch_common(addr: &Spanned<Operand>, hint: u8) -> Result<InstrBuf, EncodingError> {
    match &addr.value {
        Operand::Memory(mem) => {
            let base_info = RegInfo::from_reg(&mem.base);
            let rex = RexPrefix::new().b(base_info.needs_rex);
            
            let mut buf = InstrBuf::new();
            if let Some(rex_byte) = rex.encode() {
                buf.push(rex_byte);
            }
            buf.push(0x0F);
            buf.push(0x18);
            
            let (mod_val, has_disp) = if mem.displacement == 0 {
                (Mod::Indirect, false)
            } else if mem.displacement >= -128 && mem.displacement <= 127 {
                (Mod::Disp8, true)
            } else {
                (Mod::Disp32, true)
            };
            
            let modrm = ModRM::new(mod_val, hint, base_info.code & 0x7);
            buf.push(modrm.encode());
            
            if base_info.code == 4 {
                buf.push(SIB::for_rsp().encode());
            }
            
            if has_disp {
                buf.push(mem.displacement as u8);
            }
            
            Ok(buf)
        }
        _ => Err(EncodingError::UnsupportedOperand(addr.value.clone())),
    }
}