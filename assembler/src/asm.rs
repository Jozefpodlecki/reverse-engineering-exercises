use std::collections::HashMap;
use crate::encoder::x86_64::{self, EncoderIter};
use crate::error::AssemblerError;
use crate::source::Source;
use crate::parser::Parser;

pub struct Assembler {
    symbol_table: HashMap<String, usize>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn assemble<S: Source>(&mut self, source: S) -> Result<Vec<u8>, AssemblerError> {
        let source_name = source.name();
        let source_str = source.get_source().map_err(|err| AssemblerError::SourceError(err))?;
        
        let mut parser = Parser::new(&source_str);
        let (instructions, label_offsets) = parser.parse_with_labels()?;
        self.symbol_table = label_offsets;
        
        let mut binary = Vec::new();
        let mut current_offset = 0u64;
        
        for instr in instructions {
            let encoded = x86_64::Encoder::encode_with_labels(&instr, &self.symbol_table, current_offset);
            current_offset += encoded.len() as u64;
            binary.extend(encoded);
        }
        
        Ok(binary)
    }
    
    pub fn assemble_with_symbols<S: Source>(&mut self, source: S) -> Result<(Vec<u8>, HashMap<String, usize>), AssemblerError> {
        let source_name = source.name();
        let source_str = source.get_source().map_err(|err| AssemblerError::SourceError(err))?;
        
        let mut parser = Parser::new(&source_str);
        let (instructions, label_offsets) = parser.parse_with_labels()?;
        
        let mut binary = Vec::new();
        let mut current_offset = 0u64;
        
        for instr in instructions {
            let encoded = x86_64::Encoder::encode_with_labels(&instr, &label_offsets, current_offset);
            current_offset += encoded.len() as u64;
            binary.extend(encoded);
        }
        
        Ok((binary, label_offsets))
    }
    
    pub fn assemble_str(&mut self, source: &str) -> Result<Vec<u8>, AssemblerError> {
        self.assemble(source)
    }
    
    pub fn symbols(&self) -> &HashMap<String, usize> {
        &self.symbol_table
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}