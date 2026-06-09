use crate::encoder::x86_64;
use crate::error::AssemblerError;
use crate::source::Source;
use crate::parser::Lexer;
use crate::parser::Parser;

pub struct Assembler;

impl Assembler {
    pub fn new() -> Self {
        Self
    }
    
    pub fn assemble<S: Source>(&self, source: S) -> Result<Vec<u8>, AssemblerError> {
        let source_name = source.name().unwrap_or("<source>");
        let source_str = source.get_source()
            .map_err(|e| AssemblerError::SourceError(e.to_string()))?;
        
        let lexer = Lexer::new(&source_str, source_name);
        let (tokens, lex_errors) = lexer.tokenize();
        
        if !lex_errors.is_empty() {
            let err = &lex_errors[0];
            return Err(AssemblerError::LexerError(
                err.message.clone(),
                err.line,
                err.col,
            ));
        }
        
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse()
            .map_err(|(msg, line, col)| AssemblerError::ParserError(msg, line, col))?;
        
        let mut binary = Vec::new();
        for instr in instructions {
            binary.extend(x86_64::Encoder::encode(&instr));
        }
        
        Ok(binary)
    }
    
    pub fn assemble_str(&self, source: &str) -> Result<Vec<u8>, AssemblerError> {
        self.assemble(source)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}