use std::collections::HashMap;
use crate::encoder::x86_64::{self, EncoderIter};
use crate::error::AssemblerError;
use crate::source::Source;
use crate::parser::Parser;
use crate::symbol::SymbolResolver;

#[cfg(feature = "alloc")]
pub struct Assembler {
    symbol_table: HashMap<String, usize>,
}

#[cfg(feature = "alloc")]
impl SymbolResolver for Assembler {
    fn lookup(&self, name: &str) -> Option<usize> {
        self.symbol_table.get(name).copied()
    }
}

#[cfg(feature = "alloc")]
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
            let encoded = x86_64::Encoder::encode_with_labels(&instr, &self.symbol_table, current_offset)
                .map_err(|err| AssemblerError::EncodingError(err))?;
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
            let encoded = x86_64::Encoder::encode_with_labels(&instr, &label_offsets, current_offset)
                .map_err(|err| AssemblerError::EncodingError(err))?;
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

pub struct AssemblerNoSymbols;

impl AssemblerNoSymbols {
    pub fn new() -> Self {
        Self
    }
    
    pub fn assemble<S: Source>(&self, source: S) -> Result<Vec<u8>, AssemblerError> {
        let source_str = source.get_source().map_err(AssemblerError::SourceError)?;
        let mut parser = Parser::new(&source_str);
        let instructions = parser.parse()?;
        
        let mut binary = Vec::new();
        for instr in instructions {
            let encoded = x86_64::Encoder::encode(&instr)?;
            binary.extend_from_slice(encoded.as_ref());
        }
        Ok(binary)
    }

    pub fn assemble_str(&mut self, source: &str) -> Result<Vec<u8>, AssemblerError> {
        self.assemble(source)
    }
}