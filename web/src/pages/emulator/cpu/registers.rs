use iced_x86::Register;


#[derive(Clone, PartialEq)]
pub struct GeneralPurposeRegisters {
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
}

impl GeneralPurposeRegisters {
    pub fn new() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rsp: 0x7FFFFFFF0000,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }
    
    pub fn read_u64(&self, reg: Register) -> u64 {
        match reg {
            Register::RAX => self.rax,
            Register::RBX => self.rbx,
            Register::RCX => self.rcx,
            Register::RDX => self.rdx,
            Register::RSI => self.rsi,
            Register::RDI => self.rdi,
            Register::RBP => self.rbp,
            Register::RSP => self.rsp,
            Register::R8 => self.r8,
            Register::R9 => self.r9,
            Register::R10 => self.r10,
            Register::R11 => self.r11,
            Register::R12 => self.r12,
            Register::R13 => self.r13,
            Register::R14 => self.r14,
            Register::R15 => self.r15,
            _ => 0,
        }
    }
    
    pub fn write_u64(&mut self, reg: Register, value: u64) {
        match reg {
            Register::RAX => self.rax = value,
            Register::RBX => self.rbx = value,
            Register::RCX => self.rcx = value,
            Register::RDX => self.rdx = value,
            Register::RSI => self.rsi = value,
            Register::RDI => self.rdi = value,
            Register::RBP => self.rbp = value,
            Register::RSP => self.rsp = value,
            Register::R8 => self.r8 = value,
            Register::R9 => self.r9 = value,
            Register::R10 => self.r10 = value,
            Register::R11 => self.r11 = value,
            Register::R12 => self.r12 = value,
            Register::R13 => self.r13 = value,
            Register::R14 => self.r14 = value,
            Register::R15 => self.r15 = value,
            _ => {}
        }
    }
    
    pub fn read_u32(&self, reg: Register) -> u32 {
        match reg {
            Register::EAX => self.rax as u32,
            Register::EBX => self.rbx as u32,
            Register::ECX => self.rcx as u32,
            Register::EDX => self.rdx as u32,
            Register::ESI => self.rsi as u32,
            Register::EDI => self.rdi as u32,
            Register::EBP => self.rbp as u32,
            Register::ESP => self.rsp as u32,
            Register::R8D => self.r8 as u32,
            Register::R9D => self.r9 as u32,
            Register::R10D => self.r10 as u32,
            Register::R11D => self.r11 as u32,
            Register::R12D => self.r12 as u32,
            Register::R13D => self.r13 as u32,
            Register::R14D => self.r14 as u32,
            Register::R15D => self.r15 as u32,
            _ => 0,
        }
    }
    
    pub fn write_u32(&mut self, reg: Register, value: u32) {
        let value = value as u64;
        match reg {
            Register::EAX => self.rax = (self.rax & 0xFFFFFFFF00000000) | value,
            Register::EBX => self.rbx = (self.rbx & 0xFFFFFFFF00000000) | value,
            Register::ECX => self.rcx = (self.rcx & 0xFFFFFFFF00000000) | value,
            Register::EDX => self.rdx = (self.rdx & 0xFFFFFFFF00000000) | value,
            Register::ESI => self.rsi = (self.rsi & 0xFFFFFFFF00000000) | value,
            Register::EDI => self.rdi = (self.rdi & 0xFFFFFFFF00000000) | value,
            Register::EBP => self.rbp = (self.rbp & 0xFFFFFFFF00000000) | value,
            Register::ESP => self.rsp = (self.rsp & 0xFFFFFFFF00000000) | value,
            Register::R8D => self.r8 = (self.r8 & 0xFFFFFFFF00000000) | value,
            Register::R9D => self.r9 = (self.r9 & 0xFFFFFFFF00000000) | value,
            Register::R10D => self.r10 = (self.r10 & 0xFFFFFFFF00000000) | value,
            Register::R11D => self.r11 = (self.r11 & 0xFFFFFFFF00000000) | value,
            Register::R12D => self.r12 = (self.r12 & 0xFFFFFFFF00000000) | value,
            Register::R13D => self.r13 = (self.r13 & 0xFFFFFFFF00000000) | value,
            Register::R14D => self.r14 = (self.r14 & 0xFFFFFFFF00000000) | value,
            Register::R15D => self.r15 = (self.r15 & 0xFFFFFFFF00000000) | value,
            _ => {}
        }
    }
    
    pub fn read_u16(&self, reg: Register) -> u16 {
        match reg {
            Register::AX => self.rax as u16,
            Register::BX => self.rbx as u16,
            Register::CX => self.rcx as u16,
            Register::DX => self.rdx as u16,
            Register::SI => self.rsi as u16,
            Register::DI => self.rdi as u16,
            Register::BP => self.rbp as u16,
            Register::SP => self.rsp as u16,
            Register::R8W => self.r8 as u16,
            Register::R9W => self.r9 as u16,
            Register::R10W => self.r10 as u16,
            Register::R11W => self.r11 as u16,
            Register::R12W => self.r12 as u16,
            Register::R13W => self.r13 as u16,
            Register::R14W => self.r14 as u16,
            Register::R15W => self.r15 as u16,
            _ => 0,
        }
    }
    
    pub fn write_u16(&mut self, reg: Register, value: u16) {
        let value = value as u64;
        match reg {
            Register::AX => self.rax = (self.rax & 0xFFFFFFFFFFFF0000) | value,
            Register::BX => self.rbx = (self.rbx & 0xFFFFFFFFFFFF0000) | value,
            Register::CX => self.rcx = (self.rcx & 0xFFFFFFFFFFFF0000) | value,
            Register::DX => self.rdx = (self.rdx & 0xFFFFFFFFFFFF0000) | value,
            Register::SI => self.rsi = (self.rsi & 0xFFFFFFFFFFFF0000) | value,
            Register::DI => self.rdi = (self.rdi & 0xFFFFFFFFFFFF0000) | value,
            Register::BP => self.rbp = (self.rbp & 0xFFFFFFFFFFFF0000) | value,
            Register::SP => self.rsp = (self.rsp & 0xFFFFFFFFFFFF0000) | value,
            Register::R8W => self.r8 = (self.r8 & 0xFFFFFFFFFFFF0000) | value,
            Register::R9W => self.r9 = (self.r9 & 0xFFFFFFFFFFFF0000) | value,
            Register::R10W => self.r10 = (self.r10 & 0xFFFFFFFFFFFF0000) | value,
            Register::R11W => self.r11 = (self.r11 & 0xFFFFFFFFFFFF0000) | value,
            Register::R12W => self.r12 = (self.r12 & 0xFFFFFFFFFFFF0000) | value,
            Register::R13W => self.r13 = (self.r13 & 0xFFFFFFFFFFFF0000) | value,
            Register::R14W => self.r14 = (self.r14 & 0xFFFFFFFFFFFF0000) | value,
            Register::R15W => self.r15 = (self.r15 & 0xFFFFFFFFFFFF0000) | value,
            _ => {}
        }
    }
    
    pub fn read_u8(&self, reg: Register) -> u8 {
        match reg {
             Register::AL => self.rax as u8,
            Register::BL => self.rbx as u8,
            Register::CL => self.rcx as u8,
            Register::DL => self.rdx as u8,
            Register::SIL => self.rsi as u8,
            Register::DIL => self.rdi as u8,
            Register::BPL => self.rbp as u8,
            Register::SPL => self.rsp as u8,
            Register::R8L => self.r8 as u8,
            Register::R9L => self.r9 as u8,
            Register::R10L => self.r10 as u8,
            Register::R11L => self.r11 as u8,
            Register::R12L => self.r12 as u8,
            Register::R13L => self.r13 as u8,
            Register::R14L => self.r14 as u8,
            Register::R15L => self.r15 as u8,
            Register::AH => (self.rax >> 8) as u8,
            Register::BH => (self.rbx >> 8) as u8,
            Register::CH => (self.rcx >> 8) as u8,
            Register::DH => (self.rdx >> 8) as u8,
            _ => 0,
        }
    }
    
    pub fn write_u8(&mut self, reg: Register, value: u8) {
        let value = value as u64;
        match reg {
            Register::AL => self.rax = (self.rax & 0xFFFFFFFFFFFFFF00) | value,
            Register::BL => self.rbx = (self.rbx & 0xFFFFFFFFFFFFFF00) | value,
            Register::CL => self.rcx = (self.rcx & 0xFFFFFFFFFFFFFF00) | value,
            Register::DL => self.rdx = (self.rdx & 0xFFFFFFFFFFFFFF00) | value,
            Register::SIL => self.rsi = (self.rsi & 0xFFFFFFFFFFFFFF00) | value,
            Register::DIL => self.rdi = (self.rdi & 0xFFFFFFFFFFFFFF00) | value,
            Register::BPL => self.rbp = (self.rbp & 0xFFFFFFFFFFFFFF00) | value,
            Register::SPL => self.rsp = (self.rsp & 0xFFFFFFFFFFFFFF00) | value,
            Register::R8L => self.r8 = (self.r8 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R9L => self.r9 = (self.r9 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R10L => self.r10 = (self.r10 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R11L => self.r11 = (self.r11 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R12L => self.r12 = (self.r12 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R13L => self.r13 = (self.r13 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R14L => self.r14 = (self.r14 & 0xFFFFFFFFFFFFFF00) | value,
            Register::R15L => self.r15 = (self.r15 & 0xFFFFFFFFFFFFFF00) | value,
            Register::AH => self.rax = (self.rax & 0xFFFFFFFFFFFF00FF) | (value << 8),
            Register::BH => self.rbx = (self.rbx & 0xFFFFFFFFFFFF00FF) | (value << 8),
            Register::CH => self.rcx = (self.rcx & 0xFFFFFFFFFFFF00FF) | (value << 8),
            Register::DH => self.rdx = (self.rdx & 0xFFFFFFFFFFFF00FF) | (value << 8),
            _ => {}
        }
    }
}

impl Default for GeneralPurposeRegisters {
    fn default() -> Self {
        Self::new()
    }
}