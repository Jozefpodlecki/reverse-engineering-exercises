use yew::prelude::*;
use yew_icons::{Icon, IconData};

use super::SectionSelection;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Hex,
    Disassembly,
}

#[derive(Properties, PartialEq)]
pub struct SectionTopbarProps {
    pub selection: Option<SectionSelection>,
    pub view_mode: ViewMode,
    pub on_view_mode_change: Callback<ViewMode>,
}

#[function_component(SectionTopbar)]
pub fn section_topbar(props: &SectionTopbarProps) -> Html {
    let on_export = {
        let selection = props.selection.clone();
        Callback::from(move |_| {
            if let Some(sel) = &selection {
                // TODO: trigger export download
                web_sys::console::log_1(&format!("Exporting section: {}", sel.name).into());
            }
        })
    };
    
    let on_hex = {
        let on_change = props.on_view_mode_change.clone();
        Callback::from(move |_| on_change.emit(ViewMode::Hex))
    };
    
    let on_disasm = {
        let on_change = props.on_view_mode_change.clone();
        Callback::from(move |_| on_change.emit(ViewMode::Disassembly))
    };

    html! {
        <div class="border-b border-zinc-800 px-4 py-2">
            if let Some(sel) = &props.selection {
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <h3 class="text-lg font-mono font-semibold text-blue-400">{&sel.name}</h3>
                        <div class="flex gap-2">
                            <span class="text-xs text-zinc-500">{format!("VA: 0x{:X}", sel.virtual_address)}</span>
                            <span class="text-xs text-zinc-500">{format!("VS: 0x{:X}", sel.virtual_size)}</span>
                            <span class="text-xs text-zinc-500">{format!("RAW: 0x{:X}", sel.raw_address)}</span>
                            <span class="text-xs text-zinc-500">{format!("RS: 0x{:X}", sel.raw_size)}</span>
                        </div>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="flex bg-zinc-800/50 rounded-lg p-0.5">
                            <button
                                onclick={on_hex}
                                class={format!("px-3 py-1 text-xs rounded-md transition {}", 
                                    if props.view_mode == ViewMode::Hex { "bg-blue-500 text-white" } else { "text-zinc-400 hover:text-zinc-300" }
                                )}
                            >
                                {"Hex"}
                            </button>
                            <button
                                onclick={on_disasm}
                                class={format!("px-3 py-1 text-xs rounded-md transition {}", 
                                    if props.view_mode == ViewMode::Disassembly { "bg-blue-500 text-white" } else { "text-zinc-400 hover:text-zinc-300" }
                                )}
                            >
                                {"Disassembly"}
                            </button>
                        </div>
                        <button
                            onclick={on_export}
                            class="p-1.5 text-zinc-400 hover:text-zinc-300 transition rounded hover:bg-zinc-800/50"
                            title="Export section"
                        >
                            <Icon data={IconData::LUCIDE_DOWNLOAD} width="1.25rem" height="1.25rem" />
                        </button>
                    </div>
                </div>
            } else {
                <div class="text-center text-zinc-500 py-1">
                    {"Select a section to view"}
                </div>
            }
        </div>
    }
}