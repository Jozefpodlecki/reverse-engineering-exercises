use core::{iter::Peekable, str::Chars};

use crate::{LexerError, LexerErrorType, LexerErrors, Location, Spanned, Token, string::StackString};

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: Location
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            position: Location { line: 1, col: 1 }
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
                return self.next_token()
                    .ok_or_else(|| LexerError {
                        kind: LexerErrorType::UnexpectedEof,
                        line: start_loc.line,
                        col: start_loc.col,
                    })?
                    .map(|spanned| spanned.value);
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
                    kind: LexerErrorType::UnexpectedChar(char),
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

    fn read_identifier(&mut self) -> Result<StackString<32>, LexerError> {
        let mut ident = StackString::<32>::new();
        let start_pos = self.position.clone();
        
        while let Some(&c) = self.source.peek() {
            let should_continue = c.is_alphanumeric() 
                || c == '_' 
                || (c == 'x' && ident.as_str() == "0");
            
            if should_continue {
                ident.push(c).map_err(|_| LexerError {
                    kind: LexerErrorType::CapacityExceeded,
                    line: start_pos.line,
                    col: start_pos.col,
                })?;
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(ident)
    }
        
    fn read_identifier_or_number(&mut self) -> Result<Token, LexerError> {
        let ident = self.read_identifier()?;
        
        if let Some(val) = ident.parse_hex() {
            return Ok(Token::Immediate(val));
        }
        
        if let Some(val) = ident.parse_decimal() {
            return Ok(Token::Immediate(val));
        }
        
        if let Some(token) = Token::from_ident(ident.as_str()) {
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