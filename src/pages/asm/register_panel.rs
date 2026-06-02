use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::pages::asm::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(RegisterPanel)]
pub fn register_panel(props: &Props) -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let current_tab = manager.active_tab();
    let tab_index = manager.active_tab_index();
    
    let rip_hex = format!("0x{:X}", current_tab.registers.rip);
    
    let on_decoder_change = {
        let manager = manager.clone();
        let tab_index = tab_index;
        let tab = current_tab.clone();
        Callback::from(move |kind: DecoderKind| {
            let mut new_tab = tab.clone();
            new_tab.decoder_type = kind;
            manager.update_tab(tab_index, new_tab);
        })
    };
    
    let on_rip_input = {
        let manager = manager.clone();
        let tab_index = tab_index;
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
            let value = input.value();
            let clean = value.trim_start_matches("0x");
            if let Ok(val) = u64::from_str_radix(clean, 16) {
                manager.update_tab_register(tab_index, "RIP".to_string(), val);
            }
        })
    };
    
    let on_register_change = {
        let manager = manager.clone();
        let tab_index = tab_index;
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
            let value = input.value();
            let name = input.get_attribute("data-register").unwrap();
            let clean = value.trim_start_matches("0x");
            if let Ok(val) = u64::from_str_radix(clean, 16) {
                manager.update_tab_register(tab_index, name, val);
            }
        })
    };
    
    let register_inputs: Vec<Html> = manager.registers().into_iter().map(|(name, value)| {
        let reg_value = format!("0x{:X}", value);
        
        html! {
            <div class="flex items-center gap-2" key={name.clone()}>
                <label class="text-sm text-zinc-400 w-12">{name.clone()}</label>
                <input 
                    type="text"
                    data-register={name}
                    class="bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-sm font-mono text-zinc-200 w-32"
                    value={reg_value}
                    onchange={on_register_change.clone()}
                />
            </div>
        }
    }).collect();
    
    html! {
        <div class="bg-zinc-900/30 border-b border-zinc-800 p-4">
            <div class="flex flex-wrap gap-6 items-center">
                <div class="flex items-center gap-2">
                    <label class="text-sm text-zinc-400">{"Decoder:"}</label>
                    <DecoderSelector 
                        value={current_tab.decoder_type.clone()} 
                        on_change={on_decoder_change}
                    />
                </div>
                
                <div class="h-6 w-px bg-zinc-700"></div>
                
                <div class="flex items-center gap-2">
                    <label class="text-sm text-zinc-400">{"RIP:"}</label>
                    <input 
                        type="text"
                        data-register="RIP"
                        class="bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-sm font-mono text-zinc-200 w-36"
                        value={rip_hex}
                        onchange={on_rip_input}
                    />
                </div>
                
                <div class="h-6 w-px bg-zinc-700"></div>
                
                <div class="flex flex-wrap gap-4">
                    {register_inputs}
                </div>
            </div>
        </div>
    }
}