use std::{rc::Rc, time::Duration};
use gloo::timers::future::TimeoutFuture;
use yew::UseStateHandle;
use crate::pages::asm::*;

#[derive(Default, Clone, PartialEq)]
pub enum ExecutionState {
    #[default]
    Idle,
    Running,
    Paused,
    Finished,
}

#[derive(Clone, PartialEq)]
pub struct Emulator(UseStateHandle<EmulatorState>);

#[derive(Clone)]
pub struct EmulatorState {
    pub delay: Duration,
    pub execution_state: ExecutionState,
    pub registers: Registers,
    pub memory: Memory,
    pub instructions: AsmInstructions,
    pub decoder: Rc<dyn Decoder>,
}

impl PartialEq for EmulatorState {
    fn eq(&self, other: &Self) -> bool {
        self.execution_state == other.execution_state &&
        self.registers == other.registers &&
        self.decoder.name() == other.decoder.name() &&
        self.instructions == other.instructions
    }
}

impl Default for EmulatorState {
    fn default() -> Self {
        Self {
            delay: Duration::from_secs(1),
            execution_state: Default::default(),
            registers: Default::default(),
            instructions: Default::default(),
            memory: Default::default(),
            decoder: DecoderFactory::create("iced-x86", 64)
        }
    }
}

impl EmulatorState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Emulator {
    pub fn new(initial: UseStateHandle<EmulatorState>) -> Self {
        Self(initial)
    }
    
    pub fn is_running(&self) -> bool {
        matches!(self.0.execution_state, ExecutionState::Running)
    }
    
    pub fn registers(&self) -> Registers {
        self.0.registers.clone()
    }
    
    pub fn step_into(&self) -> bool {
        let mut new_state = (*self.0).clone();
        let rip = new_state.registers.rip;
        
        if let Some(instr) = new_state.instructions.get_by_rip(rip).cloned() {
            execute_instruction(&instr.decoded, &mut new_state);
            self.0.set(new_state);
            return true;
        }
        
        new_state.execution_state = ExecutionState::Finished;
        self.0.set(new_state);
        false
    }
    
    pub fn step_over(&self) -> bool {
        let mut new_state = (*self.0).clone();
        let rip = new_state.registers.rip;
        
        if let Some(instr) = new_state.instructions.get_by_rip(rip).cloned() {
            let is_call_or_jmp = instr.decoded.is_call_or_jmp();
            
            if !is_call_or_jmp {
                execute_instruction(&instr.decoded, &mut new_state);
            }

            self.0.set(new_state);
            return true;
        }
        
        new_state.execution_state = ExecutionState::Finished;
        self.0.set(new_state);
        false
    }
    
    pub fn run(&self, registers: Registers, instructions: AsmInstructions, decoder: Rc<dyn Decoder>) {
        let mut new_state = (*self.0).clone();
        new_state.execution_state = ExecutionState::Running;
        new_state.instructions = instructions;
        new_state.registers = registers;
        new_state.decoder = decoder;
        self.0.set(new_state);
        
        self.spawn();
    }
    
    pub fn pause(&self) {
        let mut new_state = (*self.0).clone();
        new_state.execution_state = ExecutionState::Paused;
        self.0.set(new_state);
    }
    
    pub fn reset(&self) {
        let mut new_state = (*self.0).clone();
        new_state.execution_state = ExecutionState::Idle;
        if let Some(first) = new_state.instructions.get(0) {
            new_state.registers.rip = first.address;
        }
        self.0.set(new_state);
    }
    
    pub fn continue_execution(&self) {
        self.spawn();
    }

    fn spawn(&self) {
        let emulator = self.clone();
        let delay = self.0.delay.as_millis() as u32;

        wasm_bindgen_futures::spawn_local(async move {
            while emulator.is_running() {
                TimeoutFuture::new(delay).await;
                emulator.step_into();
            }
        });
    }
}