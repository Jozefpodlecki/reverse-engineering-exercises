use yew::prelude::*;
use crate::pages::pe_inspector::types::Section;

#[derive(Properties, PartialEq)]
pub struct SectionsTabProps {
    pub sections: Vec<Section>,
}

#[function_component(SectionsTab)]
pub fn sections_tab(props: &SectionsTabProps) -> Html {
    if props.sections.is_empty() {
        return html! {
            <div class="p-8 text-center text-zinc-500">
                {"No sections found"}
            </div>
        };
    }

    html! {
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <thead class="border-b border-zinc-800 bg-zinc-900/40">
                    <tr class="text-left text-zinc-400">
                        <th class="px-4 py-2">{"Name"}</th>
                        <th class="px-4 py-2">{"Virtual Address"}</th>
                        <th class="px-4 py-2">{"Virtual Size"}</th>
                        <th class="px-4 py-2">{"Raw Size"}</th>
                        <th class="px-4 py-2">{"Raw Address"}</th>
                        <th class="px-4 py-2">{"Characteristics"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for props.sections.iter().map(|section| {
                        html! {
                            <tr class="border-b border-zinc-800/50 hover:bg-zinc-800/30">
                                <td class="px-4 py-2 font-mono text-zinc-300">{&section.name}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", section.virtual_address)}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", section.virtual_size)}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", section.size_of_raw_data)}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", section.pointer_to_raw_data)}</td>
                                <td class="px-4 py-2 font-mono text-zinc-300">{format!("0x{:08X}", section.characteristics)}</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}