use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::asm::*;

#[function_component(TabComponent)]
pub fn tab() -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let current_tab = manager.active_tab();
    let index = manager.active_index();
    let is_running = use_state(|| false);
    
    let instructions: Vec<Html> = current_tab.instructions.iter().enumerate().map(|(idx, instruction)| {
        html! {
            <AsmEditor 
                tab_index={index}
                instr_index={idx}
                instruction={instruction.clone()}
            />
        }
    }).collect();
    
    let on_add_instruction = {
        let manager = manager.clone();
        let index = index;
        Callback::from(move |_| {
            manager.add_instruction(index);
        })
    };
    
    html! {
        <div class="flex-1 flex flex-col h-full">
            <RegisterPanel />
            <div class="flex-1 flex justify-center items-center">
                <div class="overflow-y-auto p-4">
                    <div class="overflow-y-auto flex flex-col gap-2">
                        {instructions}
                        <button 
                            type="button"
                            class="w-100 flex items-center justify-center gap-2 px-4 py-2 mt-2 bg-zinc-800/50 rounded-lg hover:bg-zinc-700/50 transition text-sm text-zinc-400"
                            onclick={on_add_instruction}
                        >
                            <Icon data={IconData::LUCIDE_PLUS} width="1rem" height="1rem" />
                            {"New instruction"}
                        </button>
                    </div>
                </div>
            </div>
            <ExecutionControls />
        </div>
    }
}