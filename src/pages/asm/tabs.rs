use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::asm::*;

#[function_component(Tabs)]
pub fn tabs() -> Html {
    let manager = use_context::<TabManager>().unwrap();
    
    let on_click = {
        let manager = manager.clone();
        Callback::from(move |event: MouseEvent| {
            let mut target = event.target_unchecked_into::<HtmlElement>();
            
            if let Some(parent) = target.closest("[data-action]").unwrap() {
                target = parent.unchecked_into::<HtmlElement>();
            }

            let action = target.get_attribute("data-action").unwrap_or_else(|| "none".to_string());
            let tab_id = target.get_attribute("data-tab-id");
            
            log::info!("{}", target.tag_name());

            match action.as_str() {
                "close" => {
                    let id = tab_id.unwrap();

                    if manager.tabs().len() < 2 {
                        return;
                    }

                    manager.close_tab(id);
                }
                "select" => {
                    let index = target.get_attribute("data-tab-index").unwrap();
                    manager.select_tab(index.parse().unwrap());
                }
                "add" => {
                    manager.add_tab();
                }
                _ => {

                }
            }
        })
    };
    
    let tabs = manager.tabs();
    let active_index = manager.active_index();
    
    let tab_elements: Vec<Html> = tabs.iter().enumerate().map(|(idx, tab)| {
        let is_active = idx == active_index;
        
        let class_name = if is_active {
            "flex items-center gap-2 px-4 py-2 cursor-pointer bg-zinc-800 text-cyan-400 border-t border-x border-zinc-700"
        } else {
            "flex items-center gap-2 px-4 py-2 cursor-pointer text-zinc-400 hover:text-zinc-200"
        };
        
        html! {
            <div 
                class={class_name}
                data-action="select"
                data-tab-index={idx.to_string()}
            >
                <span>{&tab.name}</span>
                <button 
                    type="button"
                    class="p-0.5 rounded hover:bg-zinc-700"
                    data-action="close"
                    data-tab-id={tab.id.clone()}
                >
                    <Icon data={IconData::LUCIDE_X} width="1.25rem" height="1.25rem" />
                </button>
            </div>
        }
    }).collect();
    
    html! {
        <div class="flex border-b border-zinc-800 px-4 gap-1" onclick={on_click}>
            {tab_elements}
            <button 
                type="button"
                class="px-3 py-2 text-zinc-400 hover:text-zinc-200"
                data-action="add"
            >
                <Icon data={IconData::LUCIDE_PLUS} width="1rem" height="1rem" />
            </button>
        </div>
    }
}