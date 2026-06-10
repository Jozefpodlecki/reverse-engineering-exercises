
pub struct LexerErrors(pub Vec<LexerError>);

#[derive(Debug, Clone)]
pub struct LexerError {
    pub kind: LexerErrorType,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub enum LexerErrorType {
    UnexpectedChar(char),
    InvalidHexNumber(String),
    InvalidDecimalNumber(String),
    UnterminatedComment,
    UnterminatedString,
    UnexpectedEof,
    Overflow,
    InvalidEscapeSequence(char),
    InvalidIdentifier(String),
    CapacityExceeded,
}

impl core::fmt::Display for LexerErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for err in &self.0 {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl core::fmt::Debug for LexerErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::error::Error for LexerErrors {}

impl core::fmt::Display for LexerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            LexerErrorType::UnexpectedChar(c) => {
                write!(f, "{}:{}: unexpected character: '{}'", self.line, self.col, c)
            }
            LexerErrorType::InvalidHexNumber(s) => {
                write!(f, "{}:{}: invalid hex number: {}", self.line, self.col, s)
            }
            LexerErrorType::InvalidDecimalNumber(s) => {
                write!(f, "{}:{}: invalid decimal number: {}", self.line, self.col, s)
            }
            LexerErrorType::UnterminatedComment => {
                write!(f, "{}:{}: unterminated comment", self.line, self.col)
            }
            LexerErrorType::UnterminatedString => {
                write!(f, "{}:{}: unterminated string", self.line, self.col)
            }
            LexerErrorType::UnexpectedEof => {
                write!(f, "{}:{}: unexpected end of file", self.line, self.col)
            }
            LexerErrorType::Overflow => {
                write!(f, "{}:{}: numeric overflow", self.line, self.col)
            }
            LexerErrorType::InvalidEscapeSequence(c) => {
                write!(f, "{}:{}: invalid escape sequence: '{}'", self.line, self.col, c)
            }
            LexerErrorType::InvalidIdentifier(s) => {
                write!(f, "{}:{}: invalid identifier: {}", self.line, self.col, s)
            }
            LexerErrorType::CapacityExceeded => {
                write!(f, "{}:{}: identifier capacity exceeded", self.line, self.col)
            }
        }
    }
}

impl core::error::Error for LexerError {}