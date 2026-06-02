use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use crate::shared::PageHeader;

#[function_component(PeBuilder)]
pub fn pe_builder() -> Html {
    
    let content = html!{};

    html! {
        <div data-container=true class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
            <PageHeader title="PE Builder"/>
            {content}
        </div>
    }
}
    