use crate::{Mnemonic, Register, ast::Operand};

#[derive(Debug)]
pub enum ImmediateSize {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    I128,
    U128,
    I256,
    U256,
    I512,
    U512,
}

impl core::fmt::Display for ImmediateSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::I8 => write!(f, "8-bit signed"),
            Self::I16 => write!(f, "16-bit signed"),
            Self::I32 => write!(f, "32-bit signed"),
            Self::I64 => write!(f, "64-bit signed"),
            Self::U8 => write!(f, "8-bit unsigned"),
            Self::U16 => write!(f, "16-bit unsigned"),
            Self::U32 => write!(f, "32-bit unsigned"),
            Self::U64 => write!(f, "64-bit unsigned"),
            Self::I128 => write!(f, "128-bit signed"),
            Self::U128 => write!(f, "128-bit unsigned"),
            Self::I256 => write!(f, "256-bit signed"),
            Self::U256 => write!(f, "256-bit unsigned"),
            Self::I512 => write!(f, "512-bit signed"),
            Self::U512 => write!(f, "512-bit unsigned"),
        }
    }
}

#[derive(Debug)]
pub enum EncodingError {
    UnsupportedMnemonic(Mnemonic),
    UnsupportedOperand(Operand),
    InvalidRegister(Register),
    LabelNotFound(String),
    ImmediateOutOfRange {
        value: i64,
        min: i64,
        max: i64,
        size: ImmediateSize,
    },
    ImmediateNotByteAligned(i64),
    ImmediateNotPowerOfTwo(i64),
    InvalidDisplacement(i64),
    UnsupportedAddressing,
    RequiresRexPrefix(Register),
    RequiresVexPrefix(Mnemonic),
    InvalidOperandCombination(Operand, Operand),
    MissingOperand(&'static str),
    ExtraOperands,
}

impl std::fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::UnsupportedMnemonic(m) => write!(f, "Unsupported mnemonic: {:?}", m),
            EncodingError::UnsupportedOperand(op) => write!(f, "Unsupported operand: {:?}", op),
            EncodingError::InvalidRegister(r) => write!(f, "Invalid register: {:?}", r),
            EncodingError::LabelNotFound(l) => write!(f, "Label not found: {}", l),
            EncodingError::ImmediateOutOfRange { value, min, max, size } => {
                write!(f, "Immediate {} out of range for {} ({}..{})", value, size, min, max)
            }
            EncodingError::ImmediateNotByteAligned(v) => {
                write!(f, "Immediate {} must be byte-aligned", v)
            }
            EncodingError::ImmediateNotPowerOfTwo(v) => {
                write!(f, "Immediate {} must be a power of two", v)
            }
            EncodingError::InvalidDisplacement(v) => {
                write!(f, "Displacement {} out of valid range", v)
            }
            EncodingError::UnsupportedAddressing => write!(f, "Unsupported addressing mode"),
            EncodingError::RequiresRexPrefix(r) => {
                write!(f, "Register {:?} requires REX prefix but not allowed", r)
            }
            EncodingError::RequiresVexPrefix(m) => {
                write!(f, "Mnemonic {:?} requires VEX prefix", m)
            }
            EncodingError::InvalidOperandCombination(dest, src) => {
                write!(f, "Invalid operand combination: {:?}, {:?}", dest, src)
            }
            EncodingError::MissingOperand(op) => write!(f, "Missing operand: {}", op),
            EncodingError::ExtraOperands => write!(f, "Too many operands"),
        }
    }
}