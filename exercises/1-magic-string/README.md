# Rust Binary Analysis and Runtime Insights

This document details findings related to the Rust runtime, memory management, and function behavior based on disassembly and investigation.

## 1. **Rust Runtime Entry Points**
### `std::rt::lang_start`
- `lang_start` is responsible for initializing the Rust runtime before invoking `main()`.
- It sets up essential components like stack overflow handlers, signal handlers, and thread-local storage (TLS).

### `std::rt::lang_start_internal`
- This function contains lower-level initialization, including:
  - Adding a vectored exception handler for stack overflow detection.
  - Allocating thread-local storage.
  - Handling runtime state transitions.

## 2. String Comparison Mechanism in Disassembled Code

The function: `alloc::string::impl$83::eq(&local_b8, &local_c8);`

performs a string comparison through the following steps:

1. Calls `alloc::string::impl$26::index<>`, which retrieves a substring or reference.
2. Passes the result (`local_68`) into `core::slice::cmp::impl$5::equal<u8,u8>`.
3. `equal<u8,u8>` internally calls `memcmp` to compare the two memory regions.

## 3. Patch Solution: Forcing an Unconditional Jump

### Original Instruction

In the original disassembly, the instruction at `0x140001802` was: `75 14 ; JNZ 0x14 (jump if not zero)`

Modified `75 14` to: `EB 14 ; JMP 0x14`


