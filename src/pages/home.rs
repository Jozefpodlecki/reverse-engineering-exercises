use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

use yew::prelude::*;

#[function_component(PeInspectorCard)]
pub fn pe_inspector_card() -> Html {
    html! {
        <Link<Route> to={Route::PeInspector} classes="block group">

            <div class="border border-zinc-800 rounded-2xl p-6 bg-zinc-900/30 hover:bg-zinc-900/50 hover:border-cyan-500/40 transition space-y-4">

                <div class="flex items-center gap-4">

                    <img
                        src="public/images/mag-glass.svg"
                        class="w-10 h-10 opacity-80 group-hover:opacity-100 transition"
                    />

                    <div class="text-xl font-semibold">
                        { "PE Inspector" }
                    </div>

                </div>

                <p class="text-sm text-zinc-400 leading-relaxed">
                    { "Upload and analyze Windows Portable Executable files. Inspect headers, sections, imports, entry point, and raw binary layout." }
                </p>

                <div class="flex items-center justify-between text-xs text-zinc-500">

                    <span>
                        { "Windows / PE format" }
                    </span>

                </div>

            </div>

        </Link<Route>>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">

            <main class="flex-1 flex flex-col items-center justify-center px-6">

                <div class="w-full max-w-4xl space-y-10">

                    <header class="text-center space-y-3">

                        <h1 class="text-4xl md:text-5xl font-bold tracking-tight">
                            { "Reverse Engineering Exercises" }
                        </h1>

                    </header>

                    <section class="grid gap-6 md:grid-cols-2">

                        <PeInspectorCard />

                    </section>

                </div>

            </main>

        </div>
    }
}