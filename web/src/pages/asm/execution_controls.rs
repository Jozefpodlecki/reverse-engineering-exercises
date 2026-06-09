use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::asm::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(ExecutionControls)]
pub fn execution_controls(props: &Props) -> Html {
    let manager = use_context::<TabManager>().unwrap();
    let emulator = use_context::<Emulator>().unwrap();
    let current_tab = manager.active_tab();
    let is_running = emulator.is_running();
    let can_run = current_tab.can_run();

    let on_action: Callback<MouseEvent> = {
        let manager = manager.clone();
        let emulator = emulator.clone();
        let current_tab = current_tab.clone();

        Callback::from(move |event: MouseEvent| {
            let current_tab = current_tab.clone();

            let target = event.target_unchecked_into::<HtmlElement>();
            let btn: HtmlElement = target.closest("button").unwrap().unwrap().unchecked_into();
            let action = btn.get_attribute("data-action").unwrap();
            
            match action.as_str() {
                "step-into" => { emulator.step_into(); },
                "step-over" => { emulator.step_over(); },
                "continue" => { emulator.continue_execution(); },
                "run" => {
                    if emulator.is_running() {
                        emulator.pause();
                    } else {
                        
                        emulator.run(current_tab.registers, current_tab.instructions, current_tab.decoder);
                    }
                }
                _ => {}
            }
        })
    };

    let run_icon = if is_running {
        IconData::LUCIDE_PAUSE
    } else {
        IconData::LUCIDE_PLAY
    };

    let run_title = if is_running { "Pause" } else { "Run" };

    html! {
        <footer
            data-execution-controls=true
            class="bg-zinc-900/30 border-t border-zinc-800 p-2 flex justify-center gap-2">
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
                class="p-2 rounded-lg bg-emerald-600 hover:bg-emerald-700 transition disabled:bg-zinc-700 disabled:opacity-50 disabled:cursor-not-allowed"
                onclick={&on_action}
                title={run_title}
                disabled={!can_run}
            >
                <Icon data={run_icon} width="1.25rem" height="1.25rem" />
            </button>
        </footer>
    }
}