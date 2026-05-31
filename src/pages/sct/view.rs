use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use crate::pages::sct::*;
use super::PageHeader;

#[function_component(SystemCallTable)]
pub fn system_call_table() -> Html {
    let selected_family = use_state(|| OsFamily::Windows);
    let selected_versions = use_state(|| {
        let mut set = std::collections::HashSet::new();
        set.insert(WindowsVersion::Win10);
        set.insert(WindowsVersion::Win11);
        set
    });
    
    let show_selector = use_state(|| false);

    let toggle_version = {
        let selected_versions = selected_versions.clone();
        Callback::from(move |version: WindowsVersion| {
            let mut new_set = (*selected_versions).clone();
            if new_set.contains(&version) {
                new_set.remove(&version);
            } else {
                new_set.insert(version);
            }
            selected_versions.set(new_set);
        })
    };

    let select_all = {
        let selected_versions = selected_versions.clone();
        Callback::from(move |_| {
            let new_set = WindowsVersion::all().into_iter().collect();
            selected_versions.set(new_set);
        })
    };

    let clear_all = {
        let selected_versions = selected_versions.clone();
        Callback::from(move |_| {
            selected_versions.set(std::collections::HashSet::new());
        })
    };

    let on_family_change = {
        let selected_family = selected_family.clone();
        Callback::from(move |family: OsFamily| {
            selected_family.set(family);
        })
    };

    let toggle_selector = {
        let show_selector = show_selector.clone();
        Callback::from(move |_| {
            show_selector.set(!*show_selector);
        })
    };

    html! {
        <div data-container=true class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
            <PageHeader/>
            <div class="flex-1 container mx-auto px-6 py-8">
                <div class="max-w-6xl mx-auto space-y-6">
                    <FilterPanel
                        show={*show_selector}
                        selected_family={(*selected_family).clone()}
                        selected_versions={(*selected_versions).clone()}
                        on_toggle={toggle_selector}
                        on_family_change={on_family_change}
                        on_toggle_version={toggle_version}
                        on_select_all={select_all}
                        on_clear_all={clear_all}
                    />
                    
                    <SyscallTable selected={(*selected_versions).clone()} />
                </div>
            </div>
        </div>
    }
}