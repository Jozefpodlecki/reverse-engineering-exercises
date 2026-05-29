use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct LogModalProps {
    pub is_open: bool,
    pub on_close: Callback<()>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[function_component(LogModal)]
pub fn log_modal(props: &LogModalProps) -> Html {
    if !props.is_open {
        return html! {};
    }

    let has_logs = !props.warnings.is_empty() || !props.errors.is_empty();
    
    if !has_logs {
        return html! {};
    }

    let on_close = props.on_close.clone();
    let on_backdrop_click = {
        let on_close = on_close.clone();
        Callback::from(move |e: MouseEvent| {
            if e.target() == e.current_target() {
                on_close.emit(());
            }
        })
    };

    let on_close_button = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let has_warnings = !props.warnings.is_empty();
    let has_errors = !props.errors.is_empty();

    html! {
        <div 
            class="fixed inset-0 bg-black/70 flex items-center justify-center z-50"
            onclick={on_backdrop_click}
        >
            <div class="bg-zinc-900 rounded-xl border border-zinc-700 w-full max-w-2xl max-h-[80vh] flex flex-col shadow-xl">
                <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-800">
                    <div class="flex items-center gap-2">
                        <span class="text-xl">{"📋"}</span>
                        <h2 class="text-lg font-semibold text-zinc-100">{"Logs"}</h2>
                    </div>
                    <button
                        type="button"
                        onclick={&on_close_button}
                        class="text-zinc-500 hover:text-zinc-300 transition text-xl leading-none px-2"
                    >
                        <Icon data={IconData::LUCIDE_X}/>
                    </button>
                </div>

                <div class="flex-1 overflow-y-auto p-6 space-y-4">
                    if has_errors {
                        <div class="space-y-3">
                            <div class="flex items-center gap-2">
                                <span class="text-red-500">{"❌"}</span>
                                <h3 class="text-sm font-medium text-red-400">{"Errors"}</h3>
                                <span class="text-xs text-zinc-500">{format!("({})", props.errors.len())}</span>
                            </div>
                            <div class="space-y-2">
                                {for props.errors.iter().map(|err| {
                                    html! {
                                        <div class="p-3 bg-red-500/10 border border-red-500/30 rounded-lg">
                                            <p class="text-red-400 text-sm font-mono break-all">{err}</p>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }

                    if has_warnings {
                        <div class="space-y-3">
                            <div class="flex items-center gap-2">
                                <span class="text-yellow-500">{"⚠️"}</span>
                                <h3 class="text-sm font-medium text-yellow-400">{"Warnings"}</h3>
                                <span class="text-xs text-zinc-500">{format!("({})", props.warnings.len())}</span>
                            </div>
                            <div class="space-y-2">
                                {for props.warnings.iter().map(|warn| {
                                    html! {
                                        <div class="p-3 bg-yellow-500/10 border border-yellow-500/30 rounded-lg">
                                            <p class="text-yellow-400 text-sm font-mono break-all">{warn}</p>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }
                </div>

                <div class="px-6 py-4 border-t border-zinc-800 flex justify-end">
                    <button
                        onclick={&on_close_button}
                        class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded-lg text-sm transition"
                    >
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}