use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::asm::TabManager;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(ExecutionControls)]
pub fn execution_controls(props: &Props) -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let current_tab = manager.active_tab();
    let is_running = manager.is_running();

    let on_action: Callback<MouseEvent> = {
        let manager = manager.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target_unchecked_into::<HtmlElement>();
            let btn: HtmlElement = target.closest("button").unwrap().unwrap().unchecked_into();
            let action = btn.get_attribute("data-action").unwrap();
            
            match action.as_str() {
                "step-into" => manager.step_into(),
                "step-over" => manager.step_over(),
                "continue" => manager.continue_execution(),
                "run" => {
                    if manager.is_running() {
                        manager.pause();
                    } else {
                        manager.run();
                    }
                }
                _ => {}
            }
        })
    };

    html! {
        <div class="bg-zinc-900/30 border-t border-zinc-800 p-2 flex justify-center gap-2">
            <button 
                type="button"
                data-action="step-into"
                class="p-2 rounded-lg bg-zinc-800 hover:bg-zinc-700 transition disabled:opacity-50"
                onclick={&on_action}
                disabled={is_running}
                title="Step Into"
            >
                <Icon data={IconData::LUCIDE_CORNER_DOWN_LEFT} width="1.25rem" height="1.25rem" />
            </button>
            <button 
                type="button"
                data-action="step-over"
                class="p-2 rounded-lg bg-zinc-800 hover:bg-zinc-700 transition disabled:opacity-50"
                onclick={&on_action}
                disabled={is_running}
                title="Step Over"
            >
                <Icon data={IconData::LUCIDE_CORNER_DOWN_RIGHT} width="1.25rem" height="1.25rem" />
            </button>
            <button 
                type="button"
                data-action="continue"
                class="p-2 rounded-lg bg-zinc-800 hover:bg-zinc-700 transition disabled:opacity-50"
                onclick={&on_action}
                disabled={is_running}
                title="Continue"
            >
                <Icon data={IconData::LUCIDE_PLAY} width="1.25rem" height="1.25rem" />
            </button>
            <button 
                type="button"
                data-action="run"
                class="p-2 rounded-lg bg-emerald-600 hover:bg-emerald-700 transition"
                onclick={&on_action}
                title="Run"
            >
                <Icon data={IconData::LUCIDE_PLAY} width="1.25rem" height="1.25rem" />
                // <Icon data={IconData::LUCIDE_PAUSE} width="1.25rem" height="1.25rem" />
            </button>
        </div>
    }
}