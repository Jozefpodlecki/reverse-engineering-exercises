use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::asm::*;
use crate::pages::asm::editor::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab_index: usize,
    pub instr_index: usize
}

#[function_component(HexByteEditor)]
pub fn hex_byte_editor(props: &Props) -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let current_tab = manager.active_tab();
    let instruction = current_tab.instructions.get(props.instr_index).unwrap();
   
    let on_byte_change = {
        let instruction = instruction.clone();
        let tab_index = props.tab_index;
        let instr_index = props.instr_index;
        let manager = manager.clone();
        
        Callback::from(move |(idx, value): (usize, String)| {
            let mut new_bytes = instruction.bytes_input();
            
            let hex_byte = value.to_ascii_uppercase();
            if hex_byte.len() == 2 {
                if let Ok(b) = u8::from_str_radix(&hex_byte, 16) {
                    new_bytes[idx] = Some(b);
                } else {
                    new_bytes[idx] = None;
                }
            } else if hex_byte.is_empty() {
                new_bytes[idx] = None;
            } else {
                return;
            }
            
            
            let collected: Vec<u8> = new_bytes.iter().filter_map(|&b| b).collect();
            manager.update_instruction(tab_index, instr_index, collected);
        })
    };
    
    let on_clear = {
        let tab_index = props.tab_index;
        let instr_index = props.instr_index;
        let manager = manager.clone();

        Callback::from(move |_| {
            manager.update_instruction(tab_index, instr_index, Vec::new());
        })
    };

    let on_random = {
        let tab_index = props.tab_index;
        let instr_index = props.instr_index;
        let manager = manager.clone();

        Callback::from(move |_| {
            let new_bytes = random_instruction();
            let collected: Vec<u8> = new_bytes.iter().filter_map(|&b| b).collect();
            manager.update_instruction(tab_index, instr_index, collected);
        })
    };
    
    let byte_opt = instruction.bytes_input();
    let byte_inputs: Vec<Html> = (0..15).map(|idx| {
        html! {
            <ByteInput 
                key={idx}
                index={idx}
                value={byte_opt[idx]}
                on_change={on_byte_change.clone()}
            />
        }
    }).collect();
    
    let has_bytes = instruction.bytes.len() > 0;
    
    let clear_button_class = if has_bytes {
        "p-1 rounded hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 transition cursor-pointer"
    } else {
        "p-1 rounded text-zinc-700 cursor-not-allowed"
    };
    
    html! {
        <div class="flex gap-2 items-center">
            <div class="text-zinc-500 font-mono text-sm w-16 mr-2">
                {format!("0x{:08X}", instruction.address)}
            </div>
            <button type="button" onclick={on_random} class="p-2">
                <Icon data={IconData::LUCIDE_DICES} width="1rem" height="1rem" />
            </button>
            <div class="flex gap-1 font-mono">
                {byte_inputs}
            </div>
            <button 
                type="button"
                class={clear_button_class}
                disabled={!has_bytes}
                onclick={on_clear}
                title={if has_bytes { "Clear all bytes" } else { "No bytes to clear" }}
            >
                <Icon data={IconData::LUCIDE_X} width="0.875rem" height="0.875rem" />
            </button>
        </div>
    }
}