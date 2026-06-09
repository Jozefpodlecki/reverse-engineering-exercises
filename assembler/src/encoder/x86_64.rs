use crate::parser::ast::{Instruction, Operand, MemoryAddress};

pub fn encode(instruction: &Instruction) -> Vec<u8> {
    match instruction {
        Instruction::Syscall => vec![0x0F, 0x05],
        Instruction::Ret => vec![0xC3],
        Instruction::Push(op) => encode_push(&op.value),
        Instruction::Pop(op) => encode_pop(&op.value),
        Instruction::Mov(dest, src) => encode_mov(&dest.value, &src.value),
        Instruction::Sub(dest, src) => encode_sub(&dest.value, &src.value),
        Instruction::Add(dest, src) => encode_add(&dest.value, &src.value),
        Instruction::Xor(dest, src) => encode_xor(&dest.value, &src.value),
    }
}

fn encode_push(op: &Operand) -> Vec<u8> {
    match op {
        Operand::Register(reg) => {
            let reg_code = get_register_code(reg);
            vec![0x50 + reg_code]
        }
        _ => vec![],
    }
}

fn encode_pop(op: &Operand) -> Vec<u8> {
    match op {
        Operand::Register(reg) => {
            let reg_code = get_register_code(reg);
            vec![0x58 + reg_code]
        }
        _ => vec![],
    }
}

fn encode_mov(dest: &Operand, src: &Operand) -> Vec<u8> {
    match (dest, src) {
        (Operand::Register(reg), Operand::Immediate(imm)) => {
            let reg_code = get_register_code(reg);
            let mut bytes = vec![0x48, 0xB8 + reg_code];
            bytes.extend_from_slice(&imm.to_le_bytes());
            bytes
        }
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_code = get_register_code(dest_reg);
            let src_code = get_register_code(src_reg);
            vec![0x48, 0x89, 0xC0 + (src_code << 3) + dest_code]
        }
        (Operand::Memory(mem), Operand::Register(reg)) => {
            encode_mov_memory_to_reg(mem, reg)
        }
        _ => vec![],
    }
}

fn encode_sub(dest: &Operand, src: &Operand) -> Vec<u8> {
    match (dest, src) {
        (Operand::Register(reg), Operand::Immediate(imm)) if reg == "rsp" => {
            let mut bytes = vec![0x48, 0x83, 0xEC];
            bytes.extend_from_slice(&(*imm as u8).to_le_bytes());
            bytes
        }
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_code = get_register_code(dest_reg);
            let src_code = get_register_code(src_reg);
            vec![0x48, 0x29, 0xC0 + (src_code << 3) + dest_code]
        }
        _ => vec![],
    }
}

fn encode_add(dest: &Operand, src: &Operand) -> Vec<u8> {
    match (dest, src) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_code = get_register_code(dest_reg);
            let src_code = get_register_code(src_reg);
            vec![0x48, 0x01, 0xC0 + (src_code << 3) + dest_code]
        }
        _ => vec![],
    }
}

fn encode_xor(dest: &Operand, src: &Operand) -> Vec<u8> {
    match (dest, src) {
        (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
            let dest_code = get_register_code(dest_reg);
            let src_code = get_register_code(src_reg);
            vec![0x48, 0x31, 0xC0 + (src_code << 3) + dest_code]
        }
        _ => vec![],
    }
}

fn encode_mov_memory_to_reg(mem: &MemoryAddress, reg: &str) -> Vec<u8> {
    let reg_code = get_register_code(reg);
    
    let mut bytes = vec![0x4C, 0x89];
    
    let modrm = 0x40 | (reg_code << 3) | 0x04;
    bytes.push(modrm);
    
    let sib = 0x24;
    bytes.push(sib);
    
    bytes.push(mem.displacement as u8);
    
    bytes
}

fn get_register_code(reg: &str) -> u8 {
    match reg {
        "rax" => 0,
        "rcx" => 1,
        "rdx" => 2,
        "rbx" => 3,
        "rsp" => 4,
        "rbp" => 5,
        "rsi" => 6,
        "rdi" => 7,
        "r8" => 8,
        "r9" => 9,
        "r10" => 10,
        "r11" => 11,
        "r12" => 12,
        "r13" => 13,
        "r14" => 14,
        "r15" => 15,
        _ => panic!("Unknown register: {}", reg),
    }
}