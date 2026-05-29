use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct HexViewerFooterProps {
    pub current_page: usize,
    pub total_pages: usize,
    pub total_rows: usize,
    pub data_len: usize,
    pub on_page_change: Callback<usize>,
}

#[function_component(HexViewerFooter)]
pub fn hex_viewer_footer(props: &HexViewerFooterProps) -> Html {
    let on_first = {
        let on_page_change = props.on_page_change.clone();
        Callback::from(move |_| on_page_change.emit(0))
    };
    
    let on_last = {
        let on_page_change = props.on_page_change.clone();
        let total_pages = props.total_pages;
        Callback::from(move |_| on_page_change.emit(total_pages - 1))
    };
    
    let on_previous = {
        let on_page_change = props.on_page_change.clone();
        let current_page = props.current_page;
        Callback::from(move |_| {
            if current_page > 0 {
                on_page_change.emit(current_page - 1);
            }
        })
    };
    
    let on_next = {
        let on_page_change = props.on_page_change.clone();
        let current_page = props.current_page;
        let total_pages = props.total_pages;
        Callback::from(move |_| {
            if current_page < total_pages - 1 {
                on_page_change.emit(current_page + 1);
            }
        })
    };
    
    let on_page_input = {
        let on_page_change = props.on_page_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let page = input.value().parse::<usize>().unwrap_or(1).saturating_sub(1);
            on_page_change.emit(page);
        })
    };

    html! {
        <div class="flex justify-between items-center px-2 py-2 border-t border-zinc-800 text-xs text-zinc-500">
            <div>
                {"Total: "}{props.total_rows}{" rows, "}{props.data_len}{" bytes"}
            </div>
            <div class="flex gap-4">
                <div class="flex items-center gap-2">
                    <button 
                        onclick={on_first}
                        disabled={props.current_page == 0}
                        class="px-2 py-1 rounded hover:bg-zinc-800 disabled:opacity-50"
                    >
                        {"⏮"}
                    </button>
                    <button 
                        onclick={on_previous}
                        disabled={props.current_page == 0}
                        class="px-2 py-1 rounded hover:bg-zinc-800 disabled:opacity-50"
                    >
                        {"◀"}
                    </button>
                    <span class="text-zinc-400">{format!("Page {}", props.current_page + 1)}</span>
                    <button 
                        onclick={on_next}
                        disabled={props.current_page >= props.total_pages - 1}
                        class="px-2 py-1 rounded hover:bg-zinc-800 disabled:opacity-50"
                    >
                        {"▶"}
                    </button>
                    <button 
                        onclick={on_last}
                        disabled={props.current_page >= props.total_pages - 1}
                        class="px-2 py-1 rounded hover:bg-zinc-800 disabled:opacity-50"
                    >
                        {"⏭"}
                    </button>
                </div>
                <div class="flex items-center gap-2">
                    <span>{"Go to:"}</span>
                    <input 
                        type="number"
                        min="1"
                        max={props.total_pages.to_string()}
                        class="w-16 px-2 py-1 bg-zinc-900 border border-zinc-700 rounded text-xs"
                        onchange={on_page_input}
                    />
                </div>
            </div>
        </div>
    }
}