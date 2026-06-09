use std::collections::HashSet;

use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::{hooks::use_location, prelude::Link};

use crate::{pages::sct::*, shared::{LoadingScreen, PageHeader}};

#[derive(Default, PartialEq, Clone)]
pub enum QuizUiState {
    #[default]
    Loading,
    Loaded,
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct SyscallQuery {
    pub os: Option<String>,
    pub arch: Option<String> ,
}

impl SyscallQuery {
    fn from_query_str(query: &str) -> Self {
        let query = query.strip_prefix('?').unwrap_or("");
        let mut os = None;
        let mut arch = None;
        
        for part in query.split('&') {
            let mut parts = part.split('=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                match key {
                    "os" => os = Some(value.to_string()),
                    "arch" => arch = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        
        Self { os, arch }
    }
}

#[function_component(SystemCallTable)]
pub fn system_call_table() -> Html {
    let ui_state = use_state(QuizUiState::default);
    // let filter = use_state(Filter::default);
    // ?arch=windows
    let location = use_location().unwrap();
    
    // Get the query string (e.g., "?os=windows&arch=x64")
    let query = location.query_str();

    {

        use_effect_with((), move |_| {
            
            || ()
        });
    }

    // let on_change = {
    //     // let filter = filter.clone();
    //     Callback::from(move |_| {
            
    //     })
    // };

    let content = match &*ui_state {
        QuizUiState::Loaded => {
            html! {
                <div class="flex-1 container mx-auto px-6 py-8">
                    <div class="max-w-6xl mx-auto space-y-6">
                        // <FilterPanel
                        //     filter={filter}
                        //     on_change={on_change}
                        // />
                        
                        // <SyscallTable />
                    </div>
                </div>
            }
        },
        QuizUiState::Loading => {
            html! {
                <LoadingScreen />
            }
        },
        QuizUiState::Error(_) => {
            html! {
                <LoadingScreen />
            }
        },
    };

    html! {
        <div data-container=true class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
            <PageHeader title="System Call Table"/>
            {content}
        </div>
    }
}