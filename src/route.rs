use yew::*;
use yew_router::Routable;

use crate::pages::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pe-inspector")]
    PeInspector,
    #[at("/emulator")]
    Emulator,
    #[at("/quiz")]
    Quiz,
    #[at("/asm")]
    Asm,
    #[at("/system-call-table")]
    SystemCallTable,
    #[at("/pe-builder")]
    PeBuilder,
    #[at("/wiki")]
    Wiki,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::PeInspector => html! { <PeInspector /> },
        Route::Emulator => html! { <Emulator /> },
        Route::Quiz => html! { <Quiz /> },
        Route::Asm => html! { <Asm /> },
        Route::SystemCallTable => html! { <SystemCallTable /> },
        Route::PeBuilder => html! { <PeBuilder /> },
        Route::Wiki => html! { <Wiki /> },
    }
}