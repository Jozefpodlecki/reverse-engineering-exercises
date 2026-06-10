// encoder/x86_64/instr/fence.rs

use crate::encoder::buffer::InstrBuf;
use crate::encoder::EncodingError;

pub const fn mfence() -> InstrBuf {
    InstrBuf::from_array([0x0F, 0xAE, 0xF0])
}

pub fn lfence() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x0F).push(0xAE).push(0xE8);
    Ok(buf)
}

pub fn sfence() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x0F).push(0xAE).push(0xF8);
    Ok(buf)
}