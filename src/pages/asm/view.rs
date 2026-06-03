use yew::prelude::*;
use yew_icons::{Icon, IconData};
use crate::pages::asm::*;
use crate::shared::PageHeader;

#[function_component(Asm)]
pub fn asm() -> Html {
    let manager_state = use_state(TabManagerState::new);
    let manager = TabManager::new(manager_state);
    let emulator_state = use_state(EmulatorState::new);
    let emulator = Emulator::new(emulator_state);
    
    html! {
        <ContextProvider<TabManager> context={manager}>
            <ContextProvider<Emulator> context={emulator}>
                <div class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
                    <PageHeader title="Asm" />
                    <Tabs />
                    <TabComponent/>
                </div>
            </ContextProvider<Emulator>>
        </ContextProvider<TabManager>>
    }
}