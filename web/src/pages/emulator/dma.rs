use std::collections::VecDeque;

use crate::pages::emulator::memory::MemoryManager;

#[derive(Clone)]
pub struct DmaController {
    pub channels: [DmaChannel; 8],
    pub enabled: bool,
}

#[derive(Default, Clone, Copy)]
pub struct DmaChannel {
    pub address: u64,
    pub count: u16,
    pub page: u8,
    pub mode: DmaMode,
    pub autoinit: bool,
}

#[derive(Default, Clone, Copy)]
pub enum DmaMode {
    #[default]
    Demand,   // Transfer until deasserted
    Single,   // One byte/word at a time
    Block,    // Full block
    Cascade,  // For cascading controllers
}

impl DmaController {
    pub fn new() -> Self {
        Self {
            channels: [DmaChannel::default(); 8],
            enabled: false,
        }
    }
    
    pub fn transfer(&mut self, channel: usize, data: &[u8], memory: &mut MemoryManager) {
        if !self.enabled || channel >= 8 {
            return;
        }
        
        let ch = &self.channels[channel];
        let address = (ch.page as u64) << 16 | ch.address;
        
        // Transfer data to/from memory
        memory.write_bytes(address, data);
    }
}