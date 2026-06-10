use crate::{LexerErrors, SourceError, encoder::EncodingError};
use super::ParserError;

#[derive(Debug)]
pub enum AssemblerError {
    SourceError(SourceError),
    LexerError(LexerErrors),
    ParserError(ParserError),
    EncodingError(EncodingError),
}

impl core::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AssemblerError::SourceError(e) => write!(f, "Source error: {}", e),
            AssemblerError::LexerError(e) => write!(f, "Lexer error: {}", e),
            AssemblerError::ParserError(e) => write!(f, "Parser error: {}", e),
            AssemblerError::EncodingError(e) => write!(f, "Encoding error: {}", e),
        }
    }
}

impl std::error::Error for AssemblerError {}

impl From<SourceError> for AssemblerError {
    fn from(err: SourceError) -> Self {
        AssemblerError::SourceError(err)
    }
}

impl From<LexerErrors> for AssemblerError {
    fn from(err: LexerErrors) -> Self {
        AssemblerError::LexerError(err)
    }
}

impl From<ParserError> for AssemblerError {
    fn from(err: ParserError) -> Self {
        AssemblerError::ParserError(err)
    }
}

impl From<EncodingError> for AssemblerError {
    fn from(err: EncodingError) -> Self {
        AssemblerError::EncodingError(err)
    }
}