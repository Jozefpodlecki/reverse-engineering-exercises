use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub route: Route,
    pub icon_src: &'static str,
    pub icon_alt: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <Link<Route> to={props.route.clone()} classes="block group h-full">
            <div class="border border-zinc-800 rounded-2xl p-3 bg-zinc-900/30 hover:bg-zinc-900/50 hover:border-cyan-500/40 transition space-y-2 h-full flex flex-col">
                <div class="flex items-center gap-4">
                    <img
                        src={props.icon_src}
                        class="w-10 h-10 opacity-80 group-hover:opacity-100 transition"
                        alt={props.icon_alt}
                    />
                    <div class="text-xl font-semibold">
                        { props.title }
                    </div>
                </div>
                <p class="text-sm text-zinc-400 leading-relaxed flex-1">
                    { props.description }
                </p>
                <div class="flex items-center text-xs text-zinc-500">
                    {
                        props.tags.iter().take(3).map(|tag| html! {
                            <span class="px-2 py-1 bg-zinc-800/50 rounded-full">
                                { tag }
                            </span>
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </Link<Route>>
    }
}