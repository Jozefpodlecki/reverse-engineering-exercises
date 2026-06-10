use crate::{parser::{mnemonic::Mnemonic, register::Register}, string::StackString};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Byte,
    Word,
    Dword,
    Qword,
    Mnemonic(Mnemonic),
    Register(Register),
    Immediate(i64),
    Label(StackString<32>),
    OpenBracket,
    CloseBracket,
    Comma,
    Plus,
    Minus,
    Colon,
    Lock,
    Rep,
    Repne,
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
    Cld,
    Std,
    Clc,
    Stc,
    Cli,
    Sti,
    Prefetch,
    Prefetchnta,
    Prefetcht0,
    Prefetcht1,
    Prefetcht2,
    Newline,
    Eof,
}

impl Token {
    pub fn from_ident(ident: &str) -> Option<Self> {
        match ident {
            "byte" => Some(Token::Byte),
            "word" => Some(Token::Word),
            "dword" => Some(Token::Dword),
            "qword" => Some(Token::Qword),
            "lock" => Some(Token::Lock),
            "rep" => Some(Token::Rep),
            "repne" | "repnz" => Some(Token::Repne),
            "cld" => Some(Token::Cld),
            "std" => Some(Token::Std),
            "clc" => Some(Token::Clc),
            "stc" => Some(Token::Stc),
            "cli" => Some(Token::Cli),
            "sti" => Some(Token::Sti),
            "prefetch" => Some(Token::Prefetch),
            "prefetchnta" => Some(Token::Prefetchnta),
            "prefetcht0" => Some(Token::Prefetcht0),
            "prefetcht1" => Some(Token::Prefetcht1),
            "prefetcht2" => Some(Token::Prefetcht2),
            _ => {
                if let Some(reg) = Register::from_str(ident) {
                    Some(Token::Register(reg))
                } else if let Some(m) = Mnemonic::from_str(ident) {
                    Some(Token::Mnemonic(m))
                } else {
                    None
                }
            }
        }
    }
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Byte => write!(f, "Byte"),
            Self::Word => write!(f, "Word"),
            Self::Dword => write!(f, "Dword"),
            Self::Qword => write!(f, "Qword"),
            Self::Mnemonic(m) => write!(f, "Mnemonic({})", m),
            Self::Register(r) => write!(f, "Register({})", r),
            Self::Immediate(i) => write!(f, "Immediate({})", i),
            Self::Label(l) => write!(f, "Label({})", l),
            Self::OpenBracket => write!(f, "["),
            Self::CloseBracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Colon => write!(f, ":"),
            Self::Lock => write!(f, "lock"),
            Self::Rep => write!(f, "rep"),
            Self::Repne => write!(f, "repne"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Equal => write!(f, "="),
            Self::Exclamation => write!(f, "!"),
            Self::Ampersand => write!(f, "&"),
            Self::Pipe => write!(f, "|"),
            Self::Caret => write!(f, "^"),
            Self::Tilde => write!(f, "~"),
            Self::ShiftLeft => write!(f, "<<"),
            Self::ShiftRight => write!(f, ">>"),
            Self::Dot => write!(f, "."),
            Self::Dollar => write!(f, "$"),
            Self::Hash => write!(f, "#"),
            Self::Cld => write!(f, "cld"),
            Self::Std => write!(f, "std"),
            Self::Clc => write!(f, "clc"),
            Self::Stc => write!(f, "stc"),
            Self::Cli => write!(f, "cli"),
            Self::Sti => write!(f, "sti"),
            Self::Prefetch => write!(f, "prefetch"),
            Self::Prefetchnta => write!(f, "prefetchnta"),
            Self::Prefetcht0 => write!(f, "prefetcht0"),
            Self::Prefetcht1 => write!(f, "prefetcht1"),
            Self::Prefetcht2 => write!(f, "prefetcht2"),
            Self::Newline => write!(f, "\\n"),
            Self::Eof => write!(f, "EOF"),
        }
    }
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