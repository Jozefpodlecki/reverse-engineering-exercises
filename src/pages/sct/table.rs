use std::collections::HashSet;

use yew::prelude::*;
use crate::pages::sct::types::WindowsVersion;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected: HashSet<WindowsVersion>,
}

#[function_component(SyscallTable)]
pub fn syscall_table(props: &Props) -> Html {

    html! {

    }
}
//     let all_versions = WindowsVersion::all();
    
//     let show_version = |version: &WindowsVersion| -> bool {
//         props.selected.contains(version)
//     };

//     let version_headers: Vec<Html> = all_versions
//         .iter()
//         .filter(|v| show_version(v))
//         .map(|v| {
//             html! { <th class="pb-3 text-zinc-400 font-medium">{ v.name() }</th> }
//         })
//         .collect();

//     html! {
//         <div class="bg-zinc-900/20 border border-zinc-800 rounded-xl p-6">
//             <div class="overflow-x-auto">
//                 <table class="w-full text-sm">
//                     <thead class="border-b border-zinc-800">
//                         <tr class="text-left">
//                             <th class="pb-3 text-zinc-400 font-medium">{"Function Name"}</th>
//                             { for version_headers }
//                         </tr>
//                     </thead>
//                     <tbody class="divide-y divide-zinc-800/50">
//                         // { syscalls.iter().map(|(num, name)| {
//                         //     let version_cells: Vec<Html> = all_versions
//                         //         .iter()
//                         //         .filter(|v| show_version(v))
//                         //         .map(|_| {
//                         //             html! { <td class="py-3 font-mono text-xs">{ format!("0x{:02X}", num) }</td> }
//                         //         })
//                         //         .collect();
                            
//                         //     html! {
//                         //         <tr class="text-zinc-300">
//                         //             <td class="py-3 font-mono text-xs">{ name }</td>
//                         //             { for version_cells }
//                         //         </tr>
//                         //     }
//                         // }).collect::<Html>() }
//                     </tbody>
//                 </table>
//             </div>
           
//         </div>
//     }
// }