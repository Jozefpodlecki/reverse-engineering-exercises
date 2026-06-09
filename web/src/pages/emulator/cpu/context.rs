
use iced_x86::Register;

use crate::pages::emulator::{cpu::{control_regs::ControlRegisters, debug_regs::DebugRegisters, msr::Msr, registers::GeneralPurposeRegisters, rflags::Rflags, segment::SegmentRegisters, simd::SimdRegisters}, decoder::decode_instruction, dma::DmaController, error::MemoryError, execution::execute_instruction, iommu::Iommu, memory::MemoryManager, numa::NumaTopology};

#[derive(Default, Clone)]
pub struct CpuContext {
    pub gpr: GeneralPurposeRegisters,
    pub simd: SimdRegisters,
    pub seg: SegmentRegisters,
    pub msr: Msr,
    pub cr: ControlRegisters,
    pub dr: DebugRegisters,
    pub rip: u64,
    pub rflags: Rflags,
    pub dma: Option<DmaController>,
    pub iommu: Option<Iommu>,
    pub numa: Option<NumaTopology>,
}

impl CpuContext {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    pub fn step(&mut self, memory: &mut MemoryManager) -> Result<(), MemoryError> {
        
        let mut code_buffer = [0u8; 15];
        memory.read_bytes(self.rip, &mut code_buffer)?;
        
        match decode_instruction(&code_buffer, self.rip) {
            Ok(instruction) => {
                execute_instruction(self, &instruction, memory)?;
                self.rip += instruction.len() as u64;
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to decode instruction at 0x{:X}: {:?}", self.rip, e);
                Err(MemoryError::PageFault(self.rip))
            }
        }
    }
}