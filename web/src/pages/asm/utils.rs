use rand::{Rng, RngExt, rng};

pub enum InstructionPattern {
    Single(u8),
    Double(u8, u8),
    Triple(u8, u8, u8),
    MovImm32(u8, u32),
    MovImm64(u8, u64),
    Rex(u8, &'static [u8]),
}

impl InstructionPattern {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rng = rng();
        match self {
            InstructionPattern::Single(b) => vec![*b],
            InstructionPattern::Double(a, b) => vec![*a, *b],
            InstructionPattern::Triple(a, b, c) => vec![*a, *b, *c],
            InstructionPattern::MovImm32(reg, imm) => {
                let imm_val = rng.random::<u32>();
                vec![*reg, 
                     (imm_val & 0xFF) as u8,
                     ((imm_val >> 8) & 0xFF) as u8,
                     ((imm_val >> 16) & 0xFF) as u8,
                     ((imm_val >> 24) & 0xFF) as u8]
            }
            InstructionPattern::MovImm64(reg, imm) => {
                let imm_val = rng.random::<u64>();
                vec![*reg,
                     (imm_val & 0xFF) as u8,
                     ((imm_val >> 8) & 0xFF) as u8,
                     ((imm_val >> 16) & 0xFF) as u8,
                     ((imm_val >> 24) & 0xFF) as u8,
                     ((imm_val >> 32) & 0xFF) as u8,
                     ((imm_val >> 40) & 0xFF) as u8,
                     ((imm_val >> 48) & 0xFF) as u8,
                     ((imm_val >> 56) & 0xFF) as u8]
            }
            InstructionPattern::Rex(prefix, rest) => {
                let mut bytes = vec![*prefix];
                bytes.extend_from_slice(rest);
                bytes
            }
        }
    }
}

const INSTRUCTION_SAMPLES: &[InstructionPattern] = &[
    // Single byte
    InstructionPattern::Single(0x90), // nop
    InstructionPattern::Single(0xC3), // ret
    InstructionPattern::Single(0xCC), // int3
    InstructionPattern::Single(0x50), // push rax
    InstructionPattern::Single(0x51), // push rcx
    InstructionPattern::Single(0x52), // push rdx
    InstructionPattern::Single(0x53), // push rbx
    InstructionPattern::Single(0x58), // pop rax
    InstructionPattern::Single(0x59), // pop rcx
    InstructionPattern::Single(0x5A), // pop rdx
    InstructionPattern::Single(0x5B), // pop rbx
    
    // Double byte
    InstructionPattern::Double(0x0F, 0x05), // syscall
    InstructionPattern::Double(0x0F, 0x34), // sysenter
    InstructionPattern::Double(0x31, 0xC0), // xor eax, eax
    InstructionPattern::Double(0x31, 0xDB), // xor ebx, ebx
    
    // Triple byte
    InstructionPattern::Triple(0x48, 0x31, 0xC0), // xor rax, rax
    InstructionPattern::Triple(0x48, 0x31, 0xDB), // xor rbx, rbx
    InstructionPattern::Triple(0x48, 0xFF, 0xC0), // inc rax
    InstructionPattern::Triple(0x48, 0xFF, 0xC8), // dec rax
    InstructionPattern::Triple(0x48, 0x83, 0xC0), // add rax, imm8 (needs imm)
    InstructionPattern::Triple(0x48, 0x83, 0xE8), // sub rax, imm8
    
    // MOV immediate 32-bit
    InstructionPattern::MovImm32(0xB8, 0), // mov eax, imm32
    InstructionPattern::MovImm32(0xB9, 0), // mov ecx, imm32
    InstructionPattern::MovImm32(0xBA, 0), // mov edx, imm32
    InstructionPattern::MovImm32(0xBB, 0), // mov ebx, imm32
    
    // MOV immediate 64-bit
    InstructionPattern::MovImm64(0x48, 0), // mov rax, imm64 (with REX)
    InstructionPattern::MovImm64(0x49, 0), // mov r8, imm64 (with REX.B)
];

pub fn random_instruction() -> [Option<u8>; 15] {
    let mut rng = rng();
    let mut bytes = [None; 15];
    
    let idx = rng.random_range(0..INSTRUCTION_SAMPLES.len());
    let pattern = &INSTRUCTION_SAMPLES[idx];
    let instruction_bytes = pattern.bytes();
    
    let len = instruction_bytes.len().min(15);
    for i in 0..len {
        bytes[i] = Some(instruction_bytes[i]);
    }
    
    bytes
}

pub fn random_instructions(count: usize) -> Vec<[Option<u8>; 15]> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(random_instruction());
    }
    result
}

pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    (0..length)
        .map(|_| {
            let idx = rng().random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}