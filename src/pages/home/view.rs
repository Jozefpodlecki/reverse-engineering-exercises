use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

use crate::{pages::home::cards::*, route::Route};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">

            <main class="flex-1 flex flex-col items-center justify-center px-6">

                <div class="w-full max-w-6xl space-y-10">

                    <header class="text-center space-y-3">

                        <h1 class="text-4xl md:text-5xl font-bold tracking-tight">
                            { "Reverse Engineering Exercises" }
                        </h1>

                    </header>

                    <section class="grid gap-6 md:grid-cols-3 items-stretch">

                        <Card
                            route={Route::PeInspector}
                            icon_src="public/images/mag-glass.svg"
                            icon_alt="Magnifying glass icon"
                            title="PE Inspector"
                            description="Upload and analyze Windows Portable Executable files. Inspect headers, sections, imports, entry point, and raw binary layout."
                            tags={vec!["Windows", "PE format", "headers", "sections", "imports", "binary analysis"]}
                        />
                        <Card
                            route={Route::Emulator}
                            icon_src="public/images/emulator.svg"
                            icon_alt="emulator icon"
                            title="Emulator"
                            description="Interactive x86/x64 emulation sandbox. Step through instructions, inspect registers, memory, and stack."
                            tags={vec!["x86", "x64", "emulation", "registers", "step-by-step"]}
                        />
                        <Card
                            route={Route::Quiz}
                            icon_src="public/images/quiz.svg"
                            icon_alt="Quiz icon"
                            title="Quiz"
                            description="Test your Windows internals knowledge with multiple-choice questions covering ntoskrnl, memory management, IRQL, DPC, process/thread management, and more. Select your dataset and challenge yourself."
                            tags={vec!["ntoskrnl", "memory management", "IRQL", "DPC", "process/thread"]}
                        />
                        <Card
                            route={Route::Asm}
                            icon_src="public/images/code-desktop.svg"
                            icon_alt="Asm"
                            title="Asm Playground"
                            description="Interactive x64 assembly sandbox. Experiment with instructions like PUSH, POP, MOV, and more. Set initial register values, define memory content, and see how instructions behave."
                            tags={vec!["x64", "assembly", "registers", "stack", "interactive"]}
                        />
                        <Card
                            route={Route::PeBuilder}
                            icon_src="public/images/file-pencil.svg"
                            icon_alt="PE Builder icon"
                            title="PE Builder"
                            description="Build x64 PE files from scratch. Construct headers, sections (.text, .rdata), and eventually assemble executable content. A powerful tool for learning PE format internals."
                            tags={vec!["x64", "PE format", "headers", ".text", ".rdata", "executable"]}
                        />
                        <Card
                            route={Route::SystemCallTable}
                            icon_src="public/images/table.svg"
                            icon_alt="System calls"
                            title="System Call Table"
                            description="Comprehensive Windows system call reference across all versions (XP to 11, Server editions). Search by syscall number or function name. Currently supports Windows family — cross-OS comparison planned."
                            tags={vec!["syscalls", "Windows", "NT API", "SSDT", "reference"]}
                        />
                        <Card
                            route={Route::Wiki}
                            icon_src="public/images/wiki.svg"
                            icon_alt="Wiki icon"
                            title="Wiki"
                            description="Curated reference library covering Windows internals terminology, data structures, algorithms, driver development, debugging techniques, and low-level system concepts. Explanations and external references included."
                            tags={vec!["documentation", "references", "internals", "driver dev", "debugging"]}
                        />

                    </section>

                </div>

            </main>

            <footer class="py-3 px-6 border-t border-zinc-800">
                <div class="max-w-4xl mx-auto flex items-center justify-between text-sm text-zinc-500">
                    <div class="flex items-center gap-2">
                        <span>{ "©" }</span>
                        <span>{ "Jozef Podlecki" }</span>
                        <span>{ "2026" }</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <a 
                            href="https://github.com/Jozefpodlecki/reverse-engineering-exercises"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="p-2 text-zinc-400 hover:text-zinc-300 transition rounded-lg hover:bg-zinc-800/50 block group"
                        >
                            <Icon data={IconData::LUCIDE_GITHUB} width="1.25rem" height="1.25rem" />
                        </a>
                    </div>
                </div>
            </footer>

        </div>
    }
}