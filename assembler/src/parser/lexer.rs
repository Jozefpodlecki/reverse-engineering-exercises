use core::{iter::Peekable, str::Chars};

use super::token::{Token, Location, Spanned};

pub struct LexerErrors(pub Vec<LexerError>);

impl std::fmt::Display for LexerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.0 {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for LexerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for LexerErrors {}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.col, self.message)
    }
}

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
    
    pub fn tokenize(mut self) -> Result<Vec<Spanned<Token>>, LexerErrors> {
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
                    self.skip_line();
                }
            }
        }

        if !errors.is_empty() {
            return Err(LexerErrors(errors));
        }
        
        Ok(tokens)
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
        
        let token = self.parse_char(next_char, &start_loc);
        
        Some(match token {
            Ok(token) => Ok(Spanned { value: token, location: start_loc }),
            Err(err) => Err(err),
        })
    }

    fn parse_char(&mut self, char: char, start_loc: &Location) -> Result<Token, LexerError> {
        match char {
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
                self.next_token().ok_or_else(|| LexerError {
                    message: "EOF after comment".to_string(),
                    line: start_loc.line,
                    col: start_loc.col,
                })??;
                unreachable!()
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
                    message: format!("Unexpected character: '{}'", char),
                    line: start_loc.line,
                    col: start_loc.col,
                })
            }
        }
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
                self.advance();
            } else {
                break;
            }
        }
        
        if ident.starts_with("0x") || ident.starts_with("0X") {
            let val = i64::from_str_radix(&ident[2..], 16)
                .map_err(|_| LexerError {
                    message: format!("Invalid hex: {}", ident),
                    line: start_pos.line,
                    col: start_pos.col,
                })?;
            return Ok(Token::Immediate(val));
        }
        
        if let Ok(val) = ident.parse::<i64>() {
            return Ok(Token::Immediate(val));
        }
        
        if let Some(token) = Token::from_ident(&ident) {
            Ok(token)
        } else {
            Ok(Token::Label(ident))
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.source.next() {
            if c == '\n' {
                self.position.line += 1;
                self.position.col = 1;
            } else {
                self.position.col += 1;
            }
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Spanned<Token>, LexerError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}