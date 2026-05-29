use yew::prelude::*;
use crate::pages::pe_inspector::{RcBytes, sections::topbar::ViewMode, types::Section};

mod sidebar;
mod topbar;
mod searchbar;
mod view;
mod hex_viewer;
mod disassembly_viewer;

use sidebar::SectionSidebar;
use topbar::SectionTopbar;
use searchbar::SectionSearchbar;
use view::SectionView;

#[derive(Properties, PartialEq)]
pub struct SectionsTabProps {
    pub sections: Vec<Section>,
    pub raw_bytes: RcBytes,
}

#[derive(Clone, PartialEq)]
pub struct SectionSelection {
    pub index: usize,
    pub name: String,
    pub virtual_address: u64,
    pub virtual_size: u32,
    pub raw_address: u32,
    pub raw_size: u32,
    pub characteristics: u32,
}

#[function_component(SectionsTab)]
pub fn sections_tab(props: &SectionsTabProps) -> Html {
    let selected_index = use_state(|| 0);
    let search_query = use_state(String::new);
    let view_mode = use_state(|| ViewMode::Hex);
    
    let sections = props.sections.clone();
    
    if sections.is_empty() {
        return html! {
            <div class="p-8 text-center text-zinc-500">
                {"No sections found"}
            </div>
        };
    }
    
    let selected = sections.get(*selected_index).cloned().unwrap();
    let selection = SectionSelection {
        index: *selected_index,
        name: selected.name.clone(),
        virtual_address: selected.virtual_address_va,
        virtual_size: selected.virtual_size,
        raw_address: selected.pointer_to_raw_data,
        raw_size: selected.size_of_raw_data,
        characteristics: selected.characteristics,
    };
    
    let on_section_select = {
        let selected_index = selected_index.clone();
        Callback::from(move |index: usize| {
            selected_index.set(index);
        })
    };
    
    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |query: String| {
            search_query.set(query);
        })
    };
    
    let on_view_mode_change = {
        let view_mode = view_mode.clone();
        Callback::from(move |mode: ViewMode| {
            view_mode.set(mode);
        })
    };
    
    let filtered_sections = if search_query.is_empty() {
        sections.clone()
    } else {
        sections.iter()
            .filter(|s| s.name.to_lowercase().contains(&search_query.to_lowercase()))
            .cloned()
            .collect()
    };

    html! {
        <div class="flex h-[calc(100vh-16rem)]">
            <SectionSidebar 
                sections={filtered_sections}
                selected_index={*selected_index}
                on_select={on_section_select}
                search_query={(*search_query).clone()}
                on_search={on_search}
            />
            <div class="flex-1 flex flex-col">
                <SectionTopbar 
                    selection={selection.clone()}
                    view_mode={*view_mode}
                    on_view_mode_change={on_view_mode_change}
                />
                <SectionView 
                    selection={selection.clone()}
                    view_mode={*view_mode}
                    raw_bytes={props.raw_bytes.clone()}
                />
            </div>
        </div>
    }
}