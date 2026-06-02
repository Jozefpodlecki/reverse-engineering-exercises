use iced_x86::Instruction;
use rand::{RngExt, rng};
use yew::UseStateHandle;

use crate::pages::asm::{AsmInstruction, DecoderFactory, types::{DecoderKind, Tab}};

#[derive(Clone, PartialEq)]
pub struct TabManager {
    state: UseStateHandle<TabManagerState>,
}

#[derive(Default, Clone, PartialEq)]
pub struct TabManagerState {
    tabs: Vec<Tab>,
    active_index: usize,
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
                    rip: 0x1000,
                    instructions: vec![
                        AsmInstruction::default()
                    ]
                }
            ],
            active_index: 0,
        }
    }
}

impl TabManager {
    pub fn new(initial_state: UseStateHandle<TabManagerState>) -> Self {
        Self { state: initial_state }
    }
    
    pub fn tabs(&self) -> Vec<Tab> {
        self.state.tabs.clone()
    }
    
    pub fn active_index(&self) -> usize {
        self.state.active_index
    }
    
    pub fn active_tab(&self) -> Option<Tab> {
        self.state.tabs.get(self.state.active_index).cloned()
    }
    
    pub fn add_tab(&self) {
        let mut new_state = (*self.state).clone();
        let new_id = (new_state.tabs.len() + 1).to_string();
        let tab= Tab {
            id: new_id,
            name: random_string(10),
            decoder_type: DecoderKind::IcedX86,
            rip: 0x1000,
            instructions: vec![
                AsmInstruction::default()
            ]
        };
        new_state.tabs.push(tab);
        new_state.active_index = new_state.tabs.len() - 1;
        self.state.set(new_state);
    }
    
    pub fn close_tab(&self, id: String) {
        let mut new_state = (*self.state).clone();
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
        self.state.set(new_state);
    }
    
    pub fn select_tab(&self, index: usize) {
        if index < self.state.tabs.len() {
            let mut new_state = (*self.state).clone();
            new_state.active_index = index;
            self.state.set(new_state);
        }
    }
    
    pub fn update_tab(&self, index: usize, tab: Tab) {
        let mut new_state = (*self.state).clone();
        new_state.tabs[index] = tab;
        self.state.set(new_state);
    }

    pub fn add_instruction(&self, tab_index: usize) {
        let mut new_state = (*self.state).clone();
        if tab_index < new_state.tabs.len() {
            new_state.tabs[tab_index].instructions.push(AsmInstruction::default());
            self.state.set(new_state);
        }
    }
    
    pub fn remove_instruction(&self, tab_index: usize, instr_index: usize) {
        let mut new_state = (*self.state).clone();
        if tab_index < new_state.tabs.len() {
            let instructions = &mut new_state.tabs[tab_index].instructions;
            if instr_index < instructions.len() {
                instructions.remove(instr_index);
                self.state.set(new_state);
            }
        }
    }
    
    pub fn update_instruction(&self, tab_index: usize, instr_index: usize, bytes: Vec<u8>) {
        let mut new_state = (*self.state).clone();
        
        let tab = &new_state.tabs[tab_index];
        log::info!("{:?}", tab.decoder_type);
        let decoder = DecoderFactory::create(&tab.decoder_type, 64);

        let tab = &new_state.tabs[tab_index];
        let decoder = DecoderFactory::create(&tab.decoder_type, 64);
        
        let decoded = decoder.decode(&bytes, tab.rip).ok();
        
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

        self.state.set(new_state);
    }
}