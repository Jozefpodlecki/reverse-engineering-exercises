use yew::prelude::*;
use crate::pages::pe_inspector::types::{Relocation, RelocationType};

#[derive(Properties, PartialEq)]
pub struct RelocationBlockProps {
    pub relocation: Relocation,
}

#[function_component(RelocationBlock)]
pub fn relocation_block(props: &RelocationBlockProps) -> Html {
    html! {
        <div class="border border-zinc-800 rounded-lg overflow-hidden">
            <div class="px-4 py-2 bg-zinc-900/60 border-b border-zinc-800">
                <div class="flex items-center justify-between">
                    <h3 class="text-sm font-medium text-blue-400 font-mono">
                        {format!("Virtual Address: 0x{:08X}", props.relocation.virtual_address)}
                    </h3>
                    <span class="text-xs text-zinc-500">
                        {format!("Size: {} bytes", props.relocation.size_of_block)}
                    </span>
                </div>
            </div>
            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead class="bg-zinc-900/30">
                        <tr class="text-left text-zinc-400">
                            <th class="px-4 py-2">{"RVA"}</th>
                            <th class="px-4 py-2">{"Type"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for props.relocation.entries.iter().map(|entry| {
                            html! {
                                <tr class="border-t border-zinc-800/50">
                                    <td class="px-4 py-2 font-mono text-zinc-300">
                                        {format!("0x{:08X}", entry.rva)}
                                    </td>
                                    <td class="px-4 py-2 font-mono text-zinc-400">
                                        {format!("{} ({})", entry.typ.as_str(), entry.typ.value())}
                                    </td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct RelocationsSummaryProps {
    pub relocations: Vec<Relocation>,
}

#[function_component(RelocationsSummary)]
pub fn relocations_summary(props: &RelocationsSummaryProps) -> Html {
    let total_entries: usize = props.relocations.iter().map(|r| r.entries.len()).sum();
    
    html! {
        <div class="bg-zinc-900/40 border border-zinc-800 rounded-lg p-3">
            <div class="flex gap-6 text-sm">
                <div>
                    <span class="text-zinc-500">{"Blocks:"}</span>
                    <span class="text-zinc-300 ml-2 font-mono">{props.relocations.len()}</span>
                </div>
                <div>
                    <span class="text-zinc-500">{"Total Entries:"}</span>
                    <span class="text-zinc-300 ml-2 font-mono">{total_entries}</span>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct RelocationsTabProps {
    pub relocations: Vec<Relocation>,
}

#[function_component(RelocationsTab)]
pub fn relocations_tab(props: &RelocationsTabProps) -> Html {
    if props.relocations.is_empty() {
        return html! {
            <div class="p-8 text-center text-zinc-500">
                {"No relocations found"}
            </div>
        };
    }

    html! {
        <div class="space-y-4 p-4">
            <RelocationsSummary relocations={props.relocations.clone()} />
            
            {for props.relocations.iter().map(|reloc| {
                html! {
                    <RelocationBlock relocation={reloc.clone()} />
                }
            })}
        </div>
    }
}