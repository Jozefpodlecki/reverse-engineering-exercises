use iced_x86::{Decoder, DecoderOptions, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderError {
    None = 0,
    InvalidInstruction = 1,
    NoMoreBytes = 2,
}

impl From<iced_x86::DecoderError> for DecoderError {
    fn from(err: iced_x86::DecoderError) -> Self {
        match err {
            iced_x86::DecoderError::InvalidInstruction => DecoderError::InvalidInstruction,
            iced_x86::DecoderError::NoMoreBytes => DecoderError::NoMoreBytes,
            _ => DecoderError::InvalidInstruction,
        }
    }
}

pub fn decode_instruction(bytes: &[u8], ip: u64) -> Result<Instruction, DecoderError> {
    let mut decoder = Decoder::with_ip(64, bytes, ip, DecoderOptions::NONE);
    let instruction = decoder.decode();
    
     if instruction.is_invalid() {
        let err = decoder.last_error().into();
        return Err(err);
    }
  
    Ok(instruction)
}