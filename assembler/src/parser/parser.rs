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
        let token = match self.tokens.get(self.index) {
            Some(t) => t.value.clone(),
            None => Token::Eof,
        };
        
        if matches!(token, Token::Eof) {
            return Ok(None);
        }
        
        match token {
            Token::Mnemonic(mnemonic) => {
                let start_loc = self.current_location();
                self.advance();
                
                let instr = match mnemonic.as_str() {
                    "syscall" => Instruction::Syscall,
                    "ret" => Instruction::Ret,
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