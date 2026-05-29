use prometheus_disassembler::{Architecture, InstructionIterator};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Vec<u8>,
    pub virtual_address: u64,
}

#[function_component(DisassemblyViewer)]
pub fn disassembly_viewer(props: &Props) -> Html {
    let iterator = InstructionIterator::new(Architecture::X64, &props.data, props.virtual_address);
    
    let instructions: Vec<_> = iterator
        .filter_map(|result| result.ok())
        .collect();
    
    if instructions.is_empty() {
        return html! {
            <div class="bg-zinc-950/50 rounded-lg p-4">
                <div class="text-center text-zinc-500 py-8">
                    <Icon data={IconData::LUCIDE_BUG} width="2rem" height="2rem" class="mx-auto mb-2 opacity-50" />
                    <p>{"No instructions decoded"}</p>
                    <p class="text-xs mt-2 text-zinc-600">{"Section may contain data, not code"}</p>
                </div>
            </div>
        };
    }
    
    html! {
        <div class="bg-zinc-950/50 rounded-lg font-mono text-sm">
            <div class="border-b border-zinc-800 px-4 py-2 text-xs text-zinc-500 flex gap-6">
                <span>{"Address"}</span>
                <span>{"Bytes"}</span>
                <span>{"Instruction"}</span>
            </div>
            <div class="divide-y divide-zinc-800">
                {for instructions.iter().map(|inst| {
                    let address = format!("0x{:X}", inst.address);
                    let bytes: String = inst.bytes.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let operands: String = inst.operands.iter()
                        .map(|op| format!("{:?}", op))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let mnemonic = format!("{:?}", inst.mnemonic);
                    
                    html! {
                        <div class="px-4 py-2 hover:bg-zinc-800/30 font-mono text-sm grid grid-cols-12 gap-2">
                            <div class="text-zinc-500 col-span-2">{address}</div>
                            <div class="text-zinc-400 font-mono col-span-3 text-xs break-all">{bytes}</div>
                            <div class="text-yellow-400 col-span-2">{mnemonic}</div>
                            <div class="text-zinc-300 col-span-5">{operands}</div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}