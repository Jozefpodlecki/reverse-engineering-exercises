use crate::{parser::register::Register};

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixSet(u8);

impl PrefixSet {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        if self.has_lock() { count += 1; }
        if self.has_rep() { count += 1; }
        if self.has_repne() { count += 1; }
        count
    }
    
    pub fn set_lock(&mut self) {
        self.0 |= 0b001;
    }
    
    pub fn set_rep(&mut self) {
        self.0 |= 0b010;
    }
    
    pub fn set_repne(&mut self) {
        self.0 |= 0b100;
    }
    
    pub fn has_lock(&self) -> bool {
        self.0 & 0b001 != 0
    }
    
    pub fn has_rep(&self) -> bool {
        self.0 & 0b010 != 0
    }
    
    pub fn has_repne(&self) -> bool {
        self.0 & 0b100 != 0
    }
    
    pub fn iter(&self) -> impl Iterator<Item = Prefix> {
        let mut v = Vec::new();
        if self.has_lock() { v.push(Prefix::Lock); }
        if self.has_rep() { v.push(Prefix::Rep); }
        if self.has_repne() { v.push(Prefix::Repne); }
        v.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prefix {
    Lock,
    Rep,
    Repne,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionCode {
    E, NE, G, GE, L, LE, A, AE, B, BE, C, NC, Z, NZ, O, NO, S, NS, P, NP
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Memory(MemoryAddress),
    Immediate(i64),
    Label(String), 
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryAddress {
    pub base: Register,
    pub displacement: i64,
}