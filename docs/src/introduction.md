# Welcome

This is a growing set of exercises in which simple Rust programs are compiled into binaries and analyzed step by step with reverse engineering tools.

The goal is to start from the simplest possible binaries (like a main() that does nothing) and gradually move toward more complex Rust applications that showcase features such as:

- Basic console I/O
- Control flow (loops, branching, pattern matching)
- Functions and modules
- Error handling
- Structs, enums, traits
- Heap allocations and dynamic memory
- Generics and lifetimes
- Concurrency and async Rust
- Integration with external libraries

The examples in this book are tool-agnostic. You can explore them with whatever reverse engineering workflow you prefer. 

Personally, I’ll be experimenting with RetDec, Ghidra, IDA Pro, and x64dbg, but readers are encouraged to use their own favorite tools, whether that’s Radare2, Binary Ninja, Hopper, or anything else.