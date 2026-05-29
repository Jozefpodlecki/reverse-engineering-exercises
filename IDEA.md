# reverse-engineering-exercises

The project is `reverse-engineering-exercises`.

It is a serverless Yew application compiled to WebAssembly and intended to be deployed to GitHub Pages (`gh-pages`). The project is a browser-native reverse engineering learning environment with integrated tooling and interactive visualization.

The application currently prefers dark mode and is designed around a technical/laboratory-style interface rather than a traditional documentation or blog layout.

---

# Current Frontend Runtime State

The application is a Rust-based Yew frontend compiled to WebAssembly and built using Trunk.

The entry HTML acts primarily as a bootstrapping shell for the WASM application. Styling and assets are managed through Trunk directives. Tailwind-generated CSS is used globally, with additional custom fonts loaded from Google Fonts (`Oswald` and `Roboto`).

The document contains a full-screen loading animation displayed until the Trunk application startup event is triggered.

Theme initialization logic executes during page load using:
- `localStorage`
- browser color scheme preferences

The structure is intentionally static at the HTML level. All interactive UI is rendered dynamically by the Yew application once initialized.

---

# Application Bootstrap

The application entrypoint is a Yew WebAssembly client initialized through a custom `main.rs` bootstrap layer.

The project is currently modularized into:
- `app`
- `services`
- `components`
- `models`
- `pages`
- `route`

This implies a layered architecture separating:
- UI
- business logic
- routing
- shared models

Logging is configured through `wasm_logger`.

Log verbosity is determined dynamically through:
- compile-time build mode
- persisted `RUST_LOG` value in browser localStorage

The runtime environment binds directly to browser APIs through `web_sys`.

The following browser interfaces are currently treated as required startup dependencies:
- `window`
- `document`
- `body`
- `localStorage`
- `navigator`

Application metadata such as:
- package name
- package version

are injected at compile time and passed into the root component through `AppProps`.

The Yew application is mounted directly onto the document body using a root-bound renderer.

Startup is intentionally fail-fast if critical browser interfaces are unavailable.

---

# Current UX Direction

The user is first welcomed by a landing page where modules can be selected.

The project is currently intended to be organized primarily by operating system:
- Windows
- Linux
- macOS

Current focus is Windows-only.

Additional operating system modules may be added later if the project gains traction and the architecture matures properly.

The application should behave more like:
- an interactive reverse engineering laboratory
- an experimentation environment
- an educational sandbox

rather than a static exercise repository.

---

# Current Conceptual Direction

The project is converging toward:

> an interactive reverse engineering learning environment with integrated tooling

Exercises are intended to attract users initially.

Integrated tooling is intended to retain users long-term.

The project should combine:
- educational modules
- visualization systems
- interactive tooling
- execution simulation

rather than only presenting static exercises.

---

# Planned Modules

## Windows

Planned conceptual structure:

```text
Windows
 ├── PE Executables
 ├── Win32 Internals
 ├── x86 Assembly
 ├── x64 Assembly
 ├── Stack & Memory
 └── Exploitation Basics
```

This structure is intended to represent conceptual domains rather than simple page navigation.

---

# PE Executables Module

The PE module is currently the primary planned feature set.

Planned functionality:

- inspect existing PE executables
- drag-and-drop PE analysis
- file-input PE loading
- manually construct PE executables
- edit PE headers through structured forms
- manipulate:
  - DOS header
  - file header
  - optional header
  - sections
  - imports
  - alignment
  - RVA structures

Potential future additions:
- section visualizer
- RVA ↔ file offset translator
- import table explorer
- entropy visualization
- malformed/corrupted PE exercises

The PE module is intended to combine:
- education
- visualization
- tooling
- experimentation

---

# Emulator / Execution Visualizer Concept

A future planned feature is a lightweight educational execution environment.

The intent is not to build a full emulator initially.

The primary goal is:
- execution visualization
- register tracking
- stack visualization
- memory mutation inspection
- stepping through instructions interactively

Initial implementation should likely support only a constrained subset of x86 instructions.

Potential initial instruction subset:

```text
mov
push
pop
add
sub
cmp
jmp
call
ret
```

The execution environment should prioritize:
- educational clarity
- state visibility
- interaction simplicity

over perfect CPU accuracy.

Potential UI components:
- instruction viewer
- register state panel
- stack viewer
- memory dump widget
- flags display
- step execution controls

---

# Architectural Notes

The project is conceptually module-driven rather than page-driven.

Long-term architecture may eventually separate:
- educational modules
- interactive tools
- visualization systems
- execution engines

Current routing/page structure is acceptable during early development but may eventually evolve into domain-oriented module organization.

Potential future direction:

```text
modules/windows/pe/
modules/windows/x86/
modules/windows/memory/
```

instead of purely route-oriented page files.

---

# Current Project Structure

```text
src/

components/
models/
pages/
services/

app.rs
main.rs
route.rs
```

Current structure is intentionally lightweight while the core architecture is still evolving.