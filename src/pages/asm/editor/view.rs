use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::asm::*;
use crate::pages::asm::editor::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab_index: usize,
    pub instr_index: usize,
    pub instruction: AsmInstruction,
}

#[function_component(AsmEditor)]
pub fn asm_editor(props: &Props) -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let current_tab = manager.active_tab();
    let is_current = current_tab.registers.rip == props.instruction.address;
    let is_running = manager.is_running();

    let row_class = match (is_running, is_current) {
        (true, true) => "flex gap-2 items-center mb-2 p-2 bg-amber-900/30 border-l-4 border-amber-500 rounded-r-lg",
        (true, false) => "flex gap-2 items-center mb-2 p-2",
        (false, true) => "flex gap-2 items-center mb-2 p-2",
        (false, false) => "flex gap-2 items-center mb-2 p-2",
    };

    let on_delete = {
        let instr_index = props.instr_index;
        let tab_index = props.tab_index;
        let manager = manager.clone();
        Callback::from(move |_| {
            manager.remove_instruction(instr_index, tab_index);
        })
    };
    
    html! {
        <div data-instruction=true class={row_class}>
            <HexByteEditor 
                tab_index={props.tab_index}
                instr_index={props.instr_index}
                instruction={props.instruction.clone()}
            />
            <input 
                type="text"
                class="flex-1 max-w-64 bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-sm font-mono text-zinc-200"
                value={props.instruction.asm.clone()}
                readonly={true}
            />
            <button 
                type="button"
                class="p-1 rounded hover:bg-zinc-700"
                data-action="delete-instruction"
                onclick={on_delete}
                data-tab-index={props.tab_index.to_string()}
                data-instr-index={props.instr_index.to_string()}
            >
                <Icon data={IconData::LUCIDE_X} width="1.25rem" height="1.25rem" />
            </button>
        </div>
    }
}