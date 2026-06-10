#[derive(Debug)]
pub enum EncodingError {
    UnsupportedOperand(String),
    UnsupportedInstruction(String),
    InvalidRegister(String),
    LabelNotFound(String),
    InvalidImmediate(String),
    UnknownMemoryAddress(String),
}

impl std::fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::UnsupportedOperand(msg) => write!(f, "Unsupported operand: {}", msg),
            EncodingError::UnsupportedInstruction(msg) => write!(f, "Unsupported instruction: {}", msg),
            EncodingError::InvalidRegister(reg) => write!(f, "Invalid register: {}", reg),
            EncodingError::LabelNotFound(label) => write!(f, "Label not found: {}", label),
            EncodingError::InvalidImmediate(imm) => write!(f, "Invalid immediate value: {}", imm),
            EncodingError::UnknownMemoryAddress(msg) => write!(f, "Unknown memory address: {}", msg),
        }
    }
}

impl std::error::Error for EncodingError {}