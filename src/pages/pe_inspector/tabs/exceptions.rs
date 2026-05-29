use yew::prelude::*;
use crate::pages::pe_inspector::types::ExceptionHandler;

#[derive(Properties, PartialEq)]
pub struct ExceptionsTabProps {
    pub exceptions: Vec<ExceptionHandler>,
}

#[function_component(ExceptionsTab)]
pub fn exceptions_tab(props: &ExceptionsTabProps) -> Html {
    if props.exceptions.is_empty() {
        return html! {
            <div class="p-8 text-center text-zinc-500">
                {"No exception handlers found"}
            </div>
        };
    }

    html! {
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <thead class="border-b border-zinc-800 bg-zinc-900/40">
                    <tr class="text-left text-zinc-400">
                        <th class="px-4 py-2">{"Begin Address"}</th>
                        <th class="px-4 py-2">{"End Address"}</th>
                        <th class="px-4 py-2">{"Unwind Address"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for props.exceptions.iter().map(|handler| {
                        html! {
                            <tr class="border-b border-zinc-800/50 hover:bg-zinc-800/30">
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", handler.begin_address)}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", handler.end_address)}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", handler.unwind_address)}</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}