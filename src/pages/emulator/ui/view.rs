use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{pages::emulator::ui::{footer::Footer, header::Header, panel::Panel, sidebar::Sidebar}, route::Route};
// use ui::{Sidebar, BottomControls, CpuView, MemoryView, ThreadsView, ModulesView};

#[derive(PartialEq, Clone)]
pub enum ActivePanel {
    Cpu,
    Memory,
    Threads,
    Modules,
    Console,
}

#[function_component(Emulator)]
pub fn emulator() -> Html {
    let active_panel = use_state(|| ActivePanel::Cpu);
    
    html! {
        <div class="h-screen bg-zinc-950 text-zinc-100 flex flex-col">
            <Header/>
            <div class="flex flex-1 overflow-hidden">
                <Sidebar />
                
                <Panel />
            </div>
            
            <Footer />
        </div>
    }
}