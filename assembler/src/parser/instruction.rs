use crate::{Register, Spanned, ast::{ConditionCode, Operand, PrefixSet}};

pub struct Instruction {
    pub prefixes: PrefixSet,
    pub kind: InstructionKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionKind {
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

    // Prefixed(Vec<Prefix>, Box<Instruction>),
    // Prefixed(PrefixedInstruction),
    Label(String),

    Prefetch(Spanned<Operand>),
    Prefetchnta(Spanned<Operand>),
    Prefetcht0(Spanned<Operand>),
    Prefetcht1(Spanned<Operand>),
    Prefetcht2(Spanned<Operand>),
    Prefetchw(Spanned<Operand>),  // AMD
}

impl Instruction {
    pub fn new(prefixes: PrefixSet, kind: InstructionKind) -> Self {
        Self { prefixes, kind }
    }

    pub fn estimate_size(&self) -> usize {
        let prefix_size = self.prefixes.count();
        
        let kind_size = match &self.kind {
            InstructionKind::Syscall | InstructionKind::Ret | InstructionKind::Nop | InstructionKind::Int3 | 
            InstructionKind::Hlt | InstructionKind::CpuId | InstructionKind::Rdtsc | InstructionKind::Leave => 2,
            
            InstructionKind::Push(_) | InstructionKind::Pop(_) | InstructionKind::Inc(_) | InstructionKind::Dec(_) |
            InstructionKind::Neg(_) | InstructionKind::Not(_) | InstructionKind::Mul(_) | InstructionKind::Imul(_) |
            InstructionKind::Div(_) | InstructionKind::Idiv(_) => 3,
            
            InstructionKind::Mov(dest, src) => {
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
            
            InstructionKind::Add(dest, _) | InstructionKind::Sub(dest, _) |
            InstructionKind::Xor(dest, _) | InstructionKind::And(dest, _) | InstructionKind::Or(dest, _) |
            InstructionKind::Cmp(dest, _) | InstructionKind::Test(dest, _) | InstructionKind::Lea(dest, _) => {
                match dest.value {
                    Operand::Register(_) => 4,
                    Operand::Memory(_) => 5,
                    _ => 4,
                }
            }
            
            InstructionKind::Jmp(_) | InstructionKind::Call(_) => 5,
            InstructionKind::Jcc(_, _) => 6,
            
            InstructionKind::Shl(_, _) | InstructionKind::Shr(_, _) | InstructionKind::Sar(_, _) |
            InstructionKind::Rol(_, _) | InstructionKind::Ror(_, _) | InstructionKind::Rcl(_, _) |
            InstructionKind::Rcr(_, _) => 4,
            
            InstructionKind::Bt(_, _) | InstructionKind::Bts(_, _) | InstructionKind::Btr(_, _) |
            InstructionKind::Btc(_, _) => 5,
            
            InstructionKind::Bsf(_, _) | InstructionKind::Bsr(_, _) | InstructionKind::Popcnt(_, _) |
            InstructionKind::Lzcnt(_, _) | InstructionKind::Tzcnt(_, _) => 5,
            
            InstructionKind::Cmove(_, _) | InstructionKind::Cmovne(_, _) | InstructionKind::Cmovg(_, _) |
            InstructionKind::Cmovge(_, _) | InstructionKind::Cmovl(_, _) | InstructionKind::Cmovle(_, _) |
            InstructionKind::Cmova(_, _) | InstructionKind::Cmovae(_, _) | InstructionKind::Cmovb(_, _) |
            InstructionKind::Cmovbe(_, _) | InstructionKind::Cmovs(_, _) | InstructionKind::Cmovns(_, _) |
            InstructionKind::Cmovz(_, _) | InstructionKind::Cmovnz(_, _) => 5,
            
            InstructionKind::Movsb | InstructionKind::Movsw | InstructionKind::Movsq |
            InstructionKind::Cmpsb | InstructionKind::Cmpsw | InstructionKind::Cmpsd | InstructionKind::Cmpsq |
            InstructionKind::Scasb | InstructionKind::Scasw | InstructionKind::Scasd | InstructionKind::Scasq |
            InstructionKind::Stosb | InstructionKind::Stosw | InstructionKind::Stosd | InstructionKind::Stosq |
            InstructionKind::Lodsb | InstructionKind::Lodsw | InstructionKind::Lodsd | InstructionKind::Lodsq => 2,
            
            InstructionKind::Movs => 2,
            
            InstructionKind::Mfence | InstructionKind::Lfence | InstructionKind::Sfence => 3,
            
            InstructionKind::Enter(_, _) => 4,
            
            InstructionKind::Movsx(_, _) | InstructionKind::Movzx(_, _) | InstructionKind::Xchg(_, _) => 4,
            
            InstructionKind::Vaddpd(_, _, _) | InstructionKind::Vaddps(_, _, _) => 4,
            
            InstructionKind::Movsd(_, _) | InstructionKind::Movss(_, _) | InstructionKind::Movaps(_, _) |
            InstructionKind::Movapd(_, _) | InstructionKind::Addpd(_, _) | InstructionKind::Addps(_, _) |
            InstructionKind::AddSd(_, _) | InstructionKind::AddSs(_, _) => 4,
            
            InstructionKind::Label(_) => 0,
            
            InstructionKind::Prefetch(_) | InstructionKind::Prefetchnta(_) | 
            InstructionKind::Prefetcht0(_) | InstructionKind::Prefetcht1(_) | 
            InstructionKind::Prefetcht2(_) | InstructionKind::Prefetchw(_) => 3,
            
            _ => 4,
        };
        
        prefix_size + kind_size
    }
}
