use yew::prelude::*;
use crate::pages::pe_inspector::{PeState, ParsedPe};
use crate::pages::pe_inspector::utils::*;
use crate::pages::pe_inspector::components::{InfoCard, InfoRow};

#[derive(Properties, PartialEq)]
pub struct GeneralTabProps {
    pub state: PeState,
    pub parsed: ParsedPe,
}

#[function_component(GeneralTab)]
pub fn general_tab(props: &GeneralTabProps) -> Html {
    let parsed = &props.parsed;

    html! {
        <div class="p-4 space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <InfoCard title="File Information">
                    <InfoRow label="Name" value={props.state.file_name.clone()} />
                    <InfoRow label="Size" value={format!("{} bytes ({:.2} KB)", 
                        props.state.size,
                        props.state.size as f64 / 1024.0
                    )} />
                    <InfoRow label="Modified" value={format_timestamp(props.state.last_modified)} />
                </InfoCard>
            </div>
        
            <InfoCard title="Sections">
                <div class="overflow-x-auto">
                    <table class="w-full text-sm">
                        <thead class="border-b border-zinc-800">
                            <tr class="text-left text-zinc-400">
                                <th class="px-2 py-2">{"Name"}</th>
                                <th class="px-2 py-2">{"Virtual Address"}</th>
                                <th class="px-2 py-2">{"Virtual Size"}</th>
                                <th class="px-2 py-2">{"Raw Size"}</th>
                                <th class="px-2 py-2">{"Raw Address"}</th>
                                <th class="px-2 py-2">{"Characteristics"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for parsed.sections.iter().map(|section| {
                                let chars_flags = section_characteristics_to_strings(section.characteristics);
                                let chars_str = chars_flags.join(", ");
                                html! {
                                    <tr data-section={section.name.clone()} class="border-b border-zinc-800/50 hover:bg-zinc-800/30">
                                        <td class="px-2 py-2 font-mono text-zinc-300">{&section.name}</td>
                                        <td class="px-2 py-2 font-mono text-zinc-300">{format!("0x{:X}", section.virtual_address)}</td>
                                        <td class="px-2 py-2 font-mono text-zinc-300">{format!("0x{:X}", section.virtual_size)}</td>
                                        <td class="px-2 py-2 font-mono text-zinc-300">{format!("0x{:X}", section.size_of_raw_data)}</td>
                                        <td class="px-2 py-2 font-mono text-zinc-300">{format!("0x{:X}", section.pointer_to_raw_data)}</td>
                                        <td class="px-2 py-2 font-mono text-zinc-300 text-xs">
                                            {chars_str}
                                            <span class="text-zinc-500 ml-1">{format!("(0x{:X})", section.characteristics)}</span>
                                        </td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </div>
            </InfoCard>
        
            <InfoCard title="File Header">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <InfoRow label="Machine" value={format!("{} (0x{:04X})", machine_to_string(parsed.file_header.machine), parsed.file_header.machine)} />
                    <InfoRow label="Number of Sections" value={parsed.file_header.number_of_sections.to_string()} />
                    <InfoRow label="Timestamp" value={format_unix_timestamp(parsed.file_header.time_date_stamp)} />
                    <InfoRow label="Pointer to Symbol Table" value={format!("0x{:X}", parsed.file_header.pointer_to_symbol_table)} />
                    <InfoRow label="Number of Symbols" value={parsed.file_header.number_of_symbols.to_string()} />
                    <InfoRow label="Size of Optional Header" value={format!("{} bytes", parsed.file_header.size_of_optional_header)} />
                    <div class="col-span-2">
                        <div class="flex flex-col gap-1">
                            <div class="text-xs text-zinc-500 uppercase tracking-wide">{"Characteristics"}</div>
                            <div class="flex flex-wrap gap-1">
                                {for file_characteristics_to_strings(parsed.file_header.characteristics).iter().map(|flag| {
                                    html! {
                                        <span class="text-xs px-2 py-0.5 bg-zinc-800 rounded-md text-zinc-300">{flag}</span>
                                    }
                                })}
                            </div>
                            <div class="text-xs text-zinc-500 mt-1">{format!("0x{:04X}", parsed.file_header.characteristics)}</div>
                        </div>
                    </div>
                </div>
            </InfoCard>
            
            <InfoCard title="Optional Header">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <InfoRow label="Magic" value={format!("0x{:04X}", parsed.optional_header.magic)} />
                    <InfoRow label="Linker Version" value={format!("{}.{}", parsed.optional_header.major_linker_version, parsed.optional_header.minor_linker_version)} />
                    <InfoRow label="Size of Code" value={format!("0x{:X}", parsed.optional_header.size_of_code)} />
                    <InfoRow label="Size of Initialized Data" value={format!("0x{:X}", parsed.optional_header.size_of_initialized_data)} />
                    <InfoRow label="Size of Uninitialized Data" value={format!("0x{:X}", parsed.optional_header.size_of_uninitialized_data)} />
                    <InfoRow label="Address of Entry Point" value={format!("0x{:X}", parsed.optional_header.address_of_entry_point)} />
                    <InfoRow label="Base of Code" value={format!("0x{:X}", parsed.optional_header.base_of_code)} />
                    <InfoRow label="Image Base" value={format!("0x{:X}", parsed.optional_header.image_base)} />
                    <InfoRow label="Section Alignment" value={format!("0x{:X}", parsed.optional_header.section_alignment)} />
                    <InfoRow label="File Alignment" value={format!("0x{:X}", parsed.optional_header.file_alignment)} />
                    <InfoRow label="OS Version" value={format!("{}.{}", parsed.optional_header.major_os_version, parsed.optional_header.minor_os_version)} />
                    <InfoRow label="Image Version" value={format!("{}.{}", parsed.optional_header.major_image_version, parsed.optional_header.minor_image_version)} />
                    <InfoRow label="Subsystem Version" value={format!("{}.{}", parsed.optional_header.major_subsystem_version, parsed.optional_header.minor_subsystem_version)} />
                    <InfoRow label="Win32 Version" value={parsed.optional_header.win32_version_value.to_string()} />
                    <InfoRow label="Size of Image" value={format!("0x{:X}", parsed.optional_header.size_of_image)} />
                    <InfoRow label="Size of Headers" value={format!("0x{:X}", parsed.optional_header.size_of_headers)} />
                    <InfoRow label="Checksum" value={format!("0x{:X}", parsed.optional_header.checksum)} />
                    <InfoRow label="Subsystem" value={format!("{} (0x{:04X})", subsystem_to_string(parsed.optional_header.subsystem), parsed.optional_header.subsystem)} />
                    <InfoRow label="Size of Stack Reserve" value={format!("0x{:X}", parsed.optional_header.size_of_stack_reserve)} />
                    <InfoRow label="Size of Stack Commit" value={format!("0x{:X}", parsed.optional_header.size_of_stack_commit)} />
                    <InfoRow label="Size of Heap Reserve" value={format!("0x{:X}", parsed.optional_header.size_of_heap_reserve)} />
                    <InfoRow label="Size of Heap Commit" value={format!("0x{:X}", parsed.optional_header.size_of_heap_commit)} />
                    <InfoRow label="Loader Flags" value={parsed.optional_header.loader_flags.to_string()} />
                    <InfoRow label="Number of RVA and Sizes" value={parsed.optional_header.number_of_rva_and_sizes.to_string()} />
                    <div class="col-span-2">
                        <div class="flex flex-col gap-1">
                            <div class="text-xs text-zinc-500 uppercase tracking-wide">{"DLL Characteristics"}</div>
                            <div class="flex flex-wrap gap-1">
                                {for dll_characteristics_to_strings(parsed.optional_header.dll_characteristics).iter().map(|flag| {
                                    html! {
                                        <span class="text-xs px-2 py-0.5 bg-zinc-800 rounded-md text-zinc-300">{flag}</span>
                                    }
                                })}
                            </div>
                            <div class="text-xs text-zinc-500 mt-1">{format!("0x{:04X}", parsed.optional_header.dll_characteristics)}</div>
                        </div>
                    </div>
                </div>
            </InfoCard>
    
        </div>
    }
}