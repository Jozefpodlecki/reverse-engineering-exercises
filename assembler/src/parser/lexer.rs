use core::{iter::Peekable, str::Chars};

use super::token::{Token, Location, Spanned};

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: Location,
    source_name: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, source_name: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            position: Location { line: 1, col: 1 },
            source_name,
        }
    }
    
    pub fn tokenize(mut self) -> (Vec<Spanned<Token>>, Vec<LexerError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        
        while let Some(result) = self.next_token() {
            match result {
                Ok(token) => {
                    let is_eof = matches!(token.value, Token::Eof);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(err) => {
                    errors.push(err);
                    self.skip_to_next_line();
                }
            }
        }
        
        (tokens, errors)
    }
            
    fn next_token(&mut self) -> Option<Result<Spanned<Token>, LexerError>> {
        self.skip_whitespace();

        if self.source.peek().is_none() {
            let start_loc = self.position.clone();
            return Some(Ok(Spanned {
                value: Token::Eof,
                location: start_loc,
            }));
        }
        
        let start_loc = self.position.clone();
        let next_char = *self.source.peek()?;
        
        let token = match next_char {
            '[' => self.single_char_token(Token::OpenBracket),
            ']' => self.single_char_token(Token::CloseBracket),
            ',' => self.single_char_token(Token::Comma),
            '+' => self.single_char_token(Token::Plus),
            '-' => self.single_char_token(Token::Minus),
            ':' => self.single_char_token(Token::Colon),
            '*' => self.single_char_token(Token::Star),
            '/' => self.single_char_token(Token::Slash),
            '%' => self.single_char_token(Token::Percent),
            '=' => self.single_char_token(Token::Equal),
            '!' => self.single_char_token(Token::Exclamation),
            '&' => self.single_char_token(Token::Ampersand),
            '|' => self.single_char_token(Token::Pipe),
            '^' => self.single_char_token(Token::Caret),
            '~' => self.single_char_token(Token::Tilde),
            '.' => self.single_char_token(Token::Dot),
            '$' => self.single_char_token(Token::Dollar),
            '#' => self.single_char_token(Token::Hash),
            ';' => {
                self.skip_line();
                return self.next_token();
            }
            '\n' => {
                self.source.next();
                self.position.line += 1;
                self.position.col = 1;
                Ok(Token::Newline)
            }
            '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => self.read_identifier_or_number(),
            _ => {
                self.source.next();
                Err(LexerError {
                    message: format!("Unexpected character: '{}'", next_char),
                    line: start_loc.line,
                    col: start_loc.col,
                })
            }
        };
        
        Some(match token {
            Ok(token) => Ok(Spanned { value: token, location: start_loc }),
            Err(err) => Err(err),
        })
    }

    fn single_char_token(&mut self, token: Token) -> Result<Token, LexerError> {
        self.source.next();
        Ok(token)
    }
    
    fn read_identifier_or_number(&mut self) -> Result<Token, LexerError> {
        let mut ident = String::new();
        let start_pos = self.position.clone();
        
        while let Some(&c) = self.source.peek() {
            if c.is_alphanumeric() || c == '_' || (c == 'x' && ident == "0") {
                ident.push(c);
                self.source.next();
                self.position.col += 1;
            } else {
                break;
            }
        }
        
        if ident.starts_with("0x") || ident.starts_with("0X") {
            let num_str = &ident[2..];
            if num_str.chars().all(|c| c.is_ascii_hexdigit()) {
                let value = i64::from_str_radix(num_str, 16)
                    .map_err(|_| LexerError {
                        message: format!("Invalid hex number: {}", ident),
                        line: start_pos.line,
                        col: start_pos.col,
                    })?;
                return Ok(Token::Immediate(value));
            }
        }
        
        if let Ok(value) = ident.parse::<i64>() {
            return Ok(Token::Immediate(value));
        }
        
        match ident.as_str() {
            "lock" => Ok(Token::Lock),
            "rep" => Ok(Token::Rep),
            "repne" | "repnz" => Ok(Token::Repne),
            "rax" | "rcx" | "rdx" | "rbx" | "rsp" | "rbp" | "rsi" | "rdi" => Ok(Token::Register(ident)),
            "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" => Ok(Token::Register(ident)),
            "eax" | "ecx" | "edx" | "ebx" | "esp" | "ebp" | "esi" | "edi" => Ok(Token::Register(ident)),
            "ax" | "cx" | "dx" | "bx" | "sp" | "bp" | "si" | "di" => Ok(Token::Register(ident)),
            "al" | "cl" | "dl" | "bl" | "ah" | "ch" | "dh" | "bh" => Ok(Token::Register(ident)),

            _ if ident.starts_with('r') && ident.len() == 2 && ident[1..].parse::<u8>().is_ok() => {
                Ok(Token::Register(ident))
            }
            _ if ident.starts_with("xmm") && ident.len() > 3 => {
                let num = &ident[3..];
                if num.parse::<u8>().is_ok() {
                    Ok(Token::XmmRegister(ident))
                } else {
                    Ok(Token::Label(ident))
                }
            }
            _ if ident.starts_with("ymm") && ident.len() > 3 => {
                let num = &ident[3..];
                if num.parse::<u8>().is_ok() {
                    Ok(Token::YmmRegister(ident))
                } else {
                    Ok(Token::Label(ident))
                }
            }
            _ if ident.starts_with("zmm") && ident.len() > 3 => {
                let num = &ident[3..];
                if num.parse::<u8>().is_ok() && num.parse::<u8>().unwrap() <= 31 {
                    Ok(Token::ZmmRegister(ident))
                } else {
                    Ok(Token::Label(ident))
                }
            }
            _ if ident.starts_with("k") && ident.len() > 1 => {
                let num = &ident[1..];
                if num.parse::<u8>().is_ok() && num.parse::<u8>().unwrap() <= 7 {
                    Ok(Token::Register(ident))
                } else {
                    Ok(Token::Label(ident))
                }
            }
            "syscall" | "sysenter" | "sysexit" | "ret" | "nop" | "int3" | "hlt" | "cpuid" | "rdtsc" => Ok(Token::Mnemonic(ident)),
            "push" | "pop" | "mov" | "sub" | "add" | "xor" | "or" | "and" | "inc" | "dec" | "neg" | "not" => {
                Ok(Token::Mnemonic(ident))
            }
            "jmp" | "je" | "jne" | "jz" | "jnz" | "jg" | "jl" | "jge" | "jle" | "ja" | "jb" | "call" => {
                Ok(Token::Mnemonic(ident))
            }
            "cmp" | "test" | "lea" | "enter" | "leave" => Ok(Token::Mnemonic(ident)),
            "movsx" | "movzx" | "xchg" => Ok(Token::Mnemonic(ident)),
            "mul" | "imul" | "div" | "idiv" => Ok(Token::Mnemonic(ident)),
            "shl" | "shr" | "sar" | "sal" | "rol" | "ror" | "rcl" | "rcr" => Ok(Token::Mnemonic(ident)),
            "bt" | "bts" | "btr" | "btc" => Ok(Token::Mnemonic(ident)),
            "bsf" | "bsr" | "popcnt" | "lzcnt" | "tzcnt" => Ok(Token::Mnemonic(ident)),
            "cmove" | "cmovz" | "cmovne" | "cmovnz" | "cmovg" | "cmovge" | "cmovl" | "cmovle" |
            "cmova" | "cmovae" | "cmovb" | "cmovbe" | "cmovs" | "cmovns" => Ok(Token::Mnemonic(ident)),
            "movsb" | "movsw" | "movsd" | "movsq" => Ok(Token::Mnemonic(ident)),
            "cmpsb" | "cmpsw" | "cmpsd" | "cmpsq" => Ok(Token::Mnemonic(ident)),
            "scasb" | "scasw" | "scasd" | "scasq" => Ok(Token::Mnemonic(ident)),
            "stosb" | "stosw" | "stosd" | "stosq" => Ok(Token::Mnemonic(ident)),
            "lodsb" | "lodsw" | "lodsd" | "lodsq" => Ok(Token::Mnemonic(ident)),
            "mfence" | "lfence" | "sfence" => Ok(Token::Mnemonic(ident)),
            "movsd" | "movss" | "movaps" | "movups" | "movupd" | "movapd" => Ok(Token::Mnemonic(ident)),
            "addsd" | "addss" | "addpd" | "addps" => Ok(Token::Mnemonic(ident)),
            "subsd" | "subss" | "subpd" | "subps" => Ok(Token::Mnemonic(ident)),
            "mulsd" | "mulss" | "mulpd" | "mulps" => Ok(Token::Mnemonic(ident)),
            "divsd" | "divss" | "divpd" | "divps" => Ok(Token::Mnemonic(ident)),
            "sqrtpd" | "sqrtps" | "sqrtsd" | "sqrtss" => Ok(Token::Mnemonic(ident)),
            "vaddpd" | "vsubpd" | "vmulpd" | "vdivpd" => Ok(Token::Mnemonic(ident)),
            "vaddps" | "vsubps" | "vmulps" | "vdivps" => Ok(Token::Mnemonic(ident)),
            "vmovsd" | "vmovss" | "vmovapd" | "vmovaps" => Ok(Token::Mnemonic(ident)),
            "vmovdqa" | "vmovdqu" | "vmovdqa32" | "vmovdqa64" => Ok(Token::Mnemonic(ident)),
            "vpaddd" | "vpsubd" | "vpmulld" | "vpand" | "vpor" | "vpxor" => Ok(Token::Mnemonic(ident)),
            _ => Ok(Token::Label(ident)),
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.position.line += 1;
                    self.position.col = 1;
                } else {
                    self.position.col += 1;
                }
                self.source.next();
            } else {
                break;
            }
        }
    }
    
    fn skip_line(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c == '\n' {
                self.position.line += 1;
                self.position.col = 1;
                self.source.next();
                break;
            }
            self.source.next();
        }
    }
    
    fn skip_to_next_line(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c == '\n' {
                self.position.line += 1;
                self.position.col = 1;
                self.source.next();
                break;
            }
            self.source.next();
        }
    }
}