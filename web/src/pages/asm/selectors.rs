use web_sys::HtmlSelectElement;
use yew::prelude::*;
use crate::pages::asm::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(DecoderSelector)]
pub fn decoder_selector(props: &Props) -> Html {
    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |event: Event| {
            let select = event.target_unchecked_into::<HtmlSelectElement>();
            let value = select.value();
           
            on_change.emit(value);
        })
    };
    
    html! {
        <select 
            class="bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-sm text-zinc-200"
            onchange={on_change}
        >
            <option 
                value="iced" 
                selected={props.value == "iced-x86"}
            >
                {"iced-x86"}
            </option>
            <option 
                value="prometheus" 
                selected={props.value == "prometheus"}
            >
                {"prometheus"}
            </option>
        </select>
    }
}