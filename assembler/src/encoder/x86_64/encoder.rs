use crate::{Instruction, InstructionKind, Spanned, ast::{ConditionCode, Prefix}, encoder::{EncodingError, buffer::InstrBuf, x86_64::instr}, parser::ast::Operand, symbol::SymbolResolver};

pub struct Encoder;

impl Encoder {
    pub fn encode(instr: &Instruction) -> Result<InstrBuf, EncodingError> {
        let mut buf = InstrBuf::new();
        
        for prefix in instr.prefixes.iter() {
            let byte = match prefix {
                Prefix::Lock => 0xF0,
                Prefix::Rep => 0xF3,
                Prefix::Repne => 0xF2,
            };
            buf.push(byte);
        }
        
        let kind_buf = Self::encode_kind(&instr.kind)?;
        buf.extend_from_slice(kind_buf.as_ref());
        
        Ok(buf)
    }

    pub fn encode_kind(kind: &InstructionKind) -> Result<InstrBuf, EncodingError> {
        match kind {
            InstructionKind::Syscall => Ok(instr::system::syscall()),
            InstructionKind::Sysenter => Ok(instr::system::sysenter()),
            InstructionKind::Sysexit => Ok(instr::system::sysexit()),
            InstructionKind::Ret => Ok(instr::system::ret()),
            InstructionKind::Nop => Ok(instr::system::nop()),
            InstructionKind::CpuId => Ok(instr::system::cpuid()),
            InstructionKind::Hlt => Ok(instr::system::hlt()),
            InstructionKind::Int3 => Ok(instr::system::int3()),
            InstructionKind::Rdtsc => Ok(instr::system::rdtsc()),
            InstructionKind::Leave => Ok(instr::system::leave()),
            
            InstructionKind::Push(op) => instr::stack::push(op),
            InstructionKind::Pop(op) => instr::stack::pop(op),
            
            InstructionKind::Jmp(target) => instr::control_flow::jmp(target),
            InstructionKind::Call(target) => instr::control_flow::call(target),
            InstructionKind::Jcc(cc, target) => instr::control_flow::jcc(cc, target),
            
            InstructionKind::Mov(dest, src) => instr::data::mov(dest, src),
            InstructionKind::Movsx(dest, src) => instr::data::movsx(dest, src),
            InstructionKind::Movzx(dest, src) => instr::data::movzx(dest, src),
            InstructionKind::Lea(dest, src) => instr::data::lea(dest, src),
            InstructionKind::Xchg(dest, src) => instr::data::xchg(dest, src),
            
            InstructionKind::Add(dest, src) => instr::arithmetic::add(dest, src),
            InstructionKind::Sub(dest, src) => instr::arithmetic::sub(dest, src),
            InstructionKind::Mul(op) => instr::arithmetic::mul(op),
            InstructionKind::Imul(op) => instr::arithmetic::imul(op),
            InstructionKind::Div(op) => instr::arithmetic::div(op),
            InstructionKind::Idiv(op) => instr::arithmetic::idiv(op),
            InstructionKind::Inc(op) => instr::arithmetic::inc(op),
            InstructionKind::Dec(op) => instr::arithmetic::dec(op),
            InstructionKind::Neg(op) => instr::arithmetic::neg(op),
            InstructionKind::Not(op) => instr::arithmetic::not(op),
            
            InstructionKind::And(dest, src) => instr::logical::and(dest, src),
            InstructionKind::Or(dest, src) => instr::logical::or(dest, src),
            InstructionKind::Xor(dest, src) => instr::logical::xor(dest, src),
            InstructionKind::Test(dest, src) => instr::logical::test(dest, src),
            
            InstructionKind::Cmp(dest, src) => instr::compare::cmp(dest, src),
            
            InstructionKind::Shl(dest, count) => instr::shift::shl(dest, count),
            InstructionKind::Shr(dest, count) => instr::shift::shr(dest, count),
            InstructionKind::Sar(dest, count) => instr::shift::sar(dest, count),
            InstructionKind::Rol(dest, count) => instr::shift::rol(dest, count),
            InstructionKind::Ror(dest, count) => instr::shift::ror(dest, count),
            InstructionKind::Rcl(dest, count) => instr::shift::rcl(dest, count),
            InstructionKind::Rcr(dest, count) => instr::shift::rcr(dest, count),
            
            InstructionKind::Bt(dest, src) => instr::bit::bt(dest, src),
            InstructionKind::Bts(dest, src) => instr::bit::bts(dest, src),
            InstructionKind::Btr(dest, src) => instr::bit::btr(dest, src),
            InstructionKind::Btc(dest, src) => instr::bit::btc(dest, src),
            InstructionKind::Bsf(dest, src) => instr::bit::bsf(dest, src),
            InstructionKind::Bsr(dest, src) => instr::bit::bsr(dest, src),
            InstructionKind::Popcnt(dest, src) => instr::bit::popcnt(dest, src),
            InstructionKind::Lzcnt(dest, src) => instr::bit::lzcnt(dest, src),
            InstructionKind::Tzcnt(dest, src) => instr::bit::tzcnt(dest, src),
            
            InstructionKind::Cmove(dest, src) => instr::cmov::cmove(dest, src),
            InstructionKind::Cmovne(dest, src) => instr::cmov::cmovne(dest, src),
            InstructionKind::Cmovg(dest, src) => instr::cmov::cmovg(dest, src),
            InstructionKind::Cmovge(dest, src) => instr::cmov::cmovge(dest, src),
            InstructionKind::Cmovl(dest, src) => instr::cmov::cmovl(dest, src),
            InstructionKind::Cmovle(dest, src) => instr::cmov::cmovle(dest, src),
            InstructionKind::Cmova(dest, src) => instr::cmov::cmova(dest, src),
            InstructionKind::Cmovae(dest, src) => instr::cmov::cmovae(dest, src),
            InstructionKind::Cmovb(dest, src) => instr::cmov::cmovb(dest, src),
            InstructionKind::Cmovbe(dest, src) => instr::cmov::cmovbe(dest, src),
            InstructionKind::Cmovs(dest, src) => instr::cmov::cmovs(dest, src),
            InstructionKind::Cmovns(dest, src) => instr::cmov::cmovns(dest, src),
            
            InstructionKind::Movsb => instr::string::movsb(),
            InstructionKind::Movsw => instr::string::movsw(),
            InstructionKind::Movsq => instr::string::movsq(),
            InstructionKind::Movs => instr::string::movs(),
            InstructionKind::Cmpsb => instr::string::cmpsb(),
            InstructionKind::Cmpsw => instr::string::cmpsw(),
            InstructionKind::Cmpsd => instr::string::cmpsd(),
            InstructionKind::Cmpsq => instr::string::cmpsq(),
            InstructionKind::Scasb => instr::string::scasb(),
            InstructionKind::Scasw => instr::string::scasw(),
            InstructionKind::Scasd => instr::string::scasd(),
            InstructionKind::Scasq => instr::string::scasq(),
            InstructionKind::Stosb => instr::string::stosb(),
            InstructionKind::Stosw => instr::string::stosw(),
            InstructionKind::Stosd => instr::string::stosd(),
            InstructionKind::Stosq => instr::string::stosq(),
            InstructionKind::Lodsb => instr::string::lodsb(),
            InstructionKind::Lodsw => instr::string::lodsw(),
            InstructionKind::Lodsd => instr::string::lodsd(),
            InstructionKind::Lodsq => instr::string::lodsq(),
            
            InstructionKind::Mfence => Ok(instr::fence::mfence()),
            InstructionKind::Lfence => instr::fence::lfence(),
            InstructionKind::Sfence => instr::fence::sfence(),
            
            InstructionKind::Movsd(dest, src) => instr::simd::movsd(dest, src),
            InstructionKind::Movss(dest, src) => instr::simd::movss(dest, src),
            InstructionKind::Movaps(dest, src) => instr::simd::movaps(dest, src),
            InstructionKind::Movapd(dest, src) => instr::simd::movapd(dest, src),
            InstructionKind::Addpd(dest, src) => instr::simd::addpd(dest, src),
            InstructionKind::Addps(dest, src) => instr::simd::addps(dest, src),
            InstructionKind::AddSd(dest, src) => instr::simd::addsd(dest, src),
            InstructionKind::AddSs(dest, src) => instr::simd::addss(dest, src),
            InstructionKind::Vaddpd(dest, src1, src2) => instr::simd::vaddpd(dest, src1, src2),
            InstructionKind::Vaddps(dest, src1, src2) => instr::simd::vaddps(dest, src1, src2),
            
            InstructionKind::Enter(imm16, imm8) => instr::stack_frame::enter(imm16, imm8),
            
            InstructionKind::Prefetch(addr) => instr::prefetch::prefetch(addr),
            InstructionKind::Prefetchnta(addr) => instr::prefetch::prefetchnta(addr),
            InstructionKind::Prefetcht0(addr) => instr::prefetch::prefetcht0(addr),
            InstructionKind::Prefetcht1(addr) => instr::prefetch::prefetcht1(addr),
            InstructionKind::Prefetcht2(addr) => instr::prefetch::prefetcht2(addr),
            InstructionKind::Prefetchw(addr) => instr::prefetch::prefetchw(addr),
            
            _ => Err(EncodingError::Unknown),
        }
    }
    
    pub fn encode_with_labels<S: SymbolResolver>(instr: &Instruction, symbols: &S, offset: u64) -> Result<InstrBuf, EncodingError> {
        match &instr.kind {
            InstructionKind::Jmp(Spanned { value: Operand::Label(name), .. }) => {
                Ok(instr::control_flow::jmp_with_label(name, symbols, offset))
            }
            InstructionKind::Call(Spanned { value: Operand::Label(name), .. }) => {
                let mut buf = InstrBuf::new();
                buf.push(0xE8);
                if let Some(addr) = symbols.lookup(name) {
                    let rel = (addr as i64) - (offset as i64 + 5);
                    buf.push_u32(rel as u32);
                } else {
                    buf.push_u32(0);
                }
                Ok(buf)
            }
            InstructionKind::Jcc(cc, Spanned { value: Operand::Label(name), .. }) => {
                let opcode = match cc {
                    ConditionCode::E | ConditionCode::Z => 0x84,
                    ConditionCode::NE | ConditionCode::NZ => 0x85,
                    ConditionCode::G => 0x8F,
                    ConditionCode::GE => 0x8D,
                    ConditionCode::L => 0x8C,
                    ConditionCode::LE => 0x8E,
                    ConditionCode::A => 0x87,
                    ConditionCode::AE => 0x83,
                    ConditionCode::B => 0x82,
                    ConditionCode::BE => 0x86,
                    _ => 0x84,
                };
                let mut buf = InstrBuf::new();
                buf.push(0x0F).push(opcode);
                if let Some(addr) = symbols.lookup(name) {
                    let rel = (addr as i64) - (offset as i64 + 6);
                    buf.push_u32(rel as u32);
                } else {
                    buf.push_u32(0);
                }
                Ok(buf)
            }
            _ => Self::encode(instr),
        }
    }
}