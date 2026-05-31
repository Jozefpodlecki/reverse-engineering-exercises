use gloo::timers::future::TimeoutFuture;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use super::*;
use crate::{pages::pe_inspector::{ParsedPe, context::PeContext, header::PageHeader}, route::Route, services::StorageService, shared::LoadingScreen};

const STORAGE_KEY: &str = "pe_inspector_cache";
const MAX_CACHE_SIZE: usize = 2_000_000;

#[function_component(PeInspector)]
pub fn pe_inspector() -> Html {
    let ui_state = use_state(InspectorUiState::default);
    let pe_state = use_state(PeState::default);
    let parsed_pe = use_state(ParsedPe::default);
    let error = use_state(|| None::<String>);
    let show_logs = use_state(|| false);
    
    let storage = use_state(|| StorageService::new(STORAGE_KEY));

    {
        let storage = storage.clone();
        let ui_state = ui_state.clone();
        let pe_state = pe_state.clone();
        let parsed_pe = parsed_pe.clone();
        
        use_effect_with((), move |_| {
            if let Some(cached) = storage.load::<PeState>() {
                if let Ok(parsed) = ParsedPe::from_bytes(&cached.data) {
                    parsed_pe.set(parsed);
                }
                pe_state.set(cached);
                ui_state.set(InspectorUiState::Loaded);
            }
            || ()
        });
    }

    let on_file_load = {
        let ui_state = ui_state.clone();
        let pe_state = pe_state.clone();
        let parsed_pe = parsed_pe.clone();
        let error = error.clone();
        let storage = storage.clone();

        Callback::from(move |new_state: PeState| {
            let file_size = new_state.size;
            ui_state.set(InspectorUiState::Loading);

            match ParsedPe::from_bytes(&new_state.data) {
                Ok(parsed) => {
                    parsed_pe.set(parsed);
                    error.set(None);
                },
                Err(error_msg) => {
                    error.set(Some(error_msg));
                    ui_state.set(InspectorUiState::Idle);
                    return;
                },
            }
            
            if file_size <= MAX_CACHE_SIZE as u64 {
                let _ = storage.save(&new_state);
            }

            pe_state.set(new_state);
            
            let ui_state_clone = ui_state.clone();
            spawn_local(async move {
                TimeoutFuture::new(100).await;
                ui_state_clone.set(InspectorUiState::Loaded);
            });
        })
    };

    let on_close = {
        let ui_state = ui_state.clone();
        let pe_state = pe_state.clone();
        let parsed_pe = parsed_pe.clone();
        let error = error.clone();
        let storage = storage.clone();

        Callback::from(move |_| {
            storage.remove();
            pe_state.set(PeState::default());
            parsed_pe.set(Default::default());
            error.set(None);
            ui_state.set(InspectorUiState::Idle);
        })
    };

    let has_errors = error.is_some();

    let on_show_logs = {
        let show_logs = show_logs.clone();
        Callback::from(move |_| show_logs.set(true))
    };
    
    let on_close_logs = {
        let show_logs = show_logs.clone();
        Callback::from(move |_| show_logs.set(false))
    };

    let context = PeContext::new((*pe_state).clone(), (*parsed_pe).clone());

    let content = match &*ui_state {
        InspectorUiState::Idle => html! {
            <DropZone on_load={on_file_load} error={(*error).clone()} />
        },
        InspectorUiState::Loading => html! {
            <div class="flex items-center justify-center h-[calc(100vh-8rem)]">
                <div class="max-w-2xl w-full">
                    <LoadingScreen />
                </div>
            </div>
        },
        InspectorUiState::Loaded => html! {
            <ContextProvider<PeContext> context={context}>
                <PeViewer on_close={on_close} />
            </ContextProvider<PeContext>>
        },
        InspectorUiState::Error(err) => html! {
            <div class="flex items-center justify-center h-[calc(100vh-8rem)]">
                <div class="max-w-2xl w-full">
                    <DropZone on_load={on_file_load} error={Some(err.clone())} />
                </div>
            </div>
        },
    };

    html! {
         <div data-container=true class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
            <div data-header=true class="container mx-auto px-4 py-4">
                <PageHeader on_show_logs={on_show_logs} has_logs={has_errors} />
            </div>
            { content }
            <LogModal
                is_open={*show_logs}
                on_close={on_close_logs}
                warnings={vec![]}
                errors={error.as_ref().map_or(vec![], |error_message| vec![error_message.clone()])}
            />
        </div>
    }
}