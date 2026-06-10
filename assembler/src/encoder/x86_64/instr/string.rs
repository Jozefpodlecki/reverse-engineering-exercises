// encoder/x86_64/instr/string.rs

use crate::encoder::buffer::InstrBuf;
use crate::encoder::EncodingError;

pub fn movsb() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xA4);
    Ok(buf)
}

pub fn movsw() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0x66).push(0xA5);
    Ok(buf)
}

pub fn movsq() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xA5);
    Ok(buf)
}

pub fn movs() -> Result<InstrBuf, EncodingError> {
    movsd()
}

pub fn movsd() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xA5);
    Ok(buf)
}

pub fn cmpsb() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xA6);
    Ok(buf)
}

pub fn cmpsw() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0x66).push(0xA7);
    Ok(buf)
}

pub fn cmpsd() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xA7);
    Ok(buf)
}

pub fn cmpsq() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xA7);
    Ok(buf)
}

pub fn scasb() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAE);
    Ok(buf)
}

pub fn scasw() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0x66).push(0xAF);
    Ok(buf)
}

pub fn scasd() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAF);
    Ok(buf)
}

pub fn scasq() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAF);
    Ok(buf)
}

pub fn stosb() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0xAA);
    Ok(buf)
}

pub fn stosw() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0x66).push(0xAB);
    Ok(buf)
}

pub fn stosd() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAB);
    Ok(buf)
}

pub fn stosq() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAB);
    Ok(buf)
}

pub fn lodsb() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAC);
    Ok(buf)
}

pub fn lodsw() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0x66).push(0xAD);
    Ok(buf)
}

pub fn lodsd() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAD);
    Ok(buf)
}

pub fn lodsq() -> Result<InstrBuf, EncodingError> {
    let mut buf = InstrBuf::new();
    buf.push(0x48).push(0xAD);
    Ok(buf)
}