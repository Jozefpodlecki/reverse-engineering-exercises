use crate::parser::register::Register;

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
    Label(String),

    Prefetch(Spanned<Operand>),
    Prefetchnta(Spanned<Operand>),
    Prefetcht0(Spanned<Operand>),
    Prefetcht1(Spanned<Operand>),
    Prefetcht2(Spanned<Operand>),
    Prefetchw(Spanned<Operand>),  // AMD
}

impl Instruction {
    pub fn estimate_size(&self) -> usize {
        match self {
            Instruction::Syscall | Instruction::Ret | Instruction::Nop | Instruction::Int3 | 
            Instruction::Hlt | Instruction::CpuId | Instruction::Rdtsc | Instruction::Leave => 2,
            
            Instruction::Push(_) | Instruction::Pop(_) | Instruction::Inc(_) | Instruction::Dec(_) |
            Instruction::Neg(_) | Instruction::Not(_) | Instruction::Mul(_) | Instruction::Imul(_) |
            Instruction::Div(_) | Instruction::Idiv(_) => 3,
            
            Instruction::Mov(dest, src) => {
                match (&dest.value, &src.value) {
                    (Operand::Register(reg), Operand::Immediate(_)) => {
                        match reg {
                            Register::RAX | Register::RCX | Register::RDX | Register::RBX |
                            Register::RSP | Register::RBP | Register::RSI | Register::RDI |
                            Register::R8 | Register::R9 | Register::R10 | Register::R11 |
                            Register::R12 | Register::R13 | Register::R14 | Register::R15 => 10,
                            Register::EAX | Register::ECX | Register::EDX | Register::EBX |
                            Register::ESP | Register::EBP | Register::ESI | Register::EDI => 5,
                            _ => 3,
                        }
                    }
                    (Operand::Register(_), Operand::Register(_)) => 3,
                    (Operand::Memory(_), Operand::Register(_)) => 5,
                    (Operand::Register(_), Operand::Memory(_)) => 5,
                    _ => 4,
                }
            }
            
            Instruction::Add(dest, _) | Instruction::Sub(dest, _) |
            Instruction::Xor(dest, _) | Instruction::And(dest, _) | Instruction::Or(dest, _) |
            Instruction::Cmp(dest, _) | Instruction::Test(dest, _) | Instruction::Lea(dest, _) => {
                match dest.value {
                    Operand::Register(_) => 4,
                    Operand::Memory(_) => 5,
                    _ => 4,
                }
            }
            
            Instruction::Jmp(_) | Instruction::Call(_) => 5,
            Instruction::Jcc(_, _) => 6,
            
            Instruction::Shl(_, _) | Instruction::Shr(_, _) | Instruction::Sar(_, _) |
            Instruction::Rol(_, _) | Instruction::Ror(_, _) | Instruction::Rcl(_, _) |
            Instruction::Rcr(_, _) => 4,
            
            Instruction::Bt(_, _) | Instruction::Bts(_, _) | Instruction::Btr(_, _) |
            Instruction::Btc(_, _) => 5,
            
            Instruction::Bsf(_, _) | Instruction::Bsr(_, _) | Instruction::Popcnt(_, _) |
            Instruction::Lzcnt(_, _) | Instruction::Tzcnt(_, _) => 5,
            
            Instruction::Cmove(_, _) | Instruction::Cmovne(_, _) | Instruction::Cmovg(_, _) |
            Instruction::Cmovge(_, _) | Instruction::Cmovl(_, _) | Instruction::Cmovle(_, _) |
            Instruction::Cmova(_, _) | Instruction::Cmovae(_, _) | Instruction::Cmovb(_, _) |
            Instruction::Cmovbe(_, _) | Instruction::Cmovs(_, _) | Instruction::Cmovns(_, _) |
            Instruction::Cmovz(_, _) | Instruction::Cmovnz(_, _) => 5,
            
            Instruction::Movsb | Instruction::Movsw | Instruction::Movsq |
            Instruction::Cmpsb | Instruction::Cmpsw | Instruction::Cmpsd | Instruction::Cmpsq |
            Instruction::Scasb | Instruction::Scasw | Instruction::Scasd | Instruction::Scasq |
            Instruction::Stosb | Instruction::Stosw | Instruction::Stosd | Instruction::Stosq |
            Instruction::Lodsb | Instruction::Lodsw | Instruction::Lodsd | Instruction::Lodsq => 2,
            
            Instruction::Movs => 2,
            
            Instruction::Mfence | Instruction::Lfence | Instruction::Sfence => 3,
            
            Instruction::Enter(_, _) => 4,
            
            Instruction::Movsx(_, _) | Instruction::Movzx(_, _) | Instruction::Xchg(_, _) => 4,
            
            Instruction::Vaddpd(_, _, _) | Instruction::Vaddps(_, _, _) => 4,
            
            Instruction::Movsd(_, _) | Instruction::Movss(_, _) | Instruction::Movaps(_, _) |
            Instruction::Movapd(_, _) | Instruction::Addpd(_, _) | Instruction::Addps(_, _) |
            Instruction::AddSd(_, _) | Instruction::AddSs(_, _) => 4,
            
            Instruction::Prefixed(items, _) => items.len() + self.estimate_size(),
            
            Instruction::Label(_) => 0,
            
            Instruction::Prefetch(_) | Instruction::Prefetchnta(_) | 
            Instruction::Prefetcht0(_) | Instruction::Prefetcht1(_) | 
            Instruction::Prefetcht2(_) | Instruction::Prefetchw(_) => 3,
            
            _ => 4,
        }
    }
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
    Register(Register),
    Memory(MemoryAddress),
    Immediate(i64),
    Label(String), 
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAddress {
    pub base: Register,
    pub displacement: i64,
}