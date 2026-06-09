#[derive(Debug)]
pub enum AssemblerError {
    SourceError(String),
    LexerError(String, usize, usize),
    ParserError(String, usize, usize),
    EncodingError(String),
}

impl core::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AssemblerError::SourceError(msg) => write!(f, "Source error: {}", msg),
            AssemblerError::LexerError(msg, line, col) => {
                write!(f, "Lexer error at {}:{}: {}", line, col, msg)
            }
            AssemblerError::ParserError(msg, line, col) => {
                write!(f, "Parser error at {}:{}: {}", line, col, msg)
            }
            AssemblerError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
        }
    }
}

impl core::error::Error for AssemblerError {}