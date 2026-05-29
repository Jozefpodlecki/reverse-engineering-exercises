use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct SectionSearchbarProps {
    pub on_search: Callback<SearchQuery>,
    pub placeholder: String,
}

#[derive(Clone, PartialEq)]
pub struct SearchQuery {
    pub section_name: String,
    pub offset: u64,
    pub is_relative: bool,
}

impl SearchQuery {
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }
        
        let parts: Vec<&str> = input.split('+').collect();
        if parts.len() == 2 {
            let section_name = parts[0].to_string();
            let offset_str = parts[1];
            let offset = if offset_str.ends_with('h') {
                u64::from_str_radix(&offset_str[..offset_str.len()-1], 16).ok()
            } else if offset_str.starts_with("0x") {
                u64::from_str_radix(&offset_str[2..], 16).ok()
            } else {
                offset_str.parse::<u64>().ok()
            };
            
            if let Some(offset) = offset {
                return Some(SearchQuery {
                    section_name,
                    offset,
                    is_relative: true,
                });
            }
        }
        
        if let Ok(offset) = if input.ends_with('h') {
            u64::from_str_radix(&input[..input.len()-1], 16)
        } else if input.starts_with("0x") {
            u64::from_str_radix(&input[2..], 16)
        } else {
            input.parse::<u64>()
        } {
            return Some(SearchQuery {
                section_name: String::new(),
                offset,
                is_relative: false,
            });
        }
        
        None
    }
    
    pub fn calculate_rva(&self, sections: &[crate::pages::pe_inspector::types::Section]) -> Option<u32> {
        if self.is_relative {
            let section = sections.iter().find(|s| s.name == self.section_name)?;
            Some(section.virtual_address_rva + self.offset as u32)
        } else {
            Some(self.offset as u32)
        }
    }
}

#[function_component(SectionSearchbar)]
pub fn section_searchbar(props: &SectionSearchbarProps) -> Html {
    let input_ref = use_node_ref();
    let error = use_state(|| None::<String>);
    
    let on_submit = {
        let on_search = props.on_search.clone();
        let error = error.clone();
        let input_ref = input_ref.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let query_str = input.value();
            
            if let Some(query) = SearchQuery::parse(&query_str) {
                error.set(None);
                on_search.emit(query);
            } else {
                error.set(Some("Invalid format. Use: .text+100h or 0x1000".to_string()));
            }
        })
    };
    
    let error_html = error.as_ref().map(|err| {
        html! {
            <div class="text-xs text-red-400 mt-1">
                {err}
            </div>
        }
    });

    html! {
        <div class="p-3 border-b border-zinc-800">
            <form onsubmit={on_submit}>
                <div class="relative">
                    <Icon 
                        data={IconData::LUCIDE_SEARCH} 
                        width="1rem" 
                        height="1rem" 
                        class="absolute left-3 top-1/2 transform -translate-y-1/2 text-zinc-500"
                    />
                    <input
                        ref={input_ref}
                        type="text"
                        placeholder={props.placeholder.clone()}
                        class="w-full pl-9 pr-3 py-2 bg-zinc-900/40 border border-zinc-700 rounded-lg text-sm text-zinc-300 placeholder-zinc-500 focus:outline-none focus:border-blue-500"
                    />
                </div>
                {error_html}
            </form>
            <div class="text-xs text-zinc-600 mt-2">
                {"Examples: .text+100h , .rdata+0x200 , 0x140001000"}
            </div>
        </div>
    }
}