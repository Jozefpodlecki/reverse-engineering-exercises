use yew::prelude::*;
use crate::pages::pe_inspector::types::ImportModule;

#[derive(Properties, PartialEq)]
pub struct ImportsTabProps {
    pub imports: Vec<ImportModule>,
}

#[function_component(ImportsTab)]
pub fn imports_tab(props: &ImportsTabProps) -> Html {
    if props.imports.is_empty() {
        return html! {
            <div class="p-8 text-center text-zinc-500">
                {"No imports found"}
            </div>
        };
    }

    html! {
        <div class="space-y-4 p-4">
            {for props.imports.iter().map(|module| {
                html! {
                    <div class="border border-zinc-800 rounded-lg overflow-hidden">
                        <div class="px-4 py-2 bg-zinc-900/60 border-b border-zinc-800">
                            <h3 class="text-sm font-medium text-blue-400 font-mono">{&module.name}</h3>
                        </div>
                        <div class="overflow-x-auto">
                            <table class="w-full text-sm">
                                <thead class="bg-zinc-900/30">
                                    <tr class="text-left text-zinc-400">
                                        <th class="px-4 py-2">{"Name"}</th>
                                        <th class="px-4 py-2">{"Hint"}</th>
                                        <th class="px-4 py-2">{"RVA"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {for module.functions.iter().map(|func| {
                                        html! {
                                            <tr class="border-t border-zinc-800/50">
                                                <td class="px-4 py-2 font-mono text-zinc-300">
                                                    {func.name.as_deref().unwrap_or("(by ordinal)")}
                                                </td>
                                                <td class="px-4 py-2 font-mono text-zinc-400">{func.hint}</td>
                                                <td class="px-4 py-2 font-mono text-zinc-400">{format!("0x{:08X}", func.rva)}</td>
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}