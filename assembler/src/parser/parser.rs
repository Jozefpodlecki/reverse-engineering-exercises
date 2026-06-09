use crate::ast::{ConditionCode, Prefix};
use crate::parser::token;

use super::token::{Token, Spanned};
use super::ast::{Instruction, Operand, MemoryAddress};

pub struct Parser {
    tokens: Vec<Spanned<Token>>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Self {
        Self { tokens, index: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Instruction>, (String, usize, usize)> {
        let mut instructions = Vec::new();
        
        while let Some(instr) = self.parse_instruction()? {
            instructions.push(instr);
        }
        
        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<Option<Instruction>, (String, usize, usize)> {
        let mut prefixes = Vec::new();
        
        while let Some(token) = self.peek() {
            match token {
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
        
        let token = match self.tokens.get(self.index) {
            Some(t) => t.value.clone(),
            None => Token::Eof,
        };
        
        if matches!(token, Token::Eof) {
            return Ok(None);
        }
    
        let instr = match self.parse_token(token)? {
            Some(i) => i,
            None => return Ok(None),
        };
        
        if prefixes.is_empty() {
            Ok(Some(instr))
        } else {
            Ok(Some(Instruction::Prefixed(prefixes, Box::new(instr))))
        }
    }
    
    fn parse_token(&mut self, token: Token) -> Result<Option<Instruction>, (String, usize, usize)> {

        match token {
            Token::Mnemonic(mnemonic) => {
                let start_loc = self.current_location();
                self.advance();
                
                let instr = match mnemonic.as_str() {
                    "enter" => {
                        let imm16 = self.parse_operand()?;
                        self.expect_comma()?;
                        let imm8 = self.parse_operand()?;
                        Instruction::Enter(imm16, imm8)
                    }
                    "leave" => Instruction::Leave,

                    "movsx" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Movsx(dest, src)
                    }
                    "movzx" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Movzx(dest, src)
                    }
                    "xchg" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Xchg(dest, src)
                    }

                    "mul" => {
                        let op = self.parse_operand()?;
                        Instruction::Mul(op)
                    }
                    "imul" => {
                        let op = self.parse_operand()?;
                        Instruction::Imul(op)
                    }
                    "div" => {
                        let op = self.parse_operand()?;
                        Instruction::Div(op)
                    }
                    "idiv" => {
                        let op = self.parse_operand()?;
                        Instruction::Idiv(op)
                    }

                    "shl" | "sal" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Shl(dest, count)
                    }
                    "shr" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Shr(dest, count)
                    }
                    "sar" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Sar(dest, count)
                    }
                    "rol" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Rol(dest, count)
                    }
                    "ror" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Ror(dest, count)
                    }
                    "rcl" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Rcl(dest, count)
                    }
                    "rcr" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let count = self.parse_operand()?;
                        Instruction::Rcr(dest, count)
                    }

                    "bt" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Bt(dest, src)
                    }
                    "bts" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Bts(dest, src)
                    }
                    "btr" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Btr(dest, src)
                    }
                    "btc" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Btc(dest, src)
                    }

                    "bsf" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Bsf(dest, src)
                    }
                    "bsr" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Bsr(dest, src)
                    }
                    "popcnt" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Popcnt(dest, src)
                    }
                    "lzcnt" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Lzcnt(dest, src)
                    }
                    "tzcnt" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Tzcnt(dest, src)
                    }

                    "movsb" => Instruction::Movsb,
                    "movsw" => Instruction::Movsw,
                    "movsd" => {
                        if let Some(Token::XmmRegister(_)) = self.peek() {
                            // SSE movsd xmm, xmm
                            let dest = self.parse_operand()?;
                            self.expect_comma()?;
                            let src = self.parse_operand()?;
                            Instruction::Movsd(dest, src)
                        } else {
                            Instruction::Movs
                        }
                    }
                    "movsq" => Instruction::Movsq,
                    "cmpsb" => Instruction::Cmpsb,
                    "cmpsw" => Instruction::Cmpsw,
                    "cmpsd" => Instruction::Cmpsd,
                    "cmpsq" => Instruction::Cmpsq,
                    "scasb" => Instruction::Scasb,
                    "scasw" => Instruction::Scasw,
                    "scasd" => Instruction::Scasd,
                    "scasq" => Instruction::Scasq,
                    "stosb" => Instruction::Stosb,
                    "stosw" => Instruction::Stosw,
                    "stosd" => Instruction::Stosd,
                    "stosq" => Instruction::Stosq,
                    "lodsb" => Instruction::Lodsb,
                    "lodsw" => Instruction::Lodsw,
                    "lodsd" => Instruction::Lodsd,
                    "lodsq" => Instruction::Lodsq,

                    "mfence" => Instruction::Mfence,
                    "lfence" => Instruction::Lfence,
                    "sfence" => Instruction::Sfence,
                    "syscall" => Instruction::Syscall,
                    "sysenter" => Instruction::Sysenter,
                    "sysexit" => Instruction::Sysexit,
                    "ret" => Instruction::Ret,
                    "nop" => Instruction::Nop,
                    "int3" => Instruction::Int3,
                    "hlt" => Instruction::Hlt,
                    "cpuid" => Instruction::CpuId,
                    "rdtsc" => Instruction::Rdtsc,
                    "push" => {
                        let op = self.parse_operand()?;
                        Instruction::Push(op)
                    }
                    "pop" => {
                        let op = self.parse_operand()?;
                        Instruction::Pop(op)
                    }
                    "mov" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Mov(dest, src)
                    }
                    "sub" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Sub(dest, src)
                    }
                    "add" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Add(dest, src)
                    }
                    "xor" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Xor(dest, src)
                    }
                    "and" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::And(dest, src)
                    }
                    "or" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Or(dest, src)
                    }
                    "inc" => {
                        let op = self.parse_operand()?;
                        Instruction::Inc(op)
                    }
                    "dec" => {
                        let op = self.parse_operand()?;
                        Instruction::Dec(op)
                    }
                    "neg" => {
                        let op = self.parse_operand()?;
                        Instruction::Neg(op)
                    }
                    "not" => {
                        let op = self.parse_operand()?;
                        Instruction::Not(op)
                    }
                    "cmp" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmp(dest, src)
                    }
                    "test" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Test(dest, src)
                    }
                    "lea" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Lea(dest, src)
                    }
                    "jmp" => {
                        let target = self.parse_operand()?;
                        Instruction::Jmp(target)
                    }
                    "je" | "jz" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::E, target)
                    }
                    "jne" | "jnz" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::NE, target)
                    }
                    "jg" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::G, target)
                    }
                    "jge" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::GE, target)
                    }
                    "jl" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::L, target)
                    }
                    "jle" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::LE, target)
                    }
                    "ja" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::A, target)
                    }
                    "jb" => {
                        let target = self.parse_operand()?;
                        Instruction::Jcc(ConditionCode::B, target)
                    }
                    "call" => {
                        let target = self.parse_operand()?;
                        Instruction::Call(target)
                    }
                    "cmove" | "cmovz" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmove(dest, src)
                    }
                    "cmovne" | "cmovnz" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovne(dest, src)
                    }
                    "cmovg" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovg(dest, src)
                    }
                    "cmovge" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovge(dest, src)
                    }
                    "cmovl" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovl(dest, src)
                    }
                    "cmovle" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovle(dest, src)
                    }
                    "cmova" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmova(dest, src)
                    }
                    "cmovae" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovae(dest, src)
                    }
                    "cmovb" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovb(dest, src)
                    }
                    "cmovbe" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovbe(dest, src)
                    }
                    "cmovs" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovs(dest, src)
                    }
                    "cmovns" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Cmovns(dest, src)
                    }
                    "movsd" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Movsd(dest, src)
                    }
                    "movss" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Movss(dest, src)
                    }
                    "movaps" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Movaps(dest, src)
                    }
                    "movapd" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Movapd(dest, src)
                    }
                    "addpd" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Addpd(dest, src)
                    }
                    "addps" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src = self.parse_operand()?;
                        Instruction::Addps(dest, src)
                    }
                    "vaddpd" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src1 = self.parse_operand()?;
                        self.expect_comma()?;
                        let src2 = self.parse_operand()?;
                        Instruction::Vaddpd(dest, src1, src2)
                    }
                    "vaddps" => {
                        let dest = self.parse_operand()?;
                        self.expect_comma()?;
                        let src1 = self.parse_operand()?;
                        self.expect_comma()?;
                        let src2 = self.parse_operand()?;
                        Instruction::Vaddps(dest, src1, src2)
                    }
                    _ => {
                        let loc = self.current_location();
                        return Err((format!("Unknown mnemonic: {}", mnemonic), loc.line, loc.col));
                    }
                };
                
                Ok(Some(instr))
            }
            Token::Label(name) => {
                self.advance();
                self.expect(Token::Colon)?;
                self.parse_instruction()
            }
            _ => {
                let loc = self.current_location();
                Err((format!("Expected instruction, got {:?}", token), loc.line, loc.col))
            }
        }
    }
    
    fn parse_operand(&mut self) -> Result<Spanned<Operand>, (String, usize, usize)> {
        let start_loc = self.current_location();
        
        let token = match self.peek() {
            Some(t) => t.clone(),
            None => Token::Eof,
        };
        
        match token {
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
            Token::OpenBracket => {
                self.parse_memory_address()
            }
            _ => {
                let loc = self.current_location();
                Err((format!("Expected operand, got {:?}", token), loc.line, loc.col))
            }
        }
    }
        
    fn parse_memory_address(&mut self) -> Result<Spanned<Operand>, (String, usize, usize)> {
        let start_loc = self.current_location();
        self.expect(Token::OpenBracket)?;
        
        let base = match self.peek() {
            Some(Token::Register(reg)) => {
                let reg_clone = reg.clone();
                self.advance();
                reg_clone
            }
            _ => {
                let loc = self.current_location();
                return Err(("Expected base register in memory operand".to_string(), loc.line, loc.col));
            }
        };
        
        let mut displacement = 0;
        
        let next_is_plus = match self.peek() {
            Some(Token::Plus) => true,
            _ => false,
        };
        
        if next_is_plus {
            self.advance();
            
            let imm = match self.peek() {
                Some(Token::Immediate(imm)) => *imm,
                _ => {
                    let loc = self.current_location();
                    return Err(("Expected displacement after +".to_string(), loc.line, loc.col));
                }
            };
            self.advance();
            displacement = imm;
        }
        
        self.expect(Token::CloseBracket)?;
        
        Ok(Spanned {
            value: Operand::Memory(MemoryAddress { base, displacement }),
            location: start_loc,
        })
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), (String, usize, usize)> {
        match self.peek() {
            Some(token) if *token == expected => {
                self.advance();
                Ok(())
            }
            Some(token) => {
                let loc = self.current_location();
                Err((format!("Expected {:?}, got {:?}", expected, token), loc.line, loc.col))
            }
            None => {
                let loc = self.current_location();
                Err((format!("Expected {:?}, got EOF", expected), loc.line, loc.col))
            }
        }
    }
    
    fn expect_comma(&mut self) -> Result<(), (String, usize, usize)> {
        self.expect(Token::Comma)
    }
    
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index).map(|t| &t.value)
    }
    
    fn advance(&mut self) {
        if self.index < self.tokens.len() {
            self.index += 1;
        }
    }
    
    fn current_location(&self) -> token::Location {
        self.tokens.get(self.index)
            .map(|t| t.location.clone())
            .unwrap_or(token::Location { line: 0, col: 0 })
    }
}