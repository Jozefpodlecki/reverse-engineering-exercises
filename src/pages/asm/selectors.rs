use yew::prelude::*;
use crate::pages::asm::types::DecoderKind;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: DecoderKind,
    pub on_change: Callback<DecoderKind>,
}

#[function_component(DecoderSelector)]
pub fn decoder_selector(props: &Props) -> Html {
    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = select.value();
            let kind = match value.as_str() {
                "prometheus" => DecoderKind::Prometheus,
                _ => DecoderKind::IcedX86,
            };
            on_change.emit(kind);
        })
    };
    
    html! {
        <select 
            class="bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-sm text-zinc-200"
            onchange={on_change}
        >
            <option 
                value="iced" 
                selected={matches!(props.value, DecoderKind::IcedX86)}
            >
                {"iced-x86"}
            </option>
            <option 
                value="prometheus" 
                selected={matches!(props.value, DecoderKind::Prometheus)}
            >
                {"prometheus"}
            </option>
        </select>
    }
}