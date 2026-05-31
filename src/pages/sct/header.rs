use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use super::*;
use crate::{route::Route, services::StorageService};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PageHeader)]
pub fn page_header(props: &Props) -> Html {
    html! {
        <header class="border-b border-zinc-800 bg-zinc-900/50 sticky top-0 z-10">
            <div class="container mx-auto px-6 py-2">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <Link<Route> to={Route::Home} classes="p-1.5 text-zinc-400 hover:text-zinc-200 transition rounded-lg hover:bg-zinc-800/50 block group">
                            <Icon data={IconData::LUCIDE_HOME} width="1.1rem" height="1.1rem" />
                        </Link<Route>>
                        <span class="text-zinc-700 select-none text-sm">{"|"}</span>
                        <a 
                            href="https://github.com/Jozefpodlecki/reverse-engineering-exercises"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="p-1.5 text-zinc-400 hover:text-zinc-200 transition rounded-lg hover:bg-zinc-800/50 block group"
                        >
                            <Icon data={IconData::LUCIDE_GITHUB} width="1.1rem" height="1.1rem" />
                        </a>
                    </div>
                    
                    <div class="absolute left-1/2 transform -translate-x-1/2">
                        <h1 class="text-base font-medium text-zinc-300">
                            {"System Call Table"}
                        </h1>
                    </div>
                    
                    <div class="w-16"></div>
                </div>
            </div>
        </header>
    }
}