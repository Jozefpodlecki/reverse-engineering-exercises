use yew::prelude::*;
use yew_icons::{Icon, IconData};
use crate::pages::asm::tab::TabComponent;
use crate::pages::asm::tab_manager::{TabManager, TabManagerState};
use crate::pages::asm::tabs::Tabs;
use crate::shared::PageHeader;
use crate::pages::asm::types::{Tab, DecoderKind};

#[function_component(Asm)]
pub fn asm() -> Html {
    let manager_state = use_state(TabManagerState::new);
    let manager = TabManager::new(manager_state);
    
    html! {
        <ContextProvider<TabManager> context={manager}>
            <div class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
                <PageHeader title="Asm" />
                <Tabs />
                <TabComponent/>
            </div>
        </ContextProvider<TabManager>>
    }
}