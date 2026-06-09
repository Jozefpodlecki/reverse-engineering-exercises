use super::token::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Syscall,
    Push(Spanned<Operand>),
    Pop(Spanned<Operand>),
    Mov(Spanned<Operand>, Spanned<Operand>),
    Sub(Spanned<Operand>, Spanned<Operand>),
    Add(Spanned<Operand>, Spanned<Operand>),
    Xor(Spanned<Operand>, Spanned<Operand>),
    Ret,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(String),
    Memory(MemoryAddress),
    Immediate(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAddress {
    pub base: String,
    pub displacement: i64,
}