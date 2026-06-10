use std::collections::HashMap;

use crate::ast::{ConditionCode, Prefix};
use crate::parser::mnemonic::Mnemonic;
use crate::{Lexer, Location, ParserError, Spanned, Token};

use super::ast::{Instruction, Operand, MemoryAddress};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Spanned<Token>>,
    peeked: Option<Spanned<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, source_name: &'a str) -> Self {
        let lexer = Lexer::new(source, source_name);
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
            match instr {
                Instruction::Label(name) => {
                    labels.insert(name, current_offset);
                }
                _ => {
                    let size = instr.estimate_size();
                    instructions.push(instr);
                    current_offset += size;
                }
            }
        }

        Ok((instructions, labels))
    }

    fn parse_instruction(&mut self) -> Result<Option<Instruction>, ParserError> {
        let mut prefixes = Vec::new();

        while let Some(token) = self.peek() {
            match token.value {
                Token::Lock => {
                    prefixes.push(Prefix::Lock);
                    self.advance();
                }
                Token::Rep => {
                    prefixes.push(Prefix::Rep);
                    self.advance();
                }
                Token::Repne => {
                    prefixes.push(Prefix::Repne);
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

        let instr = self.parse_token(token)?;

        if prefixes.is_empty() {
            Ok(Some(instr))
        } else {
            Ok(Some(Instruction::Prefixed(prefixes, Box::new(instr))))
        }
    }

    fn parse_token(&mut self, token: Spanned<Token>) -> Result<Instruction, ParserError> {
        match token.value {
            Token::Mnemonic(mnemonic) => self.parse_mnemonic(mnemonic),
            Token::Label(name) => {
                self.advance();
                self.expect(Token::Colon)?;
                Ok(Instruction::Label(name))
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

    fn parse_mnemonic(&mut self, mnemonic: Mnemonic) -> Result<Instruction, ParserError> {
        match mnemonic {
            Mnemonic::Enter => {
                let imm16 = self.parse_operand()?;
                self.expect_comma()?;
                let imm8 = self.parse_operand()?;
                Ok(Instruction::Enter(imm16, imm8))
            }
            Mnemonic::Leave => Ok(Instruction::Leave),
            Mnemonic::Movsx => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Movsx(dest, src))
            }
            Mnemonic::Movzx => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Movzx(dest, src))
            }
            Mnemonic::Xchg => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Xchg(dest, src))
            }
            Mnemonic::Mul => {
                let op = self.parse_operand()?;
                Ok(Instruction::Mul(op))
            }
            Mnemonic::Imul => {
                let op = self.parse_operand()?;
                Ok(Instruction::Imul(op))
            }
            Mnemonic::Div => {
                let op = self.parse_operand()?;
                Ok(Instruction::Div(op))
            }
            Mnemonic::Idiv => {
                let op = self.parse_operand()?;
                Ok(Instruction::Idiv(op))
            }
            Mnemonic::Shl | Mnemonic::Sal => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Shl(dest, count))
            }
            Mnemonic::Shr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Shr(dest, count))
            }
            Mnemonic::Sar => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Sar(dest, count))
            }
            Mnemonic::Rol => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Rol(dest, count))
            }
            Mnemonic::Ror => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Ror(dest, count))
            }
            Mnemonic::Rcl => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Rcl(dest, count))
            }
            Mnemonic::Rcr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let count = self.parse_operand()?;
                Ok(Instruction::Rcr(dest, count))
            }
            Mnemonic::Bt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Bt(dest, src))
            }
            Mnemonic::Bts => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Bts(dest, src))
            }
            Mnemonic::Btr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Btr(dest, src))
            }
            Mnemonic::Btc => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Btc(dest, src))
            }
            Mnemonic::Bsf => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Bsf(dest, src))
            }
            Mnemonic::Bsr => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Bsr(dest, src))
            }
            Mnemonic::Popcnt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Popcnt(dest, src))
            }
            Mnemonic::Lzcnt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Lzcnt(dest, src))
            }
            Mnemonic::Tzcnt => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Tzcnt(dest, src))
            }
            Mnemonic::Movsb => Ok(Instruction::Movsb),
            Mnemonic::Movsw => Ok(Instruction::Movsw),
            Mnemonic::Movsd => {
                if let Some(Token::Register(reg)) = self.peek().map(|t| &t.value) {
                    if reg.is_xmm() {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Ok(Instruction::Movsd(dest, src))
                    } else {
                        Ok(Instruction::Movs)
                    }
                } else {
                    Ok(Instruction::Movs)
                }
            }
            Mnemonic::Movsq => Ok(Instruction::Movsq),
            Mnemonic::Cmpsb => Ok(Instruction::Cmpsb),
            Mnemonic::Cmpsw => Ok(Instruction::Cmpsw),
            Mnemonic::Cmpsd => Ok(Instruction::Cmpsd),
            Mnemonic::Cmpsq => Ok(Instruction::Cmpsq),
            Mnemonic::Scasb => Ok(Instruction::Scasb),
            Mnemonic::Scasw => Ok(Instruction::Scasw),
            Mnemonic::Scasd => Ok(Instruction::Scasd),
            Mnemonic::Scasq => Ok(Instruction::Scasq),
            Mnemonic::Stosb => Ok(Instruction::Stosb),
            Mnemonic::Stosw => Ok(Instruction::Stosw),
            Mnemonic::Stosd => Ok(Instruction::Stosd),
            Mnemonic::Stosq => Ok(Instruction::Stosq),
            Mnemonic::Lodsb => Ok(Instruction::Lodsb),
            Mnemonic::Lodsw => Ok(Instruction::Lodsw),
            Mnemonic::Lodsd => Ok(Instruction::Lodsd),
            Mnemonic::Lodsq => Ok(Instruction::Lodsq),
            Mnemonic::Mfence => Ok(Instruction::Mfence),
            Mnemonic::Lfence => Ok(Instruction::Lfence),
            Mnemonic::Sfence => Ok(Instruction::Sfence),
            Mnemonic::Syscall => Ok(Instruction::Syscall),
            Mnemonic::Sysenter => Ok(Instruction::Sysenter),
            Mnemonic::Sysexit => Ok(Instruction::Sysexit),
            Mnemonic::Ret => Ok(Instruction::Ret),
            Mnemonic::Nop => Ok(Instruction::Nop),
            Mnemonic::Int3 => Ok(Instruction::Int3),
            Mnemonic::Hlt => Ok(Instruction::Hlt),
            Mnemonic::Cpuid => Ok(Instruction::CpuId),
            Mnemonic::Rdtsc => Ok(Instruction::Rdtsc),
            Mnemonic::Push => {
                let op = self.parse_operand()?;
                Ok(Instruction::Push(op))
            }
            Mnemonic::Pop => {
                let op = self.parse_operand()?;
                Ok(Instruction::Pop(op))
            }
            Mnemonic::Mov => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Mov(dest, src))
            }
            Mnemonic::Sub => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Sub(dest, src))
            }
            Mnemonic::Add => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Add(dest, src))
            }
            Mnemonic::Xor => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Xor(dest, src))
            }
            Mnemonic::And => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::And(dest, src))
            }
            Mnemonic::Or => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Or(dest, src))
            }
            Mnemonic::Inc => {
                let op = self.parse_operand()?;
                Ok(Instruction::Inc(op))
            }
            Mnemonic::Dec => {
                let op = self.parse_operand()?;
                Ok(Instruction::Dec(op))
            }
            Mnemonic::Neg => {
                let op = self.parse_operand()?;
                Ok(Instruction::Neg(op))
            }
            Mnemonic::Not => {
                let op = self.parse_operand()?;
                Ok(Instruction::Not(op))
            }
            Mnemonic::Cmp => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmp(dest, src))
            }
            Mnemonic::Test => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Test(dest, src))
            }
            Mnemonic::Lea => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Lea(dest, src))
            }
            Mnemonic::Jmp => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jmp(target))
            }
            Mnemonic::Je | Mnemonic::Jz => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::E, target))
            }
            Mnemonic::Jne | Mnemonic::Jnz => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::NE, target))
            }
            Mnemonic::Jg => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::G, target))
            }
            Mnemonic::Jge => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::GE, target))
            }
            Mnemonic::Jl => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::L, target))
            }
            Mnemonic::Jle => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::LE, target))
            }
            Mnemonic::Ja => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::A, target))
            }
            Mnemonic::Jb => {
                let target = self.parse_operand()?;
                Ok(Instruction::Jcc(ConditionCode::B, target))
            }
            Mnemonic::Call => {
                let target = self.parse_operand()?;
                Ok(Instruction::Call(target))
            }
            Mnemonic::Cmove | Mnemonic::Cmovz => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmove(dest, src))
            }
            Mnemonic::Cmovne | Mnemonic::Cmovnz => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovne(dest, src))
            }
            Mnemonic::Cmovg => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovg(dest, src))
            }
            Mnemonic::Cmovge => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovge(dest, src))
            }
            Mnemonic::Cmovl => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovl(dest, src))
            }
            Mnemonic::Cmovle => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovle(dest, src))
            }
            Mnemonic::Cmova => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmova(dest, src))
            }
            Mnemonic::Cmovae => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovae(dest, src))
            }
            Mnemonic::Cmovb => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovb(dest, src))
            }
            Mnemonic::Cmovbe => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovbe(dest, src))
            }
            Mnemonic::Cmovs => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovs(dest, src))
            }
            Mnemonic::Cmovns => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Cmovns(dest, src))
            }
            Mnemonic::Movaps => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Movaps(dest, src))
            }
            Mnemonic::Movapd => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Movapd(dest, src))
            }
            Mnemonic::Addpd => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Addpd(dest, src))
            }
            Mnemonic::Addps => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src = self.parse_operand()?;
                Ok(Instruction::Addps(dest, src))
            }
            Mnemonic::Vaddpd => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src1 = self.parse_operand()?;
                self.expect_comma()?;
                let src2 = self.parse_operand()?;
                Ok(Instruction::Vaddpd(dest, src1, src2))
            }
            Mnemonic::Vaddps => {
                let dest = self.parse_operand()?;
                self.expect_comma()?;
                let src1 = self.parse_operand()?;
                self.expect_comma()?;
                let src2 = self.parse_operand()?;
                Ok(Instruction::Vaddps(dest, src1, src2))
            }
            Mnemonic::Prefetch => {
                let addr = self.parse_operand()?;
                Ok(Instruction::Prefetch(addr))
            }
            Mnemonic::Prefetchnta => {
                let addr = self.parse_operand()?;
                Ok(Instruction::Prefetchnta(addr))
            }
            Mnemonic::Prefetcht0 => {
                let addr = self.parse_operand()?;
                Ok(Instruction::Prefetcht0(addr))
            }
            Mnemonic::Prefetcht1 => {
                let addr = self.parse_operand()?;
                Ok(Instruction::Prefetcht1(addr))
            }
            Mnemonic::Prefetcht2 => {
                let addr = self.parse_operand()?;
                Ok(Instruction::Prefetcht2(addr))
            }
            Mnemonic::Prefetchw => {
                let addr = self.parse_operand()?;
                Ok(Instruction::Prefetchw(addr))
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

        let size = match self.peek() {
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
                    value: Operand::Label(label),
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