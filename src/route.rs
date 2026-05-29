use yew::*;
use yew_router::Routable;

use crate::pages::{pe_inspector::PeInspector, *};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pe-inspector")]
    PeInspector,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::PeInspector => html! { <PeInspector /> },
    }
}