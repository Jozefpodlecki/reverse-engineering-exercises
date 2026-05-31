use yew::prelude::*;
use crate::pages::sct::types::{OsFamily, WindowsVersion};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub show: bool,
    pub selected_family: OsFamily,
    pub selected_versions: std::collections::HashSet<WindowsVersion>,
    pub on_toggle: Callback<()>,
    pub on_family_change: Callback<OsFamily>,
    pub on_toggle_version: Callback<WindowsVersion>,
    pub on_select_all: Callback<()>,
    pub on_clear_all: Callback<()>,
}

#[function_component(FilterPanel)]
pub fn filter_panel(props: &Props) -> Html {
    let families: Vec<(String, OsFamily)> = vec![
        ("Windows".to_string(), OsFamily::Windows),
    ];

    let on_click = {
        let on_family_change = props.on_family_change.clone();
        let on_toggle_version = props.on_toggle_version.clone();
        let on_select_all = props.on_select_all.clone();
        let on_clear_all = props.on_clear_all.clone();
        let on_toggle = props.on_toggle.clone();
        
        Callback::from(move |event: MouseEvent| {
            let target = event.target_dyn_into::<web_sys::HtmlElement>();
            if let Some(element) = target {
                if let Some(action) = element.get_attribute("data-action") {
                    match action.as_str() {
                        "toggle-filters" => {
                            on_toggle.emit(());
                        }
                        "select-family" => {
                            if let Some(family) = element.get_attribute("data-family") {
                                match family.as_str() {
                                    "Windows" => on_family_change.emit(OsFamily::Windows),
                                    _ => {}
                                }
                            }
                        }
                        "toggle-version" => {
                            if let Some(version) = element.get_attribute("data-version") {
                                match version.as_str() {
                                    "XP" => on_toggle_version.emit(WindowsVersion::Xp),
                                    "Vista" => on_toggle_version.emit(WindowsVersion::Vista),
                                    "7" => on_toggle_version.emit(WindowsVersion::Win7),
                                    "8" => on_toggle_version.emit(WindowsVersion::Win8),
                                    "8.1" => on_toggle_version.emit(WindowsVersion::Win81),
                                    "10" => on_toggle_version.emit(WindowsVersion::Win10),
                                    "11" => on_toggle_version.emit(WindowsVersion::Win11),
                                    _ => {}
                                }
                            }
                        }
                        "select-all" => {
                            on_select_all.emit(());
                        }
                        "clear-all" => {
                            on_clear_all.emit(());
                        }
                        _ => {}
                    }
                }
            }
        })
    };

    html! {
        <div data-component="filter-panel" class="bg-zinc-900/30 border border-zinc-800 rounded-xl p-6">
            <div class="flex items-center justify-between mb-4">
                <h2 class="text-lg font-semibold text-zinc-200">{"System Call Table"}</h2>
                <button 
                    type="button"
                    data-action="toggle-filters"
                    class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded-lg transition text-zinc-300"
                >
                    { if props.show { "Hide Filters" } else { "Show Filters" } }
                </button>
            </div>
            
            if props.show {
                <div class="space-y-6">
                    <div>
                        <label class="text-sm text-zinc-400 mb-2 block">{"Operating System Family"}</label>
                        <div class="flex gap-2">
                            { families.iter().map(|(name, family)| {
                                let is_selected = props.selected_family == *family;
                                let classes = if is_selected {
                                    "bg-cyan-500/20 text-cyan-400 border border-cyan-500/50"
                                } else {
                                    "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 border border-zinc-700"
                                };
                                
                                html! {
                                    <button 
                                        type="button"
                                        data-action="select-family"
                                        data-family={name.to_string()}
                                        class={format!("px-4 py-2 rounded-lg text-sm font-medium transition {}", classes)}
                                    >
                                        { name }
                                    </button>
                                }
                            }).collect::<Html>() }
                        </div>
                    </div>
                    
                    <div>
                        <div class="flex items-center justify-between mb-2">
                            <label class="text-sm text-zinc-400">{"Windows Versions"}</label>
                            <div class="flex gap-2">
                                <button 
                                    type="button"
                                    data-action="select-all"
                                    class="text-xs px-2 py-1 bg-zinc-800 hover:bg-zinc-700 rounded text-zinc-400 transition"
                                >
                                    {"Select All"}
                                </button>
                                <button 
                                    type="button"
                                    data-action="clear-all"
                                    class="text-xs px-2 py-1 bg-zinc-800 hover:bg-zinc-700 rounded text-zinc-400 transition"
                                >
                                    {"Clear All"}
                                </button>
                            </div>
                        </div>
                        <div class="flex flex-wrap gap-2">
                            { WindowsVersion::all().iter().map(|version| {
                                let is_selected = props.selected_versions.contains(version);
                                let classes = if is_selected {
                                    "bg-cyan-500/20 text-cyan-400 border border-cyan-500/50"
                                } else {
                                    "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 border border-zinc-700"
                                };
                                
                                html! {
                                    <button 
                                        type="button"
                                        data-action="toggle-version"
                                        data-version={version.name()}
                                        class={format!("px-3 py-1.5 rounded-lg text-sm font-medium transition {}", classes)}
                                    >
                                        { version.name() }
                                    </button>
                                }
                            }).collect::<Html>() }
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}