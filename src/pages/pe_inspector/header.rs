use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use super::*;
use crate::{pages::pe_inspector::{ParsedPe, context::PeContext}, route::Route, services::StorageService};

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub on_show_logs: Callback<MouseEvent>,
    pub has_logs: bool,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <div class="mb-6 flex items-center justify-between">
            
            <div class="flex items-center gap-3">
                <Link<Route> to={Route::Home} classes="p-2 text-zinc-400 hover:text-zinc-300 transition rounded-lg hover:bg-zinc-800/50 block group">
                    <Icon data={IconData::LUCIDE_HOME} width="1.25rem" height="1.25rem" />
                </Link<Route>>
                <span class="text-zinc-600">{"|"}</span>
                <a 
                    href="https://github.com/Jozefpodlecki/reverse-engineering-exercises"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="p-2 text-zinc-400 hover:text-zinc-300 transition rounded-lg hover:bg-zinc-800/50 block group"
                >
                    <Icon data={IconData::LUCIDE_GITHUB} width="1.25rem" height="1.25rem" />
                </a>
            </div>
            
            <div class="text-center">
                <h1 class="text-2xl font-bold">{"PE Inspector"}</h1>
                <p class="text-zinc-500 text-sm">{"Portable Executable file analyzer"}</p>
            </div>

            if props.has_logs {
                <button
                    type="button"
                    onclick={&props.on_show_logs}
                    class="relative p-2 text-zinc-400 hover:text-zinc-300 transition rounded-lg hover:bg-zinc-800/50"
                >
                    <Icon data={IconData::LUCIDE_ALERT_TRIANGLE} width="1.25rem" height="1.25rem" />
                    <span class="absolute -top-1 -right-1 w-2 h-2 bg-yellow-500 rounded-full"></span>
                </button>
            } else {
                <div class="w-10 h-10"></div>
            }
        </div>
    }
}
