use crate::pages::emulator::cpu::CpuContext;


#[derive(Default, Clone)]
pub struct CpuCore {
    pub apic_id: u32,
    pub context: CpuContext
}

pub struct EmulatorState {
    pub cores: Vec<CpuCore>
}