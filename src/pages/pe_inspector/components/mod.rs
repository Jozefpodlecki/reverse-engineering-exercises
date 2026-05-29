use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InfoCardProps {
    pub title: &'static str,
    pub children: Html,
}

#[function_component(InfoCard)]
pub fn info_card(props: &InfoCardProps) -> Html {
    html! {
        <div class="border border-zinc-800 rounded-lg bg-zinc-900/40 overflow-hidden">
            <div class="px-4 py-2 border-b border-zinc-800 bg-zinc-900/60">
                <h3 class="text-sm font-medium text-zinc-300">{props.title}</h3>
            </div>
            <div class="p-4 space-y-2">
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct InfoRowProps {
    pub label: &'static str,
    pub value: String,
}

#[function_component(InfoRow)]
pub fn info_row(props: &InfoRowProps) -> Html {
    html! {
        <div class="flex flex-col gap-1">
            <div class="text-xs text-zinc-500 uppercase tracking-wide">{props.label}</div>
            <div class="text-sm text-zinc-300 font-mono break-all">{&props.value}</div>
        </div>
    }
}