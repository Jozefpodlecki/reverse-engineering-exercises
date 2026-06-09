#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Mnemonic(String),
    Register(String),
    Immediate(i64),
    Label(String),
    OpenBracket,
    CloseBracket,
    Comma,
    Plus,
    Minus,
    Colon,
    Lock,
    Rep,
    Repne,
    XmmRegister(String),
    YmmRegister(String),
    ZmmRegister(String),
    Star,
    Slash,
    Percent,
    Equal,
    Exclamation,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    ShiftLeft,
    ShiftRight,
    Dot,
    Dollar,
    Hash,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub location: Location,
}