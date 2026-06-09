use super::token::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // System
    Syscall, Sysenter, Sysexit, Ret, Nop, CpuId, Hlt, Int3, Rdtsc,
    
    // Stack
    Push(Spanned<Operand>), Pop(Spanned<Operand>),
    Enter(Spanned<Operand>, Spanned<Operand>), Leave,
    
    // Data movement
    Mov(Spanned<Operand>, Spanned<Operand>),
    Movsx(Spanned<Operand>, Spanned<Operand>), Movzx(Spanned<Operand>, Spanned<Operand>),
    Xchg(Spanned<Operand>, Spanned<Operand>),
    Lea(Spanned<Operand>, Spanned<Operand>),
    
    // Arithmetic
    Add(Spanned<Operand>, Spanned<Operand>), Sub(Spanned<Operand>, Spanned<Operand>),
    Inc(Spanned<Operand>), Dec(Spanned<Operand>), Neg(Spanned<Operand>),
    Mul(Spanned<Operand>), Imul(Spanned<Operand>),
    Div(Spanned<Operand>), Idiv(Spanned<Operand>),
    
    // Logical
    And(Spanned<Operand>, Spanned<Operand>), Or(Spanned<Operand>, Spanned<Operand>),
    Xor(Spanned<Operand>, Spanned<Operand>), Not(Spanned<Operand>),
    
    // Compare/Test
    Cmp(Spanned<Operand>, Spanned<Operand>), Test(Spanned<Operand>, Spanned<Operand>),
    
    // Shifts & Rotates
    Shl(Spanned<Operand>, Spanned<Operand>), Shr(Spanned<Operand>, Spanned<Operand>),
    Sal(Spanned<Operand>, Spanned<Operand>), Sar(Spanned<Operand>, Spanned<Operand>),
    Rol(Spanned<Operand>, Spanned<Operand>), Ror(Spanned<Operand>, Spanned<Operand>),
    Rcl(Spanned<Operand>, Spanned<Operand>), Rcr(Spanned<Operand>, Spanned<Operand>),
    
    // Bit test
    Bt(Spanned<Operand>, Spanned<Operand>), Bts(Spanned<Operand>, Spanned<Operand>),
    Btr(Spanned<Operand>, Spanned<Operand>), Btc(Spanned<Operand>, Spanned<Operand>),
    
    // Bit scan
    Bsf(Spanned<Operand>, Spanned<Operand>), Bsr(Spanned<Operand>, Spanned<Operand>),
    Popcnt(Spanned<Operand>, Spanned<Operand>),
    Lzcnt(Spanned<Operand>, Spanned<Operand>), Tzcnt(Spanned<Operand>, Spanned<Operand>),
    
    // Conditional moves
    Cmove(Spanned<Operand>, Spanned<Operand>), Cmovne(Spanned<Operand>, Spanned<Operand>),
    Cmovg(Spanned<Operand>, Spanned<Operand>), Cmovge(Spanned<Operand>, Spanned<Operand>),
    Cmovl(Spanned<Operand>, Spanned<Operand>), Cmovle(Spanned<Operand>, Spanned<Operand>),
    Cmova(Spanned<Operand>, Spanned<Operand>), Cmovae(Spanned<Operand>, Spanned<Operand>),
    Cmovb(Spanned<Operand>, Spanned<Operand>), Cmovbe(Spanned<Operand>, Spanned<Operand>),
    Cmovs(Spanned<Operand>, Spanned<Operand>), Cmovns(Spanned<Operand>, Spanned<Operand>),
    Cmovz(Spanned<Operand>, Spanned<Operand>), Cmovnz(Spanned<Operand>, Spanned<Operand>),
    
    Jmp(Spanned<Operand>), 
    Call(Spanned<Operand>),
    Jcc(ConditionCode, Spanned<Operand>),
    
    // String instructions
    Movsb, Movsw, Movsq, Movs,
    Cmpsb, Cmpsw, Cmpsd, Cmpsq,
    Scasb, Scasw, Scasd, Scasq,
    Stosb, Stosw, Stosd, Stosq,
    Lodsb, Lodsw, Lodsd, Lodsq,
    
    // Memory ordering
    Mfence, Lfence, Sfence,
    
    // SSE/AVX SIMD
    Movsd(Spanned<Operand>, Spanned<Operand>),
    Movss(Spanned<Operand>, Spanned<Operand>),
    Movaps(Spanned<Operand>, Spanned<Operand>),
    Movapd(Spanned<Operand>, Spanned<Operand>),
    Addpd(Spanned<Operand>, Spanned<Operand>),
    Addps(Spanned<Operand>, Spanned<Operand>),
    AddSd(Spanned<Operand>, Spanned<Operand>),
    AddSs(Spanned<Operand>, Spanned<Operand>),
    Vaddpd(Spanned<Operand>, Spanned<Operand>, Spanned<Operand>),
    Vaddps(Spanned<Operand>, Spanned<Operand>, Spanned<Operand>),

    Prefixed(Vec<Prefix>, Box<Instruction>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Lock,
    Rep,
    Repne,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionCode {
    E, NE, G, GE, L, LE, A, AE, B, BE, C, NC, Z, NZ, O, NO, S, NS, P, NP
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(String),
    Memory(MemoryAddress),
    Immediate(i64),
    Label(String), 
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAddress {
    pub base: String,
    pub displacement: i64,
}