use std::rc::Rc;

use web_sys::{Document, HtmlElement, Navigator, Storage, Window};
use yew::*;
use yew_router::{HashRouter, Switch};

use crate::{route::{switch, Route}, services::*};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppProps {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub local_storage: Storage,
    pub navigator: Navigator,
    pub app_name: Rc<str>,
    pub version: Rc<str>,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {

    let AppProps {
        window,
        document,
        body,
        local_storage,
        navigator,
        app_name,
        version
    } = props;

    let api_client = ApiClient::new(window.clone());
    let list = document.document_element().unwrap().class_list();
    let class_toggle = ClassToggle::new(list, "dark".into());
    let is_dark_mode = class_toggle.contains();
    let handle: UseStateHandle<bool> = use_state(|| is_dark_mode);

    html! {
        <ContextProvider<ApiClient> context={api_client}>
                <HashRouter>
                    <Switch<Route> render={switch} />
                </HashRouter>
        </ContextProvider<ApiClient>>
    }
}