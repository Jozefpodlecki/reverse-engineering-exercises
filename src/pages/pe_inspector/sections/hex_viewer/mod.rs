mod header;
mod footer;
mod row;

pub use header::HexViewerHeader;
pub use footer::HexViewerFooter;
pub use row::HexRow;

use yew::prelude::*;
use web_sys::HtmlElement;

#[derive(Properties, PartialEq)]
pub struct HexViewerProps {
    pub data: Vec<u8>,
    pub virtual_address: u64,
    pub rows_per_page: usize,
}

#[function_component(HexViewer)]
pub fn hex_viewer(props: &HexViewerProps) -> Html {
    let current_page = use_state(|| 0);
    let container_ref = use_node_ref();
    
    let total_rows = (props.data.len() + 15) / 16;
    let total_pages = (total_rows + props.rows_per_page - 1) / props.rows_per_page;
    
    let start_row = *current_page * props.rows_per_page;
    let end_row = (start_row + props.rows_per_page).min(total_rows);
    
    let visible_rows: Vec<Vec<u8>> = (start_row..end_row)
        .map(|row_idx| {
            let start = row_idx * 16;
            let end = (start + 16).min(props.data.len());
            props.data[start..end].to_vec()
        })
        .collect();
    
    let on_scroll = {
        let container_ref = container_ref.clone();
        let current_page = current_page.clone();
        let total_pages = total_pages;
        let rows_per_page = props.rows_per_page;
        let row_height = 28; // Approximate height of each row in pixels
        
        Callback::from(move |_e: Event| {
            let container = container_ref.cast::<HtmlElement>().unwrap();
            let scroll_top = container.scroll_top();
            let scroll_height = container.scroll_height();
            let client_height = container.client_height();
            
            let current_page_val = *current_page;
            let start_row = current_page_val * rows_per_page;
            let expected_scroll_top = start_row * row_height;
            
            log::info!("Scroll: top={}, height={}, client={}, current_page={}, expected_top={}", 
                scroll_top, scroll_height, client_height, current_page_val, expected_scroll_top);
            
            if scroll_top + client_height >= scroll_height - 200 {
                if current_page_val < total_pages - 1 {
                    log::info!("Loading next page: {}", current_page_val + 1);
                    current_page.set(current_page_val + 1);
                }
            } else if scroll_top <= 200 && current_page_val > 0 {
                log::info!("Loading previous page: {}", current_page_val - 1);
                current_page.set(current_page_val - 1);
            }
        })
    };
    
    let on_page_change = {
        let current_page = current_page.clone();
        let container_ref = container_ref.clone();
        Callback::from(move |new_page: usize| {
            let new_page = new_page.min(total_pages - 1);
            current_page.set(new_page);
            if let Some(container) = container_ref.cast::<HtmlElement>() {
                container.set_scroll_top(0);
            }
        })
    };

    html! {
        <div class="flex flex-col h-full">
            <HexViewerHeader 
                current_page={*current_page}
                total_pages={total_pages}
                start_row={start_row}
                end_row={end_row}
                total_rows={total_rows}
                on_page_change={on_page_change.clone()}
            />
            <div 
                ref={container_ref}
                onscroll={on_scroll}
                class="flex-1 overflow-auto font-mono text-sm"
            >
                <div class="grid gap-1">
                    {for visible_rows.iter().enumerate().map(|(i, row)| {
                        let row_abs_idx = start_row + i;
                        let offset = props.virtual_address + (row_abs_idx * 16) as u64;
                        html! {
                            <HexRow row={row.clone()} offset={offset} />
                        }
                    })}
                </div>
            </div>
            <HexViewerFooter 
                current_page={*current_page}
                total_pages={total_pages}
                total_rows={total_rows}
                data_len={props.data.len()}
                on_page_change={on_page_change}
            />
        </div>
    }
}