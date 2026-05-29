use yew::prelude::*;
use yew_icons::{Icon, IconData};

use crate::pages::pe_inspector::sections::SectionsTab;
use crate::pages::pe_inspector::tabs::{
    GeneralTab, ImportsTab, ExportsTab, ExceptionsTab, RelocationsTab, ActiveTab, TabBar
};
use crate::pages::pe_inspector::context::PeContext;

#[derive(Properties, PartialEq)]
pub struct PeViewerProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(PeViewer)]
pub fn pe_viewer(props: &PeViewerProps) -> Html {
    let context = use_context::<PeContext>().expect("No PeContext provided");
    let active_tab = use_state(ActiveTab::default);
    let on_close = props.on_close.clone();

    let on_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: ActiveTab| {
            active_tab.set(tab);
        })
    };

    let mut enabled_tabs = vec![ActiveTab::General, ActiveTab::Sections, ActiveTab::Imports];

    if !context.parsed.exports.is_empty() {
        enabled_tabs.push(ActiveTab::Exports);
    }
    if !context.parsed.exception_handlers.is_empty() {
        enabled_tabs.push(ActiveTab::Exceptions);
    }
    if !context.parsed.relocations.is_empty() {
        enabled_tabs.push(ActiveTab::Relocations);
    }

    if !enabled_tabs.contains(&*active_tab) {
        active_tab.set(ActiveTab::General);
    }

    let tab_content = match *active_tab {
        ActiveTab::General => html! {
            <GeneralTab state={context.raw.clone()} parsed={context.parsed.clone()} />
        },
        ActiveTab::Sections => html! {
            <SectionsTab sections={context.parsed.sections.clone()} raw_bytes={context.raw.data} />
        },
        ActiveTab::Imports => html! {
            <ImportsTab imports={context.parsed.imports.clone()} />
        },
        ActiveTab::Exports => html! {
            <ExportsTab exports={context.parsed.exports.clone()} />
        },
        ActiveTab::Exceptions => html! {
            <ExceptionsTab exceptions={context.parsed.exception_handlers.clone()} />
        },
        ActiveTab::Relocations => html! {
            <RelocationsTab relocations={context.parsed.relocations.clone()} />
        },
    };

    html! {
        <div class="border border-zinc-800 rounded-xl bg-zinc-900/20">
            <div class="flex items-center justify-between border-b border-zinc-800 px-4 py-2">
                <TabBar
                    tabs={enabled_tabs}
                    active_tab={(*active_tab).clone()} on_tab={on_tab} />
                <button
                    type="button"
                    onclick={on_close}
                    class="text-zinc-500 hover:text-red-400 transition text-xl leading-none px-2"
                >
                    <Icon data={IconData::LUCIDE_X}/>
                </button>
            </div>
            {tab_content}
        </div>
    }
}