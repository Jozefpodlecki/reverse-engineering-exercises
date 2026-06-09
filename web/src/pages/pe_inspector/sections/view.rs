use yew::prelude::*;
use yew_icons::{Icon, IconData};
use prometheus_disassembler::{isa::{Immediate, Operand}, iter::InstructionIterator, *};
use prometheus_disassembler::isa::Mnemonic;
use crate::pages::pe_inspector::{RcBytes, sections::{hex_viewer::HexViewer, topbar::ViewMode}};

use super::SectionSelection;

#[derive(Properties, PartialEq)]
pub struct SectionViewProps {
    pub selection: SectionSelection,
    pub view_mode: ViewMode,
    pub raw_bytes: RcBytes,
}

#[function_component(SectionView)]
pub fn section_view(props: &SectionViewProps) -> Html {
    let start = props.selection.raw_address as usize;
    let end = start + props.selection.raw_size as usize;
    
    let data = if start < props.raw_bytes.len() && end <= props.raw_bytes.len() {
        props.raw_bytes[start..end].to_vec()
    } else {
        vec![]
    };
    
    if data.is_empty() {
        return html! {
            <div class="flex-1 flex items-center justify-center text-red-400">
                {"Failed to read section data"}
            </div>
        };
    }
    
    let content = match props.view_mode {
        ViewMode::Hex => html! {
            <HexViewer 
                data={data} 
                virtual_address={props.selection.virtual_address} 
                rows_per_page={50}
            />
        },
        ViewMode::Disassembly => html! {
            <DisassemblyViewer 
                data={data} 
                virtual_address={props.selection.virtual_address as u64}
            />
        },
    };
    
    html! {
        <div class="flex-1 overflow-auto p-4">
            {content}
        </div>
    }
}

fn format_operand(operand: &Operand) -> String {
    match operand {
        Operand::Register { reg, access: _, visibility: _, opmask, zeroing } => {
            let reg_str = format!("{:?}", reg).to_lowercase();
            let mask = opmask.as_ref().map(|m| format!("{{{:?}}}", m).to_lowercase()).unwrap_or_default();
            let zero = if *zeroing { " {z}" } else { "" };
            format!("{}{}{}", mask, reg_str, zero)
        }
        Operand::Immediate { imm, visibility: _ } => {
            match imm {
                Immediate::U8(v) => format!("0x{:X}", v),
                Immediate::U16(v) => format!("0x{:X}", v),
                Immediate::U32(v) => format!("0x{:X}", v),
                Immediate::U64(v) => format!("0x{:X}", v),
                Immediate::I8(v) => format!("{}", v),
                Immediate::I16(v) => format!("{}", v),
                Immediate::I32(v) => format!("{}", v),
                Immediate::I64(v) => format!("{}", v),
            }
        }
        Operand::Memory { mem, access: _, visibility: _, opmask, zeroing } => {
            let mask = opmask.as_ref().map(|m| format!("{{{:?}}}", m).to_lowercase()).unwrap_or_default();
            let zero = if *zeroing { " {z}" } else { "" };
            
            let base = mem.base.as_ref().map(|r| format!("{:?}", r).to_lowercase()).unwrap_or_default();
            let index = mem.index.as_ref().map(|r| format!("{:?}", r).to_lowercase()).unwrap_or_default();
            let scale = if mem.scale > 1 { format!(" * {}", mem.scale) } else { String::new() };
            let displacement = if mem.displacement != 0 {
                format!("{:+}", mem.displacement)
            } else {
                String::new()
            };
            
            let addr = if !base.is_empty() && !index.is_empty() {
                format!("{}{}{}{}", base, if !displacement.is_empty() && displacement.starts_with('+') { "" } else { "+" }, index, scale)
            } else if !base.is_empty() {
                format!("{}{}", base, displacement)
            } else if !index.is_empty() {
                format!("{}{}{}", index, scale, displacement)
            } else {
                displacement
            };
            
            let size = match mem.size {
                1 => "byte",
                2 => "word",
                4 => "dword",
                8 => "qword",
                16 => "xmmword",
                32 => "ymmword",
                64 => "zmmword",
                _ => "unknown",
            };
            
            format!("{}{} ptr [{}]{}", mask, size, addr, zero)
        }
    }
}

fn format_mnemonic(mnemonic: &Mnemonic) -> String {
    let mnemonic_str = format!("{:?}", mnemonic);
    let mnemonic_lower = mnemonic_str.to_lowercase();
    
    mnemonic_lower
        .replace("auto(", "")
        .replace(")", "")
        .replace("\"", "")
}

#[derive(Properties, PartialEq)]
struct DisassemblyViewerProps {
    data: Vec<u8>,
    virtual_address: u64,
}

#[function_component(DisassemblyViewer)]
fn disassembly_viewer(props: &DisassemblyViewerProps) -> Html {
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
            <div class="border-b border-zinc-800 px-4 py-2 text-xs text-zinc-500 grid grid-cols-12 gap-2">
                <div class="col-span-2">{"Address"}</div>
                <div class="col-span-3">{"Bytes"}</div>
                <div class="col-span-2">{"Instruction"}</div>
                <div class="col-span-5">{"Operands"}</div>
            </div>
            <div class="divide-y divide-zinc-800 max-h-[calc(100vh-20rem)] overflow-y-auto">
                {for instructions.iter().map(|inst| {
                    let address = format!("0x{:X}", inst.address);
                    let bytes: String = inst.bytes.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let mnemonic = format_mnemonic(&inst.mnemonic);
                    let operands: String = inst.operands.iter()
                        .map(format_operand)
                        .collect::<Vec<_>>()
                        .join(", ");
                    
                    html! {
                        <div class="px-4 py-2 hover:bg-zinc-800/30 font-mono text-sm grid grid-cols-12 gap-2">
                            <div class="text-zinc-500 col-span-2">{address}</div>
                            <div class="text-zinc-400 font-mono col-span-3 text-xs break-all">{bytes}</div>
                            <div class="text-yellow-400 col-span-2">{mnemonic}</div>
                            <div class="text-zinc-300 col-span-5 break-all">{operands}</div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}