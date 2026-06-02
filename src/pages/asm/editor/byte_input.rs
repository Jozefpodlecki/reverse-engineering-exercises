use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub value: Option<u8>,
    pub on_change: Callback<(usize, String)>,
}

#[function_component(ByteInput)]
pub fn byte_input(props: &Props) -> Html {
    let input_ref = use_node_ref();
    
    let display_value = props.value.map(|b| format!("{:02X}", b)).unwrap_or_default();
    
    let on_change = {
        let on_change = props.on_change.clone();
        let index = props.index;
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            on_change.emit((index, input.value()));
        })
    };
    
      let on_input = {
        let on_change = props.on_change.clone();
        let index = props.index;
        let input_ref = input_ref.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let raw_value = input.value();
            
            // Filter only hex characters (0-9, A-F, a-f)
            let filtered: String = raw_value
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .map(|c| c.to_ascii_uppercase())
                .collect();
            
            if filtered != raw_value {
                input.set_value(&filtered);
            }
            
            let limited = &filtered[..filtered.len().min(2)];
            if limited != filtered {
                input.set_value(limited);
            }
            
            on_change.emit((index, limited.to_string()));
            
            // Auto-focus next on 2 chars
            if limited.len() == 2 {
                if let Some(next) = input_ref.cast::<HtmlInputElement>().and_then(|el| el.next_element_sibling()) {
                    next.dyn_into::<HtmlInputElement>().ok().map(|el| el.focus().ok());
                }
            }
        })
    };
    
    let on_keydown = {
        let input_ref = input_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            let input = event.target_unchecked_into::<HtmlInputElement>();

            if key == "Enter" {
                
            }

            if key == "Backspace" {
                
                if input.value().is_empty() {
                    if let Some(prev) = input_ref.cast::<HtmlInputElement>().and_then(|el| el.previous_element_sibling()) {
                        prev.dyn_into::<HtmlInputElement>().ok().map(|el| el.focus().ok());
                    }
                }
            }
        })
    };
    
    html! {
        <input 
            ref={input_ref}
            type="text"
            maxlength="2"
            size="2"
            class="w-10 text-center bg-zinc-800 border border-zinc-700 rounded px-1 py-1 text-sm font-mono text-zinc-200 focus:outline-none focus:border-cyan-500"
            placeholder="--"
            value={display_value}
            onchange={on_change}
            oninput={on_input}
            onkeydown={on_keydown}
        />
    }
}