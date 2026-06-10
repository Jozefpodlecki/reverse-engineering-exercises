use crate::{Token, parser::mnemonic::Mnemonic};

#[derive(Debug)]
pub enum ParserError {
    ExpectedToken {
        expected: Token,
        found: Token,
        line: usize,
        col: usize,
    },
    ExpectedOperand {
        found: Token,
        line: usize,
        col: usize,
    },
    ExpectedInstruction {
        found: Token,
        line: usize,
        col: usize,
    },
    ExpectedBaseRegister {
        line: usize,
        col: usize,
    },
    ExpectedDisplacement {
        line: usize,
        col: usize,
    },
    UnknownMnemonic {
        mnemonic: Mnemonic,
        line: usize,
        col: usize,
    },
    UnexpectedEof {
        expected: Token,
        line: usize,
        col: usize,
    },
    InvalidMemoryAddress {
        message: &'static str,
        line: usize,
        col: usize,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::ExpectedToken { expected, found, line, col } => {
                write!(f, "{}:{}: Expected {}, got {}", line, col, expected, found)
            }
            ParserError::ExpectedInstruction { found, line, col } => {
                write!(f, "{}:{}: Expected instruction got {}", line, col, found)
            }
            ParserError::ExpectedOperand { found, line, col } => {
                write!(f, "{}:{}: Expected operand, got {}", line, col, found)
            }
            ParserError::ExpectedBaseRegister { line, col } => {
                write!(f, "{}:{}: Expected base register in memory operand", line, col)
            }
            ParserError::ExpectedDisplacement { line, col } => {
                write!(f, "{}:{}: Expected displacement after + or -", line, col)
            }
            ParserError::UnknownMnemonic { mnemonic, line, col } => {
                write!(f, "{}:{}: Unknown mnemonic: {}", line, col, mnemonic)
            }
            ParserError::UnexpectedEof { expected, line, col } => {
                write!(f, "{}:{}: Expected {}, got EOF", line, col, expected)
            }
            ParserError::InvalidMemoryAddress { message, line, col } => {
                write!(f, "{}:{}: {}", line, col, message)
            }
        }
    }
}

impl std::error::Error for ParserError {}