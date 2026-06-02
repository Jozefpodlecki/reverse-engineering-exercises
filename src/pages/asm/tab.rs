use yew::prelude::*;
use yew_icons::{Icon, IconData};
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement};

use crate::pages::asm::editor::AsmEditor;
use crate::pages::asm::selectors::DecoderSelector;
use crate::pages::asm::tab_manager::TabManager;
use crate::pages::asm::types::DecoderKind;

#[function_component(TabComponent)]
pub fn tab() -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let current_tab = manager.active_tab().unwrap();
    let index = manager.active_index();
    
    let on_decoder_change = {
        let manager = manager.clone();
        let index = index;
        let tab = current_tab.clone();
        Callback::from(move |kind: DecoderKind| {
            let mut new_tab = tab.clone();
            new_tab.decoder_type = kind;
            manager.update_tab(index, new_tab);
        })
    };
    
    let on_rip_change = {
        let manager = manager.clone();
        let index = index;
        let tab = current_tab.clone();
        Callback::from(move |event: Event| {
            let mut tab = tab.clone();
            let input = event.target_unchecked_into::<HtmlInputElement>();
            if let Ok(value) = u64::from_str_radix(&input.value().trim_start_matches("0x"), 16) {
                tab.rip = value;
                manager.update_tab(index, tab.clone());
            }
        })
    };
    
    let rip_hex = format!("0x{:X}", current_tab.rip);
    
    let instructions: Vec<Html> = current_tab.instructions.iter().enumerate().map(|(idx, instruction)| {
        html! {
             <AsmEditor 
                tab_index={index}
                instr_index={idx}
                instruction={instruction.clone()}
            />
        }
    }).collect();

    log::info!("{:?}",  current_tab.decoder_type);

    html! {
        <div class="flex flex-col gap-4">
            <div class="">
                {"Decoder"}
                <DecoderSelector 
                    value={current_tab.decoder_type.clone()} 
                    on_change={on_decoder_change}
                />
                {"RIP"}
                <input 
                    type="text"
                    class="bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-sm font-mono text-zinc-200"
                    value={rip_hex}
                    onchange={on_rip_change}
                />
            </div>
            <div class="flex flex-col">
                {instructions}
                <button type="button">
                    <Icon data={IconData::LUCIDE_PLUS} width="1.25rem" height="1.25rem" />
                </button>
            </div>
        </div>
    }
}