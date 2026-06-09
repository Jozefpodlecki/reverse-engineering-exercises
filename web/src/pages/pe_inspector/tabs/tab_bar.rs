use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ActiveTab {
    General,
    Sections,
    Imports,
    Exports,
    Exceptions,
    Relocations,
}

impl Default for ActiveTab {
    fn default() -> Self {
        Self::General
    }
}

impl ActiveTab {
    pub fn from_str(tab: &str) -> Self {
        match tab {
            "General" => Self::General,
            "Sections" => Self::Sections,
            "Imports" => Self::Imports,
            "Exports" => Self::Exports,
            "Exceptions" => Self::Exceptions,
            "Relocations" => Self::Relocations,
            _ => Self::General,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Sections => "Sections",
            Self::Imports => "Imports",
            Self::Exports => "Exports",
            Self::Exceptions => "Exceptions",
            Self::Relocations => "Relocations",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TabButtonProps {
    pub label: &'static str,
    pub active: bool,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(TabButton)]
pub fn tab_button(props: &TabButtonProps) -> Html {
    let base_classes = "px-3 py-1.5 text-sm font-medium transition rounded-md";
    let active_classes = "text-blue-400 bg-blue-400/10";
    let inactive_classes = "text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50";

    let classes = if props.active {
        format!("{} {}", base_classes, active_classes)
    } else {
        format!("{} {}", base_classes, inactive_classes)
    };

    html! {
        <button
            type="button"
            data-tab={props.label}
            onclick={props.on_click.clone()}
            class={classes}
        >
            {props.label}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabBarProps {
    pub active_tab: ActiveTab,
    pub on_tab: Callback<ActiveTab>,
    pub tabs: Vec<ActiveTab>,
}

#[function_component(TabBar)]
pub fn tab_bar(props: &TabBarProps) -> Html {
    let tabs = &props.tabs;

    let on_tab_inner: Callback<MouseEvent> = {
        let on_tab = props.on_tab.clone();

        Callback::from(move |event: MouseEvent| {
            let target: HtmlElement = event.target_unchecked_into();
            let tab_name = target.dataset().get("tab").unwrap();
            on_tab.emit(ActiveTab::from_str(&tab_name));
        })
    };

    html! {
        <div class="flex flex-wrap gap-1">
            {for tabs.into_iter().map(|tab| {
                let label = tab.as_str();
                let active = props.active_tab == *tab;
                
                html! {
                    <TabButton
                        label={label}
                        active={active}
                        on_click={&on_tab_inner}
                    />
                }
            })}
        </div>
    }
}