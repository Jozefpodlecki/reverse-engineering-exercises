use yew::prelude::*;

#[function_component(LoadingScreen)]
pub fn loading_screen() -> Html {
    html! {
        <div class="flex justify-center items-center min-h-[400px]">
            <div class="grid grid-cols-3 gap-[15px]" style="--size: 10px; --scale: 1.5;">
                {for (0..9).map(|i| {
                    let delay = format!("{}ms", i * 75);
                    html! {
                        <div 
                            class="w-[15px] h-[15px] bg-blue-500 rounded-sm animate-pulse"
                            style={format!("animation-delay: {}; animation-duration: 675ms; animation-iteration-count: infinite; animation-timing-function: ease-in-out; animation-direction: alternate; opacity: 0.2;", delay)}
                        />
                    }
                })}
            </div>
        </div>
    }
}