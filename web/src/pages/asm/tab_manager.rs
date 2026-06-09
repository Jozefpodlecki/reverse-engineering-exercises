use iced_x86::Instruction;
use rand::{RngExt, rng};
use yew::UseStateHandle;

use crate::pages::asm::*;

#[derive(Clone, PartialEq)]
pub struct TabManager(UseStateHandle<TabManagerState>);

#[derive(Default, Clone, PartialEq)]
pub struct TabManagerState {
    tabs: Vec<Tab>,
    active_index: usize
}

impl TabManagerState {
    pub fn new() -> Self {
        Self {
            tabs: vec![
                Tab {
                    id: rng().random(),
                    name: random_string(10),
                    decoder: DecoderFactory::create("iced-x86", 64),
                    instructions: Default::default(),
                    registers: Default::default(),
                    memory: Default::default(),
                    sub_tab: SubTab::Code
                }
            ],
            active_index: 0,
        }
    }

    pub fn new_with_instr() -> Self {
        Self {
            tabs: vec![
                Tab {
                    id: rng().random(),
                    name: random_string(10),
                    decoder: DecoderFactory::create("iced-x86", 64),
                    instructions: AsmInstructions::from(vec![
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                        AsmInstruction::default(),
                    ]),
                    registers: Default::default(),
                    memory: Default::default(),
                    sub_tab: SubTab::Code
                }
            ],
            active_index: 0,
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
        let new_id = rng().random();
        let tab= Tab {
            id: new_id,
            name: random_string(10),
            decoder: DecoderFactory::create("iced-x86", 64),
            instructions: AsmInstructions::from(vec![
                AsmInstruction::default()
            ]),
            registers: Default::default(),
            memory: Default::default(),
            sub_tab: SubTab::Code
        };

        new_state.tabs.push(tab);
        new_state.active_index = new_state.tabs.len() - 1;

        self.0.set(new_state);
    }
    
    pub fn close_tab(&self, id: u32) {
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

    pub fn update_decoder(&self, kind: &str, index: usize) {
        let mut new_state = (*self.0).clone();
        new_state.tabs[index].decoder = DecoderFactory::create(kind, 64);
        self.0.set(new_state);
    }
    
    pub fn update_tab(&self, index: usize, tab: Tab) {
        let mut new_state = (*self.0).clone();
        new_state.tabs[index] = tab;
        self.0.set(new_state);
    }

    pub fn add_instruction(&self, tab_index: usize) {
        let mut new_state = (*self.0).clone();
        let current_tab = &mut new_state.tabs[tab_index];

        let index = current_tab.instructions.len();
        current_tab.instructions.push(AsmInstruction::new(index));
        
        self.0.set(new_state);
    }

    fn recalculate_addresses(&self, instructions: &mut AsmInstructions, start_rip: u64) {
        let mut addr = start_rip;
        for instr in instructions.iter_mut() {
            instr.address = addr;
            addr += instr.decoded.length as u64;
        }
    }

    pub fn remove_instruction(&self, instr_index: usize, tab_index: usize) {
        let mut new_state = (*self.0).clone();
        let tab = &mut new_state.tabs[tab_index];
        
        tab.instructions.remove(instr_index);
        
        for (idx, instr) in tab.instructions.iter_mut().enumerate() {
            instr.index = idx;
        }
        
        self.recalculate_addresses(&mut tab.instructions, tab.registers.rip);
        self.0.set(new_state);
    }
    
    pub fn update_instruction(&self, tab_index: usize, instr_index: usize, bytes: Vec<u8>) {
        let mut new_state = (*self.0).clone();
        let tab = &new_state.tabs[tab_index];
        
        let decoder = tab.decoder.clone();
        let rip = tab.registers.rip;
        
        let instructions = &mut new_state.tabs[tab_index].instructions;
        
        let address = if instr_index > 0 {
            instructions[instr_index - 1].address + instructions[instr_index - 1].decoded.length as u64
        } else {
            rip
        };
        
        instructions[instr_index] = AsmInstruction::from_bytes(instr_index, bytes, address, &*decoder);
        
        self.recalculate_addresses(instructions, rip);
        self.0.set(new_state);
    }
    
    // pub fn remove_instruction(&self, instr_index: usize, tab_index: usize) {
    //     let mut new_state = (*self.0).clone();
        
    //     let instructions = &mut new_state.tabs[tab_index].instructions;
    //     instructions.remove(instr_index);

    //     for (index, instruction) in instructions.iter_mut().enumerate() {
    //         instruction.index = index;
    //     }

    //     self.0.set(new_state);
    // }
    
    // pub fn update_instruction(&self, tab_index: usize, instr_index: usize, bytes: Vec<u8>) {
    //     let mut new_state = (*self.0).clone();
    //     let tab = &new_state.tabs[tab_index];
        
    //     let decoder = tab.decoder.clone();
    //     let rip = tab.registers.rip;

    //     let mut address = if instr_index > 0 {
    //         let instructions = &new_state.tabs[tab_index].instructions;
    //         let prev_instr_addr = instructions[instr_index - 1].address;
    //         prev_instr_addr + prev_instr_addr as u64
    //     } else {
    //         rip
    //     };
        
    //     let instructions = &mut new_state.tabs[tab_index].instructions;
    //     instructions[instr_index] = AsmInstruction::from_bytes(instr_index, bytes, address, &*decoder);
        
    //     for instruction in instructions.iter_mut() {
    //         instruction.address = address;
    //         if instruction.decoded.length > 0 {
    //             address += instruction.decoded.length as u64;
    //         }
    //     }

    //     self.0.set(new_state);
    // }

    pub fn update_tab_register(&self, tab_index: usize, name: String, value: u64) {
        let mut new_state = (*self.0).clone();
        new_state.tabs[tab_index].registers.update(&name, value);
        self.0.set(new_state);
    }

    pub fn registers(&self) -> Vec<(String, u64)> {
        self.active_tab().registers.iter()
    }
}