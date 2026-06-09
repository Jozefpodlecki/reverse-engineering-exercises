use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HexViewerHeaderProps {
    pub current_page: usize,
    pub total_pages: usize,
    pub start_row: usize,
    pub end_row: usize,
    pub total_rows: usize,
    pub on_page_change: Callback<usize>,
}

#[function_component(HexViewerHeader)]
pub fn hex_viewer_header(props: &HexViewerHeaderProps) -> Html {
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

    html! {
        <div class="flex justify-between items-center px-2 py-2 border-b border-zinc-800 text-xs text-zinc-500">
            <div>
                {format!("Rows {}-{} of {}", props.start_row, props.end_row.min(props.total_rows) - 1, props.total_rows)}
            </div>
            <div class="flex gap-2">
                <button 
                    onclick={on_previous}
                    disabled={props.current_page == 0}
                    class="px-2 py-1 rounded hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"Previous"}
                </button>
                <span>{format!("Page {} of {}", props.current_page + 1, props.total_pages)}</span>
                <button 
                    onclick={on_next}
                    disabled={props.current_page >= props.total_pages - 1}
                    class="px-2 py-1 rounded hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"Next"}
                </button>
            </div>
        </div>
    }
}