use std::collections::HashMap;

use crate::{Spanned, ast::{ConditionCode, Prefix}, parser::ast::{Instruction, MemoryAddress, Operand}};

pub struct Encoder;

impl Encoder {

    
    // pub fn encode(instruction: &Instruction) -> Vec<u8> {
    //     match instruction {
    //         Instruction::Syscall => vec![0x0F, 0x05],
    //         Instruction::Sysenter => vec![0x0F, 0x34],
    //         Instruction::Sysexit => vec![0x0F, 0x35],
    //         Instruction::Ret => vec![0xC3],
    //         Instruction::Nop => vec![0x90],
    //         Instruction::CpuId => vec![0x0F, 0xA2],
    //         Instruction::Hlt => vec![0xF4],
    //         Instruction::Int3 => vec![0xCC],
    //         Instruction::Rdtsc => vec![0x0F, 0x31],
    //         Instruction::Push(op) => Self::encode_push(op),
    //         Instruction::Pop(op) => Self::encode_pop(op),
    //         Instruction::Mov(dest, src) => Self::encode_mov(dest, src),
    //         Instruction::Add(dest, src) => Self::encode_add(dest, src),
    //         Instruction::Sub(dest, src) => Self::encode_sub(dest, src),
    //         Instruction::Xor(dest, src) => Self::encode_xor(dest, src),
    //         Instruction::And(dest, src) => Self::encode_and(dest, src),
    //         Instruction::Or(dest, src) => Self::encode_or(dest, src),
    //         Instruction::Inc(op) => Self::encode_inc(op),
    //         Instruction::Dec(op) => Self::encode_dec(op),
    //         Instruction::Neg(op) => Self::encode_neg(op),
    //         Instruction::Not(op) => Self::encode_not(op),
    //         Instruction::Cmp(dest, src) => Self::encode_cmp(dest, src),
    //         Instruction::Test(dest, src) => Self::encode_test(dest, src),
    //         Instruction::Lea(dest, src) => Self::encode_lea(dest, src),
    //         Instruction::Jmp(target) => Self::encode_jmp(target),
    //         Instruction::Jcc(cc, target) => Self::encode_jcc(cc, target),
    //         Instruction::Call(target) => Self::encode_call(target),
    //         Instruction::Movsd(dest, src) => Self::encode_movsd(dest, src),
    //         Instruction::Movss(dest, src) => Self::encode_movss(dest, src),
    //         Instruction::Movaps(dest, src) => Self::encode_movaps(dest, src),
    //         Instruction::Movapd(dest, src) => Self::encode_movapd(dest, src),
    //         Instruction::Addpd(dest, src) => Self::encode_addpd(dest, src),
    //         Instruction::Addps(dest, src) => Self::encode_addps(dest, src),
    //         Instruction::AddSd(dest, src) => Self::encode_addsd(dest, src),
    //         Instruction::AddSs(dest, src) => Self::encode_addss(dest, src),
    //         Instruction::Vaddpd(dest, src1, src2) => Self::encode_vaddpd(dest, src1, src2),
    //         Instruction::Vaddps(dest, src1, src2) => Self::encode_vaddps(dest, src1, src2),
    //         Instruction::Enter(imm16, imm8) => Self::encode_enter(imm16, imm8),
    //         Instruction::Leave => vec![0xC9],
    //         Instruction::Movsx(dest, src) => Self::encode_movsx(dest, src),
    //         Instruction::Movzx(dest, src) => Self::encode_movzx(dest, src),
    //         Instruction::Xchg(dest, src) => Self::encode_xchg(dest, src),
    //         Instruction::Mul(op) => Self::encode_mul(op),
    //         Instruction::Imul(op) => Self::encode_imul(op),
    //         Instruction::Div(op) => Self::encode_div(op),
    //         Instruction::Idiv(op) => Self::encode_idiv(op),
    //         Instruction::Shl(dest, count) => Self::encode_shift("shl", dest, count),
    //         Instruction::Shr(dest, count) => Self::encode_shift("shr", dest, count),
    //         Instruction::Sal(dest, count) => Self::encode_shift("sal", dest, count),
    //         Instruction::Sar(dest, count) => Self::encode_shift("sar", dest, count),
    //         Instruction::Rol(dest, count) => Self::encode_shift("rol", dest, count),
    //         Instruction::Ror(dest, count) => Self::encode_shift("ror", dest, count),
    //         Instruction::Rcl(dest, count) => Self::encode_shift("rcl", dest, count),
    //         Instruction::Rcr(dest, count) => Self::encode_shift("rcr", dest, count),
    //         Instruction::Bt(dest, src) => Self::encode_bit_test("bt", dest, src),
    //         Instruction::Bts(dest, src) => Self::encode_bit_test("bts", dest, src),
    //         Instruction::Btr(dest, src) => Self::encode_bit_test("btr", dest, src),
    //         Instruction::Btc(dest, src) => Self::encode_bit_test("btc", dest, src),
    //         Instruction::Bsf(dest, src) => Self::encode_bsf(dest, src),
    //         Instruction::Bsr(dest, src) => Self::encode_bsr(dest, src),
    //         Instruction::Popcnt(dest, src) => Self::encode_popcnt(dest, src),
    //         Instruction::Lzcnt(dest, src) => Self::encode_lzcnt(dest, src),
    //         Instruction::Tzcnt(dest, src) => Self::encode_tzcnt(dest, src),
    //         Instruction::Cmove(dest, src) => Self::encode_cmov("cmove", dest, src),
    //         Instruction::Cmovne(dest, src) => Self::encode_cmov("cmovne", dest, src),
    //         Instruction::Cmovg(dest, src) => Self::encode_cmov("cmovg", dest, src),
    //         Instruction::Cmovge(dest, src) => Self::encode_cmov("cmovge", dest, src),
    //         Instruction::Cmovl(dest, src) => Self::encode_cmov("cmovl", dest, src),
    //         Instruction::Cmovle(dest, src) => Self::encode_cmov("cmovle", dest, src),
    //         Instruction::Cmova(dest, src) => Self::encode_cmov("cmova", dest, src),
    //         Instruction::Cmovae(dest, src) => Self::encode_cmov("cmovae", dest, src),
    //         Instruction::Cmovb(dest, src) => Self::encode_cmov("cmovb", dest, src),
    //         Instruction::Cmovbe(dest, src) => Self::encode_cmov("cmovbe", dest, src),
    //         Instruction::Cmovs(dest, src) => Self::encode_cmov("cmovs", dest, src),
    //         Instruction::Cmovns(dest, src) => Self::encode_cmov("cmovns", dest, src),
    //         Instruction::Cmovz(dest, src) => Self::encode_cmov("cmovz", dest, src),
    //         Instruction::Cmovnz(dest, src) => Self::encode_cmov("cmovnz", dest, src),
    //         Instruction::Movsb => Self::encode_string("movsb"),
    //         Instruction::Movsw => Self::encode_string("movsw"),
    //         Instruction::Movsq => Self::encode_string("movsq"),
    //         Instruction::Cmpsb => Self::encode_string("cmpsb"),
    //         Instruction::Cmpsw => Self::encode_string("cmpsw"),
    //         Instruction::Cmpsd => Self::encode_string("cmpsd"),
    //         Instruction::Cmpsq => Self::encode_string("cmpsq"),
    //         Instruction::Scasb => Self::encode_string("scasb"),
    //         Instruction::Scasw => Self::encode_string("scasw"),
    //         Instruction::Scasd => Self::encode_string("scasd"),
    //         Instruction::Scasq => Self::encode_string("scasq"),
    //         Instruction::Stosb => Self::encode_string("stosb"),
    //         Instruction::Stosw => Self::encode_string("stosw"),
    //         Instruction::Stosd => Self::encode_string("stosd"),
    //         Instruction::Stosq => Self::encode_string("stosq"),
    //         Instruction::Lodsb => Self::encode_string("lodsb"),
    //         Instruction::Lodsw => Self::encode_string("lodsw"),
    //         Instruction::Lodsd => Self::encode_string("lodsd"),
    //         Instruction::Lodsq => Self::encode_string("lodsq"),
    //         Instruction::Mfence => Self::encode_fence("mfence"),
    //         Instruction::Lfence => Self::encode_fence("lfence"),
    //         Instruction::Sfence => Self::encode_fence("sfence"),
    //         Instruction::Movs => Self::encode_string("movsd"),
    //         Instruction::Prefixed(items, instruction) => Self::encode_prefixed(items, instruction),
    //         Instruction::Label(_) => todo!(),
    //         Instruction::Prefetch(spanned) => todo!(),
    //         Instruction::Prefetchnta(spanned) => todo!(),
    //         Instruction::Prefetcht0(spanned) => todo!(),
    //         Instruction::Prefetcht1(spanned) => todo!(),
    //         Instruction::Prefetcht2(spanned) => todo!(),
    //         Instruction::Prefetchw(spanned) => todo!(),
    //     }
    // }

    // pub fn encode_with_labels(instruction: &Instruction, symbols: &HashMap<String, usize>, current_offset: u64) -> Vec<u8> {
    //     match instruction {
    //         // Instruction::Jmp(Spanned { value: Operand::Label(name), .. }) => {
    //         //     if let Some(&target) = symbols.get(name) {
    //         //         let offset = (target as i64) - (current_offset as i64 + 5);
    //         //         let mut bytes = vec![0xE9];
    //         //         bytes.extend_from_slice(&(offset as u32).to_le_bytes());
    //         //         bytes
    //         //     } else {
    //         //         vec![0xE9, 0x00, 0x00, 0x00, 0x00]
    //         //     }
    //         // }
    //         Instruction::Jmp(Spanned { value: Operand::Label(name), .. }) => {
                
    //             if let Some(&target) = symbols.get(name) {
    //                 let offset = (target as i64) - (current_offset as i64 + 5);
    //                 let mut bytes = vec![0xE9];
    //                 bytes.extend_from_slice(&(offset as u32).to_le_bytes());
    //                 bytes
    //             } else {
    //                 vec![0xE9, 0x00, 0x00, 0x00, 0x00]
    //             }
    //         }
    //         Instruction::Call(Spanned { value: Operand::Label(name), .. }) => {
    //             if let Some(&target) = symbols.get(name) {
    //                 let offset = (target as i64) - (current_offset as i64 + 5);
    //                 let mut bytes = vec![0xE8];
    //                 bytes.extend_from_slice(&(offset as u32).to_le_bytes());
    //                 bytes
    //             } else {
    //                 vec![0xE8, 0x00, 0x00, 0x00, 0x00]
    //             }
    //         }
    //         Instruction::Jcc(cc, Spanned { value: Operand::Label(name), .. }) => {
    //             if let Some(&target) = symbols.get(name) {
    //                 let opcode = match cc {
    //                     ConditionCode::E | ConditionCode::Z => 0x84,
    //                     ConditionCode::NE | ConditionCode::NZ => 0x85,
    //                     ConditionCode::G => 0x8F,
    //                     ConditionCode::GE => 0x8D,
    //                     ConditionCode::L => 0x8C,
    //                     ConditionCode::LE => 0x8E,
    //                     ConditionCode::A => 0x87,
    //                     ConditionCode::AE => 0x83,
    //                     ConditionCode::B => 0x82,
    //                     ConditionCode::BE => 0x86,
    //                     _ => 0x84,
    //                 };
    //                 let offset = (target as i64) - (current_offset as i64 + 6);
    //                 let mut bytes = vec![0x0F, opcode];
    //                 bytes.extend_from_slice(&(offset as u32).to_le_bytes());
    //                 bytes
    //             } else {
    //                 vec![0x0F, 0x84, 0x00, 0x00, 0x00, 0x00]
    //             }
    //         }
    //         _ => Self::encode(instruction),
    //     }
    // }

    // fn encode_prefixed(prefixes: &[Prefix], instruction: &Instruction) -> Vec<u8> {
    //     let mut bytes = Vec::new();
    //     for prefix in prefixes {
    //         bytes.push(match prefix {
    //             Prefix::Rep => 0xF3,
    //             Prefix::Repne => 0xF2,
    //             Prefix::Lock => 0xF0,
    //         });
    //     }
    //     bytes.extend(Self::encode(instruction));
    //     bytes
    // }

    // fn encode_push(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             // r8 through r15 need REX prefix
    //             if reg.starts_with('r') && reg.len() >= 2 && reg != "rax" && reg != "rcx" && reg != "rdx" && 
    //             reg != "rbx" && reg != "rsp" && reg != "rbp" && reg != "rsi" && reg != "rdi" {
    //                 vec![0x41, 0x50 + (reg_code - 8)]
    //             } else {
    //                 vec![0x50 + reg_code]
    //             }
    //         }
    //         Operand::Immediate(imm) => {
    //             let mut bytes = vec![0x68];
    //             bytes.extend_from_slice(&(*imm as u32).to_le_bytes());
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_pop(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             if reg.starts_with('r') && reg.len() == 2 && reg != "rsp" && reg != "rbp" && reg != "rsi" && reg != "rdi" {
    //                 vec![0x41, 0x58 + (reg_code - 8)]
    //             } else {
    //                 vec![0x58 + reg_code]
    //             }
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_mov(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let mut bytes = vec![0x48, 0xB8 + reg_code];
    //             bytes.extend_from_slice(&imm.to_le_bytes());
    //             bytes
    //         }
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x89, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Memory(mem)) => {
    //             // mov reg, [mem] - destination is register, source is memory
    //             Self::encode_mov_reg_from_memory(reg, mem)
    //         }
    //         (Operand::Memory(mem), Operand::Register(reg)) => {
    //             // mov [mem], reg - destination is memory, source is register
    //             Self::encode_mov_memory_to_reg(mem, reg)
    //         }
    //         (Operand::Memory(mem), Operand::Immediate(imm)) => {
    //             Self::encode_mov_memory_imm(mem, *imm)
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_mov_reg_from_memory(reg: &str, mem: &MemoryAddress) -> Vec<u8> {
    //     let reg_code = Self::get_register_code(reg);
    //     let base_code = Self::get_register_code(&mem.base);
    //     let mut bytes = vec![0x48, 0x8B];  // 0x8B is correct
        
    //     let modrm = if mem.displacement == 0 {
    //         0x00 | (reg_code << 3) | base_code
    //     } else {
    //         0x40 | (reg_code << 3) | base_code
    //     };
    //     bytes.push(modrm);
        
    //     if mem.displacement != 0 {
    //         bytes.push(mem.displacement as u8);
    //     }
        
    //     bytes
    // }

    // fn encode_mov_memory_imm(mem: &MemoryAddress, imm: i64) -> Vec<u8> {
    //     let base_code = Self::get_register_code(&mem.base);
    //     let mut bytes = vec![0x48, 0xC7];
        
    //     let modrm = if mem.displacement == 0 {
    //         0x00 | (0x00 << 3) | base_code
    //     } else {
    //         0x40 | (0x00 << 3) | base_code
    //     };
    //     bytes.push(modrm);
        
    //     if mem.displacement != 0 {
    //         bytes.push(mem.displacement as u8);
    //     }
        
    //     // For simplicity, encode as 32-bit immediate (most common)
    //     // For 64-bit, would need 8 bytes and different opcode
    //     bytes.extend_from_slice(&(imm as u32).to_le_bytes());
    //     bytes
    // }

    // fn encode_mov_memory_to_reg(mem: &MemoryAddress, reg: &str) -> Vec<u8> {
    //     let reg_code = Self::get_register_code(reg);
    //     let base_code = Self::get_register_code(&mem.base);
    //     let mut bytes = vec![0x48, 0x89];  // 0x89 is correct for mov rm64, r64
        
    //     let modrm = if mem.displacement == 0 {
    //         0x00 | (reg_code << 3) | base_code
    //     } else {
    //         0x40 | (reg_code << 3) | base_code
    //     };
    //     bytes.push(modrm);
        
    //     if mem.displacement != 0 {
    //         bytes.push(mem.displacement as u8);
    //     }
        
    //     bytes
    // }

    // fn encode_mov_reg_to_memory(reg: &str, mem: &MemoryAddress) -> Vec<u8> {
    //     let reg_code = Self::get_register_code(reg);
    //     let base_code = Self::get_register_code(&mem.base);
    //     let mut bytes = vec![0x48, 0x89];
        
    //     let modrm = if mem.displacement == 0 {
    //         0x00 | (reg_code << 3) | base_code
    //     } else {
    //         0x40 | (reg_code << 3) | base_code
    //     };
    //     bytes.push(modrm);
        
    //     if mem.displacement != 0 {
    //         bytes.push(mem.displacement as u8);
    //     }
        
    //     bytes
    // }

    // fn encode_sub(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(reg), Operand::Immediate(imm)) if reg == "rsp" => {
    //             let mut bytes = vec![0x48, 0x83, 0xEC];
    //             bytes.extend_from_slice(&(*imm as u8).to_le_bytes());
    //             bytes
    //         }
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x29, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_add(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x01, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             if *imm >= -128 && *imm <= 127 {
    //                 vec![0x48, 0x83, 0xC0 + reg_code, *imm as u8]
    //             } else {
    //                 let mut bytes = vec![0x48, 0x81, 0xC0 + reg_code];
    //                 bytes.extend_from_slice(&imm.to_le_bytes());
    //                 bytes
    //             }
    //         }
    //         (Operand::Memory(mem), Operand::Immediate(imm)) => {
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x83];
                
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (0x00 << 3) | base_code
    //             } else {
    //                 0x40 | (0x00 << 3) | base_code
    //             };
    //             bytes.push(modrm);
                
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
                
    //             bytes.push(*imm as u8);
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_xor(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x31, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             if *imm >= -128 && *imm <= 127 {
    //                 vec![0x48, 0x83, 0xF0 + reg_code, *imm as u8]
    //             } else {
    //                 let mut bytes = vec![0x48, 0x81, 0xF0 + reg_code];
    //                 bytes.extend_from_slice(&imm.to_le_bytes());
    //                 bytes
    //             }
    //         }
    //         (Operand::Memory(mem), Operand::Register(reg)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x31];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (reg_code << 3) | base_code
    //             } else {
    //                 0x40 | (reg_code << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_and(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x21, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             if *imm >= -128 && *imm <= 127 {
    //                 vec![0x48, 0x83, 0xE0 + reg_code, *imm as u8]
    //             } else {
    //                 let mut bytes = vec![0x48, 0x81, 0xE0 + reg_code];
    //                 bytes.extend_from_slice(&imm.to_le_bytes());
    //                 bytes
    //             }
    //         }
    //         (Operand::Memory(mem), Operand::Immediate(imm)) => {
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x83];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (0x04 << 3) | base_code
    //             } else {
    //                 0x40 | (0x04 << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes.push(*imm as u8);
    //             bytes
    //         }
    //         (Operand::Memory(mem), Operand::Register(reg)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x21];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (reg_code << 3) | base_code
    //             } else {
    //                 0x40 | (reg_code << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_or(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x09, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             println!("imm={}, fits 8-bit={}", imm, *imm >= -128 && *imm <= 127);
    //             if *imm >= -128 && *imm <= 127 {
    //                 vec![0x48, 0x83, 0xC8 + reg_code, *imm as u8]
    //             } else {
    //                 let mut bytes = vec![0x48, 0x81, 0xC8 + reg_code];
    //                 bytes.extend_from_slice(&imm.to_le_bytes());
    //                 bytes
    //             }
    //         }
    //         (Operand::Memory(mem), Operand::Immediate(imm)) => {
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x83];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (0x01 << 3) | base_code
    //             } else {
    //                 0x40 | (0x01 << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes.push(*imm as u8);
    //             bytes
    //         }
    //         (Operand::Memory(mem), Operand::Register(reg)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x09];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (reg_code << 3) | base_code
    //             } else {
    //                 0x40 | (reg_code << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_inc(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xFF, 0xC0 + reg_code]
    //         }
    //         Operand::Memory(mem) => {
    //             Self::encode_inc_memory(mem)
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_inc_memory(mem: &MemoryAddress) -> Vec<u8> {
    //     let base_code = Self::get_register_code(&mem.base);
    //     let mut bytes = vec![0x48, 0xFF];
        
    //     let modrm = 0x00 | (0x00 << 3) | base_code;
    //     bytes.push(modrm);
        
    //     if mem.displacement != 0 {
    //         bytes[1] = 0xFF;
    //         bytes.push(mem.displacement as u8);
    //     }
        
    //     bytes
    // }

    // fn encode_dec(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xFF, 0xC8 + reg_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_neg(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xF7, 0xD8 + reg_code]
    //         }
    //         Operand::Memory(mem) => {
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0xF7];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (0x03 << 3) | base_code
    //             } else {
    //                 0x40 | (0x03 << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_not(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xF7, 0xD0 + reg_code]
    //         }
    //         Operand::Memory(mem) => {
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0xF7];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (0x02 << 3) | base_code
    //             } else {
    //                 0x40 | (0x02 << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_cmp(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x39, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let mut bytes = vec![0x48, 0x81, 0xF8 + reg_code];
    //             bytes.extend_from_slice(&imm.to_le_bytes());
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_test(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x85, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Register(reg), Operand::Immediate(imm)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let mut bytes = vec![0x48, 0xF7, 0xC0 + reg_code];
    //             bytes.extend_from_slice(&imm.to_le_bytes());
    //             bytes
    //         }
    //         (Operand::Memory(mem), Operand::Immediate(imm)) => {
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0xF7];
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (0x00 << 3) | base_code
    //             } else {
    //                 0x40 | (0x00 << 3) | base_code
    //             };
    //             bytes.push(modrm);
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
    //             bytes.extend_from_slice(&imm.to_le_bytes());
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_lea(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Memory(mem)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x8D];
    //             let modrm = 0x40 | (dest_code << 3) | base_code;
    //             bytes.push(modrm);
    //             bytes.push(0x24);
    //             bytes.push(mem.displacement as u8);
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_jmp(target: &Spanned<Operand>) -> Vec<u8> {
    //     match &target.value {
    //         Operand::Label(name) => {
    //             vec![0xE9, 0x00, 0x00, 0x00, 0x00]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_jcc(cc: &ConditionCode, target: &Spanned<Operand>) -> Vec<u8> {
    //     let opcode = match cc {
    //         ConditionCode::E | ConditionCode::Z => 0x84,
    //         ConditionCode::NE | ConditionCode::NZ => 0x85,
    //         ConditionCode::G => 0x8F,
    //         ConditionCode::GE => 0x8D,
    //         ConditionCode::L => 0x8C,
    //         ConditionCode::LE => 0x8E,
    //         ConditionCode::A => 0x87,
    //         ConditionCode::AE => 0x83,
    //         ConditionCode::B => 0x82,
    //         ConditionCode::BE => 0x86,
    //         _ => 0x84,
    //     };
    //     vec![0x0F, opcode, 0x00, 0x00, 0x00, 0x00]
    // }

    // fn encode_call(target: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xE8, 0x00, 0x00, 0x00, 0x00]
    // }

    // fn encode_movsd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xF2, 0x0F, 0x10, 0xC0]
    // }

    // fn encode_movss(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xF3, 0x0F, 0x10, 0xC0]
    // }

    // fn encode_movaps(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0x0F, 0x28, 0xC0]
    // }

    // fn encode_movapd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0x66, 0x0F, 0x28, 0xC0]
    // }

    // fn encode_addpd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0x66, 0x0F, 0x58, 0xC0]
    // }

    // fn encode_addps(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0x0F, 0x58, 0xC0]
    // }

    // fn encode_addsd(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xF2, 0x0F, 0x58, 0xC0]
    // }

    // fn encode_addss(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xF3, 0x0F, 0x58, 0xC0]
    // }

    // fn encode_vaddpd(dest: &Spanned<Operand>, src1: &Spanned<Operand>, src2: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xC5, 0xF1, 0x58, 0xC0]
    // }

    // fn encode_vaddps(dest: &Spanned<Operand>, src1: &Spanned<Operand>, src2: &Spanned<Operand>) -> Vec<u8> {
    //     vec![0xC5, 0xF0, 0x58, 0xC0]
    // }

    // fn encode_enter(imm16: &Spanned<Operand>, imm8: &Spanned<Operand>) -> Vec<u8> {
    //     match (&imm16.value, &imm8.value) {
    //         (Operand::Immediate(alloc_bytes), Operand::Immediate(nest_level)) => {
    //             let mut bytes = vec![0xC8];
    //             bytes.extend_from_slice(&(*alloc_bytes as u16).to_le_bytes());
    //             bytes.push(*nest_level as u8);
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_movsx(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x63, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_movzx(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x0F, 0xB6, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_xchg(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x87, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         (Operand::Memory(mem), Operand::Register(reg)) => {
    //             let reg_code = Self::get_register_code(reg);
    //             let base_code = Self::get_register_code(&mem.base);
    //             let mut bytes = vec![0x48, 0x87];
                
    //             let modrm = if mem.displacement == 0 {
    //                 0x00 | (reg_code << 3) | base_code
    //             } else {
    //                 0x40 | (reg_code << 3) | base_code
    //             };
    //             bytes.push(modrm);
                
    //             if mem.displacement != 0 {
    //                 bytes.push(mem.displacement as u8);
    //             }
                
    //             bytes
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_mul(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xF7, 0xE0 + reg_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_imul(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xF7, 0xE8 + reg_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_div(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xF7, 0xF0 + reg_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_idiv(op: &Spanned<Operand>) -> Vec<u8> {
    //     match &op.value {
    //         Operand::Register(reg) => {
    //             let reg_code = Self::get_register_code(reg);
    //             vec![0x48, 0xF7, 0xF8 + reg_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_shift(op: &str, dest: &Spanned<Operand>, count: &Spanned<Operand>) -> Vec<u8> {
    //     let (base_opcode, reg_code) = match op {
    //         "shl" | "sal" => (0xE0, 4),
    //         "shr" => (0xE8, 5),
    //         "sar" => (0xF8, 7),
    //         "rol" => (0xC0, 0),
    //         "ror" => (0xC8, 1),
    //         "rcl" => (0xD0, 2),
    //         "rcr" => (0xD8, 3),
    //         _ => return vec![],
    //     };
        
    //     match (&dest.value, &count.value) {
    //         (Operand::Register(dest_reg), Operand::Immediate(1)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             vec![0x48, 0xD1, base_opcode + dest_code]
    //         }
    //         (Operand::Register(dest_reg), Operand::Immediate(imm)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             vec![0x48, 0xC1, base_opcode + dest_code, *imm as u8]
    //         }
    //         (Operand::Register(dest_reg), Operand::Register(count_reg)) if count_reg == "cl" => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             vec![0x48, 0xD3, base_opcode + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_bit_test(op: &str, dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     let opcode = match op {
    //         "bt" => 0xA3,
    //         "bts" => 0xAB,
    //         "btr" => 0xB3,
    //         "btc" => 0xBB,
    //         _ => return vec![],
    //     };
        
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x0F, opcode, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_bsf(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x0F, 0xBC, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_bsr(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x0F, 0xBD, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_popcnt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0xF3, 0x48, 0x0F, 0xB8, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_lzcnt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0xF3, 0x48, 0x0F, 0xBD, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_tzcnt(dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0xF3, 0x48, 0x0F, 0xBC, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_cmov(op: &str, dest: &Spanned<Operand>, src: &Spanned<Operand>) -> Vec<u8> {
    //     let opcode = match op {
    //         "cmove" | "cmovz" => 0x44,
    //         "cmovne" | "cmovnz" => 0x45,
    //         "cmovg" => 0x4F,
    //         "cmovge" => 0x4D,
    //         "cmovl" => 0x4C,
    //         "cmovle" => 0x4E,
    //         "cmova" => 0x47,
    //         "cmovae" => 0x43,
    //         "cmovb" => 0x42,
    //         "cmovbe" => 0x46,
    //         "cmovs" => 0x48,
    //         "cmovns" => 0x49,
    //         _ => return vec![],
    //     };
        
    //     match (&dest.value, &src.value) {
    //         (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
    //             let dest_code = Self::get_register_code(dest_reg);
    //             let src_code = Self::get_register_code(src_reg);
    //             vec![0x48, 0x0F, opcode, 0xC0 + (src_code << 3) + dest_code]
    //         }
    //         _ => vec![],
    //     }
    // }

    // fn encode_string(op: &str) -> Vec<u8> {
    //     match op {
    //         "movsb" => vec![0x48, 0xA4],
    //         "movsw" => vec![0x48, 0x66, 0xA5],
    //         "movsd" => vec![0x48, 0xA5],
    //         "movsq" => vec![0x48, 0xA5],
    //         "cmpsb" => vec![0x48, 0xA6],
    //         "cmpsw" => vec![0x48, 0x66, 0xA7],
    //         "cmpsd" => vec![0x48, 0xA7],
    //         "cmpsq" => vec![0x48, 0xA7],
    //         "scasb" => vec![0x48, 0xAE],
    //         "scasw" => vec![0x48, 0x66, 0xAF],
    //         "scasd" => vec![0x48, 0xAF],
    //         "scasq" => vec![0x48, 0xAF],
    //         "stosb" => vec![0xAA],
    //         "stosw" => vec![0x48, 0x66, 0xAB],
    //         "stosd" => vec![0x48, 0xAB],
    //         "stosq" => vec![0x48, 0xAB],
    //         "lodsb" => vec![0x48, 0xAC],
    //         "lodsw" => vec![0x48, 0x66, 0xAD],
    //         "lodsd" => vec![0x48, 0xAD],
    //         "lodsq" => vec![0x48, 0xAD],
    //         _ => vec![],
    //     }
    // }

    // fn encode_fence(op: &str) -> Vec<u8> {
    //     match op {
    //         "mfence" => vec![0x0F, 0xAE, 0xF0],
    //         "lfence" => vec![0x0F, 0xAE, 0xE8],
    //         "sfence" => vec![0x0F, 0xAE, 0xF8],
    //         _ => vec![],
    //     }
    // }

    // fn get_register_code(reg: &str) -> u8 {
    //     match reg {
    //         // General purpose registers
    //         "rax" | "eax" | "ax" | "al" => 0,
    //         "rcx" | "ecx" | "cx" | "cl" => 1,
    //         "rdx" | "edx" | "dx" | "dl" => 2,
    //         "rbx" | "ebx" | "bx" | "bl" => 3,
    //         "rsp" | "esp" | "sp" | "spl" => 4,
    //         "rbp" | "ebp" | "bp" | "bpl" => 5,
    //         "rsi" | "esi" | "si" | "sil" => 6,
    //         "rdi" | "edi" | "di" | "dil" => 7,
    //         "r8" => 8, "r9" => 9, "r10" => 10, "r11" => 11,
    //         "r12" => 12, "r13" => 13, "r14" => 14, "r15" => 15,
            
    //         // SIMD registers
    //         "xmm0" => 0, "xmm1" => 1, "xmm2" => 2, "xmm3" => 3,
    //         "xmm4" => 4, "xmm5" => 5, "xmm6" => 6, "xmm7" => 7,
    //         "xmm8" => 8, "xmm9" => 9, "xmm10" => 10, "xmm11" => 11,
    //         "xmm12" => 12, "xmm13" => 13, "xmm14" => 14, "xmm15" => 15,
            
    //         "ymm0" => 0, "ymm1" => 1, "ymm2" => 2, "ymm3" => 3,
    //         "ymm4" => 4, "ymm5" => 5, "ymm6" => 6, "ymm7" => 7,
    //         "ymm8" => 8, "ymm9" => 9, "ymm10" => 10, "ymm11" => 11,
    //         "ymm12" => 12, "ymm13" => 13, "ymm14" => 14, "ymm15" => 15,
            
    //         "zmm0" => 0, "zmm1" => 1, "zmm2" => 2, "zmm3" => 3,
    //         "zmm4" => 4, "zmm5" => 5, "zmm6" => 6, "zmm7" => 7,
    //         "zmm8" => 8, "zmm9" => 9, "zmm10" => 10, "zmm11" => 11,
    //         "zmm12" => 12, "zmm13" => 13, "zmm14" => 14, "zmm15" => 15,
    //         "zmm16" => 16, "zmm17" => 17, "zmm18" => 18, "zmm19" => 19,
    //         "zmm20" => 20, "zmm21" => 21, "zmm22" => 22, "zmm23" => 23,
    //         "zmm24" => 24, "zmm25" => 25, "zmm26" => 26, "zmm27" => 27,
    //         "zmm28" => 28, "zmm29" => 29, "zmm30" => 30, "zmm31" => 31,
            
    //         // Control registers
    //         "cr0" => 0, "cr1" => 1, "cr2" => 2, "cr3" => 3,
    //         "cr4" => 4, "cr5" => 5, "cr6" => 6, "cr7" => 7,
    //         "cr8" => 8, "cr9" => 9, "cr10" => 10, "cr11" => 11,
    //         "cr12" => 12, "cr13" => 13, "cr14" => 14, "cr15" => 15,
            
    //         // Debug registers
    //         "dr0" => 0, "dr1" => 1, "dr2" => 2, "dr3" => 3,
    //         "dr4" => 4, "dr5" => 5, "dr6" => 6, "dr7" => 7,
    //         "dr8" => 8, "dr9" => 9, "dr10" => 10, "dr11" => 11,
    //         "dr12" => 12, "dr13" => 13, "dr14" => 14, "dr15" => 15,
            
    //         // Segment registers
    //         "es" => 0, "cs" => 1, "ss" => 2, "ds" => 3,
    //         "fs" => 4, "gs" => 5,
            
    //         // Test registers (obsolete, but some CPUs)
    //         "tr0" => 0, "tr1" => 1, "tr2" => 2, "tr3" => 3,
    //         "tr4" => 4, "tr5" => 5, "tr6" => 6, "tr7" => 7,
            
    //         // MMX registers
    //         "mm0" => 0, "mm1" => 1, "mm2" => 2, "mm3" => 3,
    //         "mm4" => 4, "mm5" => 5, "mm6" => 6, "mm7" => 7,
            
    //         _ => panic!("Unknown register: {}", reg),
    //     }
    // }
}