use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use crate::{pages::quiz::{form::QuizForm, manager::SessionManager, session::QuizSessionComponent, types::*}, services::StorageService, shared::LoadingScreen};

#[function_component(Quiz)]
pub fn quiz() -> Html {
    let ui_state = use_state(|| QuizUiState::Loading);
    let storage = use_state(|| StorageService::new("quiz"));
    let manager: UseStateHandle<SessionManager> = use_state(SessionManager::new);
    let session: UseStateHandle<Option<QuizSession>> = use_state(|| None);

    {
        let storage = storage.clone();
        let session = session.clone();
        let ui_state = ui_state.clone();
        
        use_effect_with((), move |_| {
            if let Some(saved_session) = storage.load::<QuizSession>() {
                session.set(Some(saved_session));
                ui_state.set(QuizUiState::Active);
            }
            else {
                ui_state.set(QuizUiState::Idle);
            }
            || ()
        });
    }

    let content = match &*ui_state {
        QuizUiState::Idle => {
            html! {
                <QuizForm/>
            }
            
        },
        QuizUiState::Loading => {
            html! {
                <LoadingScreen />
            }
        },
        QuizUiState::Active => {
            html! {
                <QuizSessionComponent/>
            }
        },
        QuizUiState::Completed => {
            html! {}
        },
        QuizUiState::Error(_) => {
            html! {}
        },
    };

    html! {
        <ContextProvider<SessionManager> context={(&*manager).clone()}>
            {content}
        </ContextProvider<SessionManager>>
    }
}