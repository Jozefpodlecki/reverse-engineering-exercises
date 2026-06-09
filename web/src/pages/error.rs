use yew::*;

use crate::{models::AppError};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub error: AppError,
    pub on_reload: Callback<MouseEvent>,
}

#[function_component(ErrorPage)]
pub fn error(props: &Props) -> Html {

    html! {
        <>
            <div class="h-0 sticky top-1 right-1">
            </div>
            <div>
            </div>
        </>
    }
}