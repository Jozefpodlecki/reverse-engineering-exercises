use yew::prelude::*;
use crate::pages::pe_inspector::types::Export;

#[derive(Properties, PartialEq)]
pub struct ExportsTabProps {
    pub exports: Vec<Export>,
}

#[function_component(ExportsTab)]
pub fn exports_tab(props: &ExportsTabProps) -> Html {
    if props.exports.is_empty() {
        return html! {
            <div class="p-8 text-center text-zinc-500">
                {"No exports found"}
            </div>
        };
    }

    html! {
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <thead class="border-b border-zinc-800 bg-zinc-900/40">
                    <tr class="text-left text-zinc-400">
                        <th class="px-4 py-2">{"Name"}</th>
                        <th class="px-4 py-2">{"Ordinal"}</th>
                        <th class="px-4 py-2">{"RVA"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for props.exports.iter().map(|export| {
                        html! {
                            <tr class="border-b border-zinc-800/50 hover:bg-zinc-800/30">
                                <td class="px-4 py-2 font-mono text-zinc-300">{&export.name}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{export.ordinal}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", export.rva)}</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}