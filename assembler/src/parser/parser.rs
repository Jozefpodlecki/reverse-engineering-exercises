use std::collections::HashMap;

use crate::ast::{ConditionCode, Prefix, PrefixSet};
use crate::parser::mnemonic::Mnemonic;
use crate::{Instruction, InstructionKind, Lexer, Location, ParserError, Spanned, Token};

use super::ast::{Operand, MemoryAddress};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Spanned<Token>>,
    peeked: Option<Spanned<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let lexer = Lexer::new(source);
        let mut parser = Self {
            lexer,
            current: None,
            peeked: None,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        if let Some(peeked) = self.peeked.take() {
            self.current = Some(peeked);
            return;
        }

        match self.lexer.next() {
            Some(Ok(token)) => {
                if matches!(token.value, Token::Eof) {
                    self.current = None;
                } else {
                    self.current = Some(token);
                }
            }
            Some(Err(err)) => {
                self.current = None;
            }
            None => {
                self.current = None;
            }
        }
    }

    fn peek(&mut self) -> Option<&Spanned<Token>> {
        if self.peeked.is_none() && self.current.is_some() {
            match self.lexer.next() {
                Some(Ok(token)) => self.peeked = Some(token),
                Some(Err(_)) => {}
                None => {}
            }
        }
        self.peeked.as_ref().or(self.current.as_ref())
    }

    fn current_location(&self) -> Location {
        self.current.as_ref().map(|t| t.location.clone()).unwrap_or(Location { line: 0, col: 0 })
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        match self.peek() {
            Some(token) if token.value == expected => {
                self.advance();
                Ok(())
            }
            Some(token) => {
                let loc = token.location.clone();
                 Err(ParserError::ExpectedToken {
                    expected,
                    found: token.value.clone(),
                    line: loc.line,
                    col: loc.col,
                })
            }
            None => {
                let loc = self.current_location();
                Err(ParserError::UnexpectedEof {
                    expected,
                    line: loc.line,
                    col: loc.col,
                })
            }
        }
    }

    fn expect_comma(&mut self) -> Result<(), ParserError> {
        self.expect(Token::Comma)
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>, ParserError> {
        let mut instructions = Vec::new();
        while let Some(instr) = self.parse_instruction()? {
            instructions.push(instr);
        }
        Ok(instructions)
    }

    pub fn parse_with_labels(&mut self) -> Result<(Vec<Instruction>, HashMap<String, usize>), ParserError> {
        let mut instructions = Vec::new();
        let mut labels = HashMap::new();
        let mut current_offset = 0usize;

        while let Some(instr) = self.parse_instruction()? {
            if let InstructionKind::Label(name) = &instr.kind {
                labels.insert(name.clone(), current_offset);
            } else {
                let size = instr.estimate_size();
                instructions.push(instr);
                current_offset += size;
            }
        }

        Ok((instructions, labels))
    }

    fn parse_instruction(&mut self) -> Result<Option<Instruction>, ParserError> {
        let mut prefixes = PrefixSet::new();

        while let Some(token) = self.peek() {
            match token.value {
                Token::Lock => {
                    prefixes.set_lock();
                    self.advance();
                }
                Token::Rep => {
                    prefixes.set_rep();
                    self.advance();
                }
                Token::Repne => {
                    prefixes.set_repne();
                    self.advance();
                }
                _ => break,
            }
        }

        let token = match self.current.clone() {
            Some(t) => t,
            None => return Ok(None),
        };

        if matches!(token.value, Token::Eof) {
            return Ok(None);
        }

        let kind = self.parse_token(token)?;
        
        Ok(Some(Instruction::new(prefixes, kind)))
    }

    fn parse_token(&mut self, token: Spanned<Token>) -> Result<InstructionKind, ParserError> {
        match token.value {
            Token::Mnemonic(mnemonic) => self.parse_mnemonic(mnemonic),
            Token::Label(name) => {
                self.advance();
                self.expect(Token::Colon)?;
                Ok(InstructionKind::Label(name.to_string()))
            }
            _ => {
                let loc = token.location;
                Err(ParserError::ExpectedInstruction {
                    found: token.value.clone(),
                    line: loc.line,
                    col: loc.col,
                })
            }
        }
    }

    fn parse_mnemonic(&mut self, mnemonic: Mnemonic) -> Result<InstructionKind, ParserError> {
        match mnemonic {
            Mnemonic::Enter => {
                let imm16 = self.parse_operand()?;
                self.expect_comma()?;
                let imm8 = self.parse_operand()?;
                Ok(InstructionKind::Enter(imm16, imm8))
            }
            Mnemonic::Leave => Ok(InstructionKind::Leave),
            Mnemonic::Movsx => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Movsx(dest, src))
            }
            Mnemonic::Movzx => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Movzx(dest, src))
            }
            Mnemonic::Xchg => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Xchg(dest, src))
            }
            Mnemonic::Mul => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Mul(op))
            }
            Mnemonic::Imul => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Imul(op))
            }
            Mnemonic::Div => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Div(op))
            }
            Mnemonic::Idiv => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Idiv(op))
            }
            Mnemonic::Shl | Mnemonic::Sal => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Shl(dest, count))
            }
            Mnemonic::Shr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Shr(dest, count))
            }
            Mnemonic::Sar => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Sar(dest, count))
            }
            Mnemonic::Rol => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Rol(dest, count))
            }
            Mnemonic::Ror => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Ror(dest, count))
            }
            Mnemonic::Rcl => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Rcl(dest, count))
            }
            Mnemonic::Rcr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(InstructionKind::Rcr(dest, count))
            }
            Mnemonic::Bt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Bt(dest, src))
            }
            Mnemonic::Bts => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Bts(dest, src))
            }
            Mnemonic::Btr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Btr(dest, src))
            }
            Mnemonic::Btc => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Btc(dest, src))
            }
            Mnemonic::Bsf => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Bsf(dest, src))
            }
            Mnemonic::Bsr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Bsr(dest, src))
            }
            Mnemonic::Popcnt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Popcnt(dest, src))
            }
            Mnemonic::Lzcnt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Lzcnt(dest, src))
            }
            Mnemonic::Tzcnt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Tzcnt(dest, src))
            }
            Mnemonic::Movsb => Ok(InstructionKind::Movsb),
            Mnemonic::Movsw => Ok(InstructionKind::Movsw),
            Mnemonic::Movsd => {
                if let Some(Token::Register(reg)) = self.peek().map(|t| &t.value) {
                    if reg.is_xmm() {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Ok(InstructionKind::Movsd(dest, src))
                    } else {
                        Ok(InstructionKind::Movs)
                    }
                } else {
                    Ok(InstructionKind::Movs)
                }
            }
            Mnemonic::Movsq => Ok(InstructionKind::Movsq),
            Mnemonic::Cmpsb => Ok(InstructionKind::Cmpsb),
            Mnemonic::Cmpsw => Ok(InstructionKind::Cmpsw),
            Mnemonic::Cmpsd => Ok(InstructionKind::Cmpsd),
            Mnemonic::Cmpsq => Ok(InstructionKind::Cmpsq),
            Mnemonic::Scasb => Ok(InstructionKind::Scasb),
            Mnemonic::Scasw => Ok(InstructionKind::Scasw),
            Mnemonic::Scasd => Ok(InstructionKind::Scasd),
            Mnemonic::Scasq => Ok(InstructionKind::Scasq),
            Mnemonic::Stosb => Ok(InstructionKind::Stosb),
            Mnemonic::Stosw => Ok(InstructionKind::Stosw),
            Mnemonic::Stosd => Ok(InstructionKind::Stosd),
            Mnemonic::Stosq => Ok(InstructionKind::Stosq),
            Mnemonic::Lodsb => Ok(InstructionKind::Lodsb),
            Mnemonic::Lodsw => Ok(InstructionKind::Lodsw),
            Mnemonic::Lodsd => Ok(InstructionKind::Lodsd),
            Mnemonic::Lodsq => Ok(InstructionKind::Lodsq),
            Mnemonic::Mfence => Ok(InstructionKind::Mfence),
            Mnemonic::Lfence => Ok(InstructionKind::Lfence),
            Mnemonic::Sfence => Ok(InstructionKind::Sfence),
            Mnemonic::Syscall => Ok(InstructionKind::Syscall),
            Mnemonic::Sysenter => Ok(InstructionKind::Sysenter),
            Mnemonic::Sysexit => Ok(InstructionKind::Sysexit),
            Mnemonic::Ret => Ok(InstructionKind::Ret),
            Mnemonic::Nop => Ok(InstructionKind::Nop),
            Mnemonic::Int3 => Ok(InstructionKind::Int3),
            Mnemonic::Hlt => Ok(InstructionKind::Hlt),
            Mnemonic::Cpuid => Ok(InstructionKind::CpuId),
            Mnemonic::Rdtsc => Ok(InstructionKind::Rdtsc),
            Mnemonic::Push => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Push(op))
            }
            Mnemonic::Pop => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Pop(op))
            }
            Mnemonic::Mov => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Mov(dest, src))
            }
            Mnemonic::Sub => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Sub(dest, src))
            }
            Mnemonic::Add => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Add(dest, src))
            }
            Mnemonic::Xor => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Xor(dest, src))
            }
            Mnemonic::And => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::And(dest, src))
            }
            Mnemonic::Or => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Or(dest, src))
            }
            Mnemonic::Inc => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Inc(op))
            }
            Mnemonic::Dec => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Dec(op))
            }
            Mnemonic::Neg => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Neg(op))
            }
            Mnemonic::Not => {
                let op = self.parse_operand()?;
                Ok(InstructionKind::Not(op))
            }
            Mnemonic::Cmp => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmp(dest, src))
            }
            Mnemonic::Test => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Test(dest, src))
            }
            Mnemonic::Lea => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Lea(dest, src))
            }
            Mnemonic::Jmp => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jmp(target))
            }
            Mnemonic::Je | Mnemonic::Jz => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::E, target))
            }
            Mnemonic::Jne | Mnemonic::Jnz => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::NE, target))
            }
            Mnemonic::Jg => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::G, target))
            }
            Mnemonic::Jge => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::GE, target))
            }
            Mnemonic::Jl => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::L, target))
            }
            Mnemonic::Jle => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::LE, target))
            }
            Mnemonic::Ja => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::A, target))
            }
            Mnemonic::Jb => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Jcc(ConditionCode::B, target))
            }
            Mnemonic::Call => {
                let target = self.parse_operand()?;
                Ok(InstructionKind::Call(target))
            }
            Mnemonic::Cmove | Mnemonic::Cmovz => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmove(dest, src))
            }
            Mnemonic::Cmovne | Mnemonic::Cmovnz => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovne(dest, src))
            }
            Mnemonic::Cmovg => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovg(dest, src))
            }
            Mnemonic::Cmovge => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovge(dest, src))
            }
            Mnemonic::Cmovl => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovl(dest, src))
            }
            Mnemonic::Cmovle => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovle(dest, src))
            }
            Mnemonic::Cmova => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmova(dest, src))
            }
            Mnemonic::Cmovae => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovae(dest, src))
            }
            Mnemonic::Cmovb => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovb(dest, src))
            }
            Mnemonic::Cmovbe => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovbe(dest, src))
            }
            Mnemonic::Cmovs => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovs(dest, src))
            }
            Mnemonic::Cmovns => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Cmovns(dest, src))
            }
            Mnemonic::Movaps => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Movaps(dest, src))
            }
            Mnemonic::Movapd => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Movapd(dest, src))
            }
            Mnemonic::Addpd => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Addpd(dest, src))
            }
            Mnemonic::Addps => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(InstructionKind::Addps(dest, src))
            }
            Mnemonic::Vaddpd => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src1 = self.parse_operand()?;
                self.expect_comma()?;
                let src2 = self.parse_operand()?;
                Ok(InstructionKind::Vaddpd(dest, src1, src2))
            }
            Mnemonic::Vaddps => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src1 = self.parse_operand()?;
                self.expect_comma()?;
                let src2 = self.parse_operand()?;
                Ok(InstructionKind::Vaddps(dest, src1, src2))
            }
            Mnemonic::Prefetch => {
                let addr = self.parse_operand()?;
                Ok(InstructionKind::Prefetch(addr))
            }
            Mnemonic::Prefetchnta => {
                let addr = self.parse_operand()?;
                Ok(InstructionKind::Prefetchnta(addr))
            }
            Mnemonic::Prefetcht0 => {
                let addr = self.parse_operand()?;
                Ok(InstructionKind::Prefetcht0(addr))
            }
            Mnemonic::Prefetcht1 => {
                let addr = self.parse_operand()?;
                Ok(InstructionKind::Prefetcht1(addr))
            }
            Mnemonic::Prefetcht2 => {
                let addr = self.parse_operand()?;
                Ok(InstructionKind::Prefetcht2(addr))
            }
            Mnemonic::Prefetchw => {
                let addr = self.parse_operand()?;
                Ok(InstructionKind::Prefetchw(addr))
            }
            _ => {
                let loc = self.current_location();
                Err(ParserError::UnknownMnemonic {
                    mnemonic,
                    line: loc.line,
                    col: loc.col,
                })
            }
        }
    }

    fn parse_operand(&mut self) -> Result<Spanned<Operand>, ParserError> {
        let start_loc = self.current_location();

        let _size = match self.peek() {
            Some(token) if token.value == Token::Byte => {
                self.advance();
                Some(1)
            }
            Some(token) if token.value == Token::Word => {
                self.advance();
                Some(2)
            }
            Some(token) if token.value == Token::Dword => {
                self.advance();
                Some(4)
            }
            Some(token) if token.value == Token::Qword => {
                self.advance();
                Some(8)
            }
            _ => None,
        };

        let token = match self.current.clone() {
            Some(t) => t,
            None => {
                let loc = self.current_location();
                return Err(ParserError::UnexpectedEof {
                    expected: Token::Immediate(0),
                    line: loc.line,
                    col: loc.col,
                })
            }
        };

        match token.value {
            Token::Register(reg) => {
                self.advance();
                Ok(Spanned {
                    value: Operand::Register(reg),
                    location: start_loc,
                })
            }
            Token::Immediate(imm) => {
                self.advance();
                Ok(Spanned {
                    value: Operand::Immediate(imm),
                    location: start_loc,
                })
            }
            Token::Label(label) => {
                self.advance();
                Ok(Spanned {
                    value: Operand::Label(label.to_string()),
                    location: start_loc,
                })
            }
            Token::OpenBracket => {
                let mem = self.parse_memory_address()?;
                Ok(mem)
            }
            _ => {
                let loc = token.location;
                Err(ParserError::ExpectedOperand {
                    found: Token::Eof,
                    line: loc.line,
                    col: loc.col,
                })
            }
        }
    }

    fn parse_memory_address(&mut self) -> Result<Spanned<Operand>, ParserError> {
        let start_loc = self.current_location();
        self.expect(Token::OpenBracket)?;

        let base = match self.current.clone() {
            Some(Spanned { value: Token::Register(reg), .. }) => {
                self.advance();
                reg
            }
            _ => {
                let loc = self.current_location();
                return Err(ParserError::ExpectedBaseRegister {
                    line: loc.line,
                    col: loc.col,
                })
            }
        };

        let mut displacement = 0;
        let mut sign = 1;

        if let Some(token) = self.peek() {
            if token.value == Token::Plus {
                self.advance();
                sign = 1;
            } else if token.value == Token::Minus {
                self.advance();
                sign = -1;
            }
        }

        if let Some(Spanned { value: Token::Immediate(imm), .. }) = self.current.clone() {
            self.advance();
            displacement = sign * imm;
        }

        self.expect(Token::CloseBracket)?;

        Ok(Spanned {
            value: Operand::Memory(MemoryAddress { base, displacement }),
            location: start_loc,
        })
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Instruction, ParserError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.parse_instruction() {
            Ok(Some(instr)) => Some(Ok(instr)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}