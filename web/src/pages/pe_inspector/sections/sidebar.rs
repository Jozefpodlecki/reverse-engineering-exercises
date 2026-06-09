use yew::prelude::*;
use yew_icons::{Icon, IconData};
use crate::pages::pe_inspector::types::Section;

#[derive(Properties, PartialEq)]
pub struct SectionSidebarProps {
    pub sections: Vec<Section>,
    pub selected_index: usize,
    pub on_select: Callback<usize>,
    pub search_query: String,
    pub on_search: Callback<String>,
}

#[function_component(SectionSidebar)]
pub fn section_sidebar(props: &SectionSidebarProps) -> Html {
    let on_input = {
        let on_search = props.on_search.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            on_search.emit(input.value());
        })
    };
    
    let section_flags = |chars: u32| -> &'static str {
        if chars & 0x20000000 != 0 { "code" }
        else if chars & 0x80000000 != 0 { "data" }
        else { "rodata" }
    };

    html! {
        <div class="w-80 border-r border-zinc-800 flex flex-col">
            <div class="p-3 border-b border-zinc-800">
                <div class="relative">
                    <Icon 
                        data={IconData::LUCIDE_SEARCH} 
                        width="1rem" 
                        height="1rem" 
                        class="absolute left-3 top-1/2 transform -translate-y-1/2 text-zinc-500"
                    />
                    <input
                        type="text"
                        placeholder="Filter sections..."
                        class="w-full pl-9 pr-3 py-2 bg-zinc-900/40 border border-zinc-700 rounded-lg text-sm text-zinc-300 placeholder-zinc-500 focus:outline-none focus:border-blue-500"
                        value={props.search_query.clone()}
                        oninput={on_input}
                    />
                </div>
            </div>
            <div class="flex-1 overflow-y-auto">
                {for props.sections.iter().enumerate().map(|(idx, section)| {
                    let is_selected = idx == props.selected_index;
                    let flag_class = section_flags(section.characteristics);
                    let flag_icon = match flag_class {
                        "code" => IconData::LUCIDE_ZAP,
                        "data" => IconData::LUCIDE_ARCHIVE,
                        _ => IconData::LUCIDE_FILE_TEXT,
                    };
                    let flag_color = match flag_class {
                        "code" => "text-yellow-500",
                        "data" => "text-blue-500",
                        _ => "text-green-500",
                    };
                    
                    let selected_classes = if is_selected {
                        "bg-blue-500/20 border-blue-500/50"
                    } else {
                        "hover:bg-zinc-800/50 border-transparent"
                    };
                    
                    let on_click = {
                        let on_select = props.on_select.clone();
                        Callback::from(move |_| on_select.emit(idx))
                    };
                    
                    html! {
                        <button
                            onclick={on_click}
                            class={format!("w-full text-left px-3 py-2 rounded-lg mb-1 border transition-all {}", selected_classes)}
                        >
                            <div class="flex items-center gap-2">
                                <Icon data={flag_icon} width="1rem" height="1rem" class={flag_color} />
                                <span class="font-mono text-sm text-zinc-300">{&section.name}</span>
                            </div>
                            <div class="flex items-center gap-2 mt-1">
                                <span class="text-xs text-zinc-500">{format!("0x{:X}", section.virtual_address_rva)}</span>
                                <span class="text-xs text-zinc-600">{"•"}</span>
                                <span class="text-xs text-zinc-500">{format!("0x{:X} bytes", section.virtual_size)}</span>
                            </div>
                        </button>
                    }
                })}
            </div>
        </div>
    }
}