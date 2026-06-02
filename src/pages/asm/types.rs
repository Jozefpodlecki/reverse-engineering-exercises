use std::rc::Rc;

use crate::pages::asm::Decoder;

#[derive(Clone)]
pub struct Tab {
    pub id: String,
    pub name: String,
    pub decoder: Rc<dyn Decoder>,
    pub instructions: Vec<AsmInstruction>,
    pub registers: Registers,
}

impl Tab {
    pub fn instructions_valid(&self) -> bool {
        self.instructions.iter().all(|instr| instr.is_valid())
    }
}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.decoder.name() == other.decoder.name() &&
        self.instructions == other.instructions &&
        self.registers == other.registers
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[derive(Default, Clone, PartialEq)]
pub struct AsmInstruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub length: u8,
    pub asm: String,
    pub mnemonic: String,
    pub operands: String,
    pub is_valid: bool,
}

impl AsmInstruction {
    pub fn new() -> Self {
        Self {
            address: 0,
            bytes: Vec::new(),
            length: 0,
            asm: String::new(),
            mnemonic: String::new(),
            operands: String::new(),
            is_valid: false,
        }
    }
    
    pub fn from_bytes(bytes: Vec<u8>, address: u64, decoder: &dyn Decoder) -> Self {
        if bytes.is_empty() {
            return Self::new();
        }
        
        match decoder.decode(&bytes, address) {
            Ok(decoded) => Self {
                address,
                bytes: bytes.clone(),
                length: decoded.size,
                asm: decoded.asm,
                mnemonic: decoded.mnemonic,
                operands: decoded.operands,
                is_valid: true,
            },
            Err(_) => Self {
                address,
                bytes,
                length: 0,
                asm: String::new(),
                mnemonic: String::new(),
                operands: String::new(),
                is_valid: false,
            },
        }
    }
    
    pub fn is_valid(&self) -> bool {
        self.is_valid && !self.bytes.is_empty()
    }
}