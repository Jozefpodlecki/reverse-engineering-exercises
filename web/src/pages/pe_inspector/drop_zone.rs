use js_sys::Uint8Array;
use yew::prelude::*;
use web_sys::{DragEvent, File, FileReader, HtmlInputElement};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::ProgressEvent;

use crate::pages::pe_inspector::PeState;

#[derive(Properties, PartialEq)]
pub struct DropZoneProps {
    pub on_load: Callback<PeState>,
    pub error: Option<String>,
}

#[function_component(DropZone)]
pub fn drop_zone(props: &DropZoneProps) -> Html {
    let input_ref = use_node_ref();
    let on_load = props.on_load.clone();
    let error = props.error.clone();
    let is_dragging = use_state(|| false);

    let handle_file = {
        let on_load = on_load.clone();

        move |file: File| {
            let file_name = file.name();
            let last_modified = file.last_modified();

            let reader = FileReader::new().unwrap();
            let on_load_clone = on_load.clone();

            let onloadend = Closure::<dyn FnMut(ProgressEvent)>::new(move |event: ProgressEvent| {
                let reader: FileReader = event.target().unwrap().unchecked_into();
                let array = Uint8Array::new(&reader.result().unwrap());

                let mut data = vec![0; array.length() as usize];
                array.copy_to(&mut data);

                on_load_clone.emit(PeState {
                    last_modified,
                    file_name: file_name.clone(),
                    size: data.len() as u64,
                    data: data.into(),
                });
            });

            reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
            reader.read_as_array_buffer(&file).unwrap();

            onloadend.forget();
        }
    };

    let on_drag_enter = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |drag_event: DragEvent| {
            drag_event.prevent_default();
            is_dragging.set(true);
        })
    };

    let on_drag_leave = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |drag_event: DragEvent| {
            drag_event.prevent_default();
            is_dragging.set(false);
        })
    };

    let on_drop = {
        let handle_file = handle_file.clone();
        let is_dragging = is_dragging.clone();

        Callback::from(move |drag_event: DragEvent| {
            drag_event.prevent_default();
            is_dragging.set(false);

            if let Some(file) = drag_event
                .data_transfer()
                .and_then(|transfer| transfer.files())
                .and_then(|files| files.get(0))
            {
                handle_file(file);
            }
        })
    };

    let on_drag_over = Callback::from(|drag_event: DragEvent| {
        drag_event.prevent_default();
    });

    let on_click = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            input.click();
        })
    };

    let on_file_input_change = {
        let handle_file = handle_file.clone();

        Callback::from(move |change_event: Event| {
            let input: HtmlInputElement = change_event.target_unchecked_into();

            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                handle_file(file);
            }
        })
    };

    let error_html = error.as_ref().map(|error_message| {
        html! {
            <div class="mt-4 p-3 bg-red-500/10 border border-red-500/50 rounded-lg text-red-400 text-sm">
                <div class="flex items-center gap-2">
                    <span class="text-lg">{"⚠️"}</span>
                    <span>{error_message}</span>
                </div>
            </div>
        }
    });

    let base_classes = "border-2 border-dashed rounded-xl p-10 text-center transition-all duration-200 cursor-pointer";
    
    let dragging_classes = if *is_dragging {
        "border-blue-500 bg-blue-500/10 scale-105"
    } else if error.is_some() {
        "border-red-500/50 bg-red-900/10 hover:bg-red-900/20"
    } else {
        "border-zinc-700 bg-zinc-900/20 hover:bg-zinc-900/30 hover:border-zinc-500"
    };

    let scale_class = if *is_dragging { "scale-110" } else { "scale-100" };
    let image_opacity = if *is_dragging { "opacity-100" } else { "opacity-50" };
    let drop_text = if *is_dragging { "Drop your file here" } else { "Drag & drop file here" };
    
    let text_color = if error.is_some() {
        "text-red-400"
    } else if *is_dragging {
        "text-blue-400"
    } else {
        "text-zinc-400"
    };

    html! {
        <div 
            class="flex-1 flex items-center justify-center cursor-pointer"
            onclick={on_click}
            ondragenter={on_drag_enter}
            ondragleave={on_drag_leave}
            ondrop={on_drop}
            ondragover={on_drag_over}
        >
            <div class={format!("{} {}", base_classes, dragging_classes)}>
                <div class="flex flex-col items-center justify-center text-center p-10">
                    <div class={format!("flex justify-center mb-4 transition-transform duration-200 {}", scale_class)}>
                        <img 
                            src="public/images/upload.png" 
                            alt="upload" 
                            class={format!("w-20 h-20 opacity-50 invert transition-all duration-200 {}", image_opacity)}
                        />
                    </div>
                    <p class={text_color}>
                        {drop_text}
                    </p>
                    <p class="text-xs text-zinc-600 mt-2">
                        {"or click to select file"}
                    </p>
                    {error_html}
                </div>
            </div>
            <input
                ref={input_ref}
                type="file"
                class="hidden"
                onchange={on_file_input_change}
            />
        </div>
    }
}