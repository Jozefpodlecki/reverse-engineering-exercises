use iced_x86::{Instruction, Code};
use crate::pages::emulator::cpu::CpuContext;
use crate::pages::emulator::error::MemoryError;
use crate::pages::emulator::memory::{MemoryManager};

pub fn execute_instruction(
    cpu: &mut CpuContext, 
    instruction: &Instruction, 
    memory: &mut MemoryManager
) -> Result<(), MemoryError> {
    // match instruction.code() {
    //     Code::Push_r64 => {
    //         let reg = cpu.get_register(instruction.op0_register());
    //         cpu.rsp = cpu.rsp.wrapping_sub(8);
    //         memory.write_u64(cpu.rsp, reg)?;
    //     }
        
    //     Code::Pop_r64 => {
    //         let value = memory.read_u64(cpu.rsp)?;
    //         cpu.set_register(instruction.op0_register(), value);
    //         cpu.rsp = cpu.rsp.wrapping_add(8);
    //     }
        
    //     Code::Mov_r64_imm64 => {
    //         cpu.set_register(instruction.op0_register(), instruction.op1_immediate());
    //     }
        
    //     Code::Add_rm64_r64 => {
    //         let src = cpu.get_register(instruction.op1_register());
    //         let dst = cpu.get_register(instruction.op0_register());
    //         let result = dst.wrapping_add(src);
    //         update_flags_add(cpu, dst, src, result);
    //         cpu.set_register(instruction.op0_register(), result);
    //     }
        
    //     Code::Sub_rm64_r64 => {
    //         let src = cpu.get_register(instruction.op1_register());
    //         let dst = cpu.get_register(instruction.op0_register());
    //         let result = dst.wrapping_sub(src);
    //         update_flags_sub(cpu, dst, src, result);
    //         cpu.set_register(instruction.op0_register(), result);
    //     }
        
    //     _ if instruction.has_lock_prefix() => {
    //         cpu.rflags.lock = true;
    //         let result = execute_instruction(cpu, instruction, memory);
    //         cpu.rflags.lock = false;
    //         result?;
    //     }
        
    //     _ => {
    //         log::warn!("Unimplemented instruction: {:?}", instruction.code());
    //     }
    // }
    
    Ok(())
}