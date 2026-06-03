use std::rc::Rc;

use crate::pages::asm::*;

#[derive(Clone)]
pub struct Tab {
    pub id: u32,
    pub name: String,
    pub decoder: Rc<dyn Decoder>,
    pub instructions: AsmInstructions,
    pub registers: Registers,
    pub memory: Memory,
    pub sub_tab: SubTab
}

#[derive(Default, Clone, PartialEq)]
pub enum SubTab {
    #[default]
    Code,
    Memory,
}


impl Tab {
    pub fn can_run(&self) -> bool {
        self.instructions.is_all_valid() && !self.instructions.is_empty()
    }
}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.decoder.name() == other.decoder.name() &&
        self.instructions == other.instructions
    }
}

#[derive(Clone, PartialEq)]
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

impl Default for Registers {
    fn default() -> Self {
        Self {
            rax: Default::default(), rbx: Default::default(), rcx: Default::default(), rdx: Default::default(),
            rsi: Default::default(), rdi: Default::default(), rbp: Default::default(),
            rsp: 0x7FFFFFFF0000,
            r8: Default::default(), r9: Default::default(), r10: Default::default(), r11: Default::default(),
            r12: Default::default(), r13: Default::default(), r14: Default::default(), r15: Default::default(),
            rip: Default::default(),
            rflags: Default::default()
        }
    }
}

impl Registers {
    pub fn update(&mut self, name: &str, value: u64) {
        match name {
            "RAX" => self.rax = value,
            "RBX" => self.rbx = value,
            "RCX" => self.rcx = value,
            "RDX" => self.rdx = value,
            "RSI" => self.rsi = value,
            "RDI" => self.rdi = value,
            "RBP" => self.rbp = value,
            "RSP" => self.rsp = value,
            "R8" => self.r8 = value,
            "R9" => self.r9 = value,
            "R10" => self.r10 = value,
            "R11" => self.r11 = value,
            "R12" => self.r12 = value,
            "R13" => self.r13 = value,
            "R14" => self.r14 = value,
            "R15" => self.r15 = value,
            "RIP" => self.rip = value,
            _ => {}
        }
    }
    
    pub fn iter(&self) -> Vec<(String, u64)> {
        vec![
            ("RAX".to_string(), self.rax),
            ("RBX".to_string(), self.rbx),
            ("RCX".to_string(), self.rcx),
            ("RDX".to_string(), self.rdx),
            ("RSI".to_string(), self.rsi),
            ("RDI".to_string(), self.rdi),
            ("RBP".to_string(), self.rbp),
            ("RSP".to_string(), self.rsp),
            ("R8".to_string(), self.r8),
            ("R9".to_string(), self.r9),
            ("R10".to_string(), self.r10),
            ("R11".to_string(), self.r11),
            ("R12".to_string(), self.r12),
            ("R13".to_string(), self.r13),
            ("R14".to_string(), self.r14),
            ("R15".to_string(), self.r15),
        ]
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AsmInstructions(Vec<AsmInstruction>);

impl From<Vec<AsmInstruction>> for AsmInstructions {
    fn from(vec: Vec<AsmInstruction>) -> Self {
        Self(vec)
    }
}

impl AsmInstructions {

    pub fn new() -> Self {
        Self(Vec::new())
    }
    
    pub fn get(&self, index: usize) -> Option<&AsmInstruction> {
        self.0.get(index)
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut AsmInstruction> {
        self.0.get_mut(index)
    }
    
    pub fn get_by_rip(&self, addr: u64) -> Option<&AsmInstruction> {
        self.0.iter().find(|instr| instr.address == addr)
    }
    
    pub fn get_index_by_rip(&self, addr: u64) -> Option<usize> {
        self.0.iter().position(|instr| instr.address == addr)
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    pub fn push(&mut self, instruction: AsmInstruction) {
        self.0.push(instruction);
    }
    
    pub fn remove(&mut self, index: usize) -> AsmInstruction {
        self.0.remove(index)
    }
    
    pub fn iter(&self) -> std::slice::Iter<'_, AsmInstruction> {
        self.0.iter()
    }
    
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, AsmInstruction> {
        self.0.iter_mut()
    }
    
    pub fn is_all_valid(&self) -> bool {
        self.0.iter().all(|instr| instr.is_valid())
    }

    pub fn recalculate_addresses(&mut self, start_rip: u64) {
        let mut addr = start_rip;
        for instr in self.0.iter_mut() {
            instr.address = addr;
            if instr.decoded.length > 0 {
                addr += instr.decoded.length as u64;
            }
        }
    }
}

impl std::ops::Index<usize> for AsmInstructions {
    type Output = AsmInstruction;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for AsmInstructions {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AsmInstruction {
    pub index: usize,
    pub address: u64,
    pub bytes: Vec<u8>,
    pub decoded: DecodedInstruction,
    pub is_valid: bool,
}

impl AsmInstruction {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            address: 0,
            bytes: Vec::new(),
            decoded: Default::default(),
            is_valid: false,
        }
    }

    pub fn bytes_input(&self) -> [Option<u8>; 15] {
        let mut arr = [None; 15];

        for (i, &b) in self.bytes.iter().enumerate() {
            if i < 15 {
                arr[i] = Some(b);
            }
        }

        arr
    }
    
    pub fn from_bytes(index: usize, bytes: Vec<u8>, address: u64, decoder: &dyn Decoder) -> Self {
        if bytes.is_empty() {
            return Self::new(index);
        }
        
        match decoder.decode(&bytes, address) {
            Ok(decoded) => Self {
                index,
                address,
                bytes,
                decoded,
                is_valid: true,
                ..Default::default()
            },
            Err(_) => Self {
                index,
                address,
                bytes,
                is_valid: false,
                ..Default::default()
            },
        }
    }
    
    pub fn is_valid(&self) -> bool {
        self.is_valid && !self.bytes.is_empty()
    }
}