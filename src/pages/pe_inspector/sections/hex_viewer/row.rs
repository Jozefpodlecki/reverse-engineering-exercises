use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HexRowProps {
    pub row: Vec<u8>,
    pub offset: u64,
}

#[function_component(HexRow)]
pub fn hex_row(props: &HexRowProps) -> Html {
    let hex: String = props.row.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
    let hex_padding = " ".repeat((16 - props.row.len()) * 3);
    let ascii: String = props.row.iter().map(|b| {
        if *b >= 0x20 && *b <= 0x7E { *b as char } else { '.' }
    }).collect();
    
    html! {
        <div class="grid grid-cols-12 gap-2 hover:bg-zinc-800/30 px-2 py-0.5 rounded font-mono">
            <div class="text-zinc-500 col-span-2 select-none">{format!("0x{:X}", props.offset)}</div>
            <div class="text-zinc-300 font-mono col-span-7 break-all">{hex}{hex_padding}</div>
            <div class="text-zinc-400 col-span-3 font-mono">{ascii}</div>
        </div>
    }
}