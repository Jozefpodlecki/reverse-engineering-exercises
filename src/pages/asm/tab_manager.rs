use iced_x86::Instruction;
use rand::{RngExt, rng};
use yew::UseStateHandle;

use crate::pages::asm::*;

#[derive(Clone, PartialEq)]
pub struct TabManager(UseStateHandle<TabManagerState>);

#[derive(Default, Clone, PartialEq)]
pub struct TabManagerState {
    tabs: Vec<Tab>,
    active_index: usize,
    emulator: Emulator
}

#[derive(Default, Clone, PartialEq)]
pub struct Emulator {
    is_running: bool
}

pub fn random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    (0..length)
        .map(|_| {
            let idx = rng().random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

impl TabManagerState {
    pub fn new() -> Self {
        Self {
            tabs: vec![
                Tab {
                    id: "1".to_string(),
                    name: random_string(10),
                    decoder_type: DecoderKind::IcedX86,
                    instructions: vec![
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default()
                    ],
                    registers: Registers::default()
                }
            ],
            active_index: 0,
            emulator: Default::default()
        }
    }
}

impl TabManager {
    pub fn new(initial: UseStateHandle<TabManagerState>) -> Self {
        Self(initial)
    }
    
    pub fn tabs(&self) -> Vec<Tab> {
        self.0.tabs.clone()
    }
    
    pub fn active_index(&self) -> usize {
        self.0.active_index
    }
    
    pub fn active_tab(&self) -> &Tab {
        self.0.tabs.get(self.0.active_index).unwrap()
    }

    pub fn active_tab_index(&self) -> usize {
        self.0.active_index
    }
    
    pub fn add_tab(&self) {
        let mut new_state = (*self.0).clone();
        let new_id = (new_state.tabs.len() + 1).to_string();
        let tab= Tab {
            id: new_id,
            name: random_string(10),
            decoder_type: DecoderKind::IcedX86,
            instructions: vec![
                AsmInstruction::default()
            ],
            registers: Registers::default()
        };

        new_state.tabs.push(tab);
        new_state.active_index = new_state.tabs.len() - 1;

        self.0.set(new_state);
    }
    
    pub fn close_tab(&self, id: String) {
        let mut new_state = (*self.0).clone();
        let new_tabs: Vec<Tab> = new_state.tabs
            .into_iter()
            .filter(|t| t.id != id)
            .collect();
        
        if new_tabs.is_empty() {
            new_state.tabs = vec![];
            new_state.active_index = 0;
        } else {
            let new_active = new_state.active_index.min(new_tabs.len() - 1);
            new_state.tabs = new_tabs;
            new_state.active_index = new_active;
        }
        self.0.set(new_state);
    }
    
    pub fn select_tab(&self, index: usize) {
        if index < self.0.tabs.len() {
            let mut new_state = (*self.0).clone();
            new_state.active_index = index;
            self.0.set(new_state);
        }
    }
    
    pub fn update_tab(&self, index: usize, tab: Tab) {
        let mut new_state = (*self.0).clone();
        new_state.tabs[index] = tab;
        self.0.set(new_state);
    }

    pub fn add_instruction(&self, tab_index: usize) {
        let mut new_state = (*self.0).clone();
        new_state.tabs[tab_index].instructions.push(AsmInstruction::default());
        self.0.set(new_state);
    }
    
    pub fn remove_instruction(&self, instr_index: usize, tab_index: usize) {
        let mut new_state = (*self.0).clone();
        
        let instructions = &mut new_state.tabs[tab_index].instructions;
        if instr_index < instructions.len() {
            instructions.remove(instr_index);
            self.0.set(new_state);
        }
    }
    
    pub fn update_instruction(&self, tab_index: usize, instr_index: usize, bytes: Vec<u8>) {
        let mut new_state = (*self.0).clone();
        
        let tab = &new_state.tabs[tab_index];
        let decoder = DecoderFactory::create(&tab.decoder_type, 64);

        let tab = &new_state.tabs[tab_index];
        let decoder = DecoderFactory::create(&tab.decoder_type, 64);
        
        let decoded = decoder.decode(&bytes, tab.registers.rip).ok();
        
        let instructions = &mut new_state.tabs[tab_index].instructions;
        let instr_len = instructions.len();
        let instruction = &mut instructions[instr_index];

        if instr_index < instr_len {
            instruction.bytes = bytes;
        }

        if let Some(decoded) = decoded {
            instruction.asm = decoded.asm;
        }
        else {
            instruction.asm = Default::default();
        }

        self.0.set(new_state);
    }

    pub fn update_tab_register(&self, tab_index: usize, name: String, value: u64) {
        let mut new_state = (*self.0).clone();
        
        if tab_index < new_state.tabs.len() {
            match name.as_str() {
                "RAX" => new_state.tabs[tab_index].registers.rax = value,
                "RBX" => new_state.tabs[tab_index].registers.rbx = value,
                "RCX" => new_state.tabs[tab_index].registers.rcx = value,
                "RDX" => new_state.tabs[tab_index].registers.rdx = value,
                "RSI" => new_state.tabs[tab_index].registers.rsi = value,
                "RDI" => new_state.tabs[tab_index].registers.rdi = value,
                "RBP" => new_state.tabs[tab_index].registers.rbp = value,
                "RSP" => new_state.tabs[tab_index].registers.rsp = value,
                "R8" => new_state.tabs[tab_index].registers.r8 = value,
                "R9" => new_state.tabs[tab_index].registers.r9 = value,
                "R10" => new_state.tabs[tab_index].registers.r10 = value,
                "R11" => new_state.tabs[tab_index].registers.r11 = value,
                "R12" => new_state.tabs[tab_index].registers.r12 = value,
                "R13" => new_state.tabs[tab_index].registers.r13 = value,
                "R14" => new_state.tabs[tab_index].registers.r14 = value,
                "R15" => new_state.tabs[tab_index].registers.r15 = value,
                _ => {}
            }
            self.0.set(new_state);
        }
    }

    pub fn is_running(&self) -> bool {
        self.0.emulator.is_running
    }

     pub fn step_into(&self) {
        let mut new_state = (*self.0).clone();
        let active_index = new_state.active_index;
        let tab = &mut new_state.tabs[active_index];
        
        let rip = tab.registers.rip;
        let instructions = &tab.instructions;
        
        let instr_index = instructions.iter().position(|instr| {
            let instr_end = tab.registers.rip; // Need to track instruction addresses
            false // Simplified - need instruction addresses
        }).unwrap_or(0);
        
        if instr_index < instructions.len() {
            let instr = &instructions[instr_index];
            let decoder = DecoderFactory::create(&tab.decoder_type, 64);
            
            if let Ok(decoded) = decoder.decode(&instr.bytes, rip) {
                Self::execute_instruction(&mut tab.registers, &decoded);
                tab.registers.rip = rip + decoded.size as u64;
            }
        }
        
        self.0.set(new_state);
    }
    
    pub fn step_over(&self) {
        let mut new_state = (*self.0).clone();
        let active_index = new_state.active_index;
        let tab = &mut new_state.tabs[active_index];
        
        let rip = tab.registers.rip;
        let instructions = &tab.instructions;
        let instr_index = instructions.iter().position(|instr| {
            false
        }).unwrap_or(0);
        
        if instr_index < instructions.len() {
            let instr = &instructions[instr_index];
            let decoder = DecoderFactory::create(&tab.decoder_type, 64);
            
            if let Ok(decoded) = decoder.decode(&instr.bytes, rip) {
                let is_call = decoded.mnemonic.to_lowercase().contains("call");
                
                if is_call {
                    tab.registers.rip = rip + decoded.size as u64;
                } else {
                    // Normal step
                    Self::execute_instruction(&mut tab.registers, &decoded);
                    tab.registers.rip = rip + decoded.size as u64;
                }
            }
        }
        
        self.0.set(new_state);
    }
    
    pub fn continue_execution(&self) {
        let mut new_state = (*self.0).clone();
        new_state.emulator.is_running = true;
        self.0.set(new_state);
        

        while self.0.emulator.is_running {
            self.step_into();
        }
    }
    
    pub fn run(&self) {
        let mut new_state = (*self.0).clone();
        new_state.emulator.is_running = true;
        self.0.set(new_state);
        
        let manager = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            while manager.is_running() {
                manager.step_into();

                gloo::timers::future::TimeoutFuture::new(1).await;
            }
        });
    }
    
    pub fn pause(&self) {
        let mut new_state = (*self.0).clone();
        new_state.emulator.is_running = false;
        self.0.set(new_state);
    }

    fn execute_instruction(registers: &mut Registers, decoded: &DecodedInstruction) {
    }

    pub fn registers(&self) -> Vec<(String, u64)> {
        let current_tab = &self.0.tabs[self.0.active_index];

        let registers: Vec<(String, _)> = vec![
            ("RAX".to_string(), current_tab.registers.rax),
            ("RBX".to_string(), current_tab.registers.rbx),
            ("RCX".to_string(), current_tab.registers.rcx),
            ("RDX".to_string(), current_tab.registers.rdx),
            ("RSI".to_string(), current_tab.registers.rsi),
            ("RDI".to_string(), current_tab.registers.rdi),
            ("RBP".to_string(), current_tab.registers.rbp),
            ("RSP".to_string(), current_tab.registers.rsp),
            ("R8".to_string(), current_tab.registers.r8),
            ("R9".to_string(), current_tab.registers.r9),
            ("R10".to_string(), current_tab.registers.r10),
            ("R11".to_string(), current_tab.registers.r11),
            ("R12".to_string(), current_tab.registers.r12),
            ("R13".to_string(), current_tab.registers.r13),
            ("R14".to_string(), current_tab.registers.r14),
            ("R15".to_string(), current_tab.registers.r15),
        ];

        registers
    }
}