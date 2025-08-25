# 1-no-std

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/no-std/src/main.rs)

This exercise explores a minimal Windows executable written in Rust. The program is compiled without the standard library (#![no_std]) and does not link against the C runtime.

All functionality — writing to the console and exiting the process — is implemented manually using Windows API calls.

```sh
cd no-std
cargo build --release
```

```sh
peanalyse -i no-std.exe --entry
Architecture: Pe64
AddressOfEntryPoint (RVA): 0x00001000
ImageBase:                0x140000000
EntryPoint (VA):          0x140001000
```

The program’s entry point is at 0x140001000.

### Disassembly

```sh
cd target/release
peanalyse -i no-std.exe --disassemble-from-addr=0x140001000
0x140001000: sub        rsp, 0x28
0x140001004: mov        ecx, 0xfffffff5
0x140001009: call       qword ptr [rip + 0x1009] ; target = 0x140002018
0x14000100F: mov        qword ptr [rsp + 0x20], 0
0x140001018: lea        rdx, [rip + 0x1009]  ; target = 0x140002028
0x14000101F: mov        rcx, rax
0x140001022: mov        r8d, 0xc
0x140001028: xor        r9d, r9d
0x14000102B: call       qword ptr [rip + 0xfcf] ; target = 0x140002000
0x140001031: mov        ecx, 0x2710
0x140001036: call       qword ptr [rip + 0xfcc] ; target = 0x140002008
0x14000103C: xor        ecx, ecx
0x14000103E: call       qword ptr [rip + 0xfcc] ; target = 0x140002010
0x140001044: int3
```

```asm
0x140001000: sub        rsp, 0x28
```

Reserve 0x28 bytes (40 bytes) on the stack for local variables / alignment.

```
0x140001004: mov        ecx, 0xfffffff5
```

```rs
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
```

`0xfffffff5` is the signed 32-bit representation of 4294967285u32.

### RIP-relative addressing for calls

Calls use RIP-relative addressing to locate the thunk table:

```
0x140001009: call       qword ptr [rip + 0x1009]
```

Resolving:

```
0x140001009 + 6 + 0x1009 = 0x140002018
```

### Thunk table contents

```sh
peanalyse -i no-std.exe --read-addr=0x140002018
0x140002018 - 0x2228
```

Each thunk entry holds the RVA of the corresponding IAT slot:

### IAT entries patched by the loader

```sh
peanalyse -i no-std.exe --iat-entries
DLL: kernel32.dll
  WriteConsoleA @ RVA 0x00002238 VA 0x140002238 (hint 1296)
  Sleep @ RVA 0x00002248 VA 0x140002248 (hint 1192)
  ExitProcess @ RVA 0x00002250 VA 0x140002250 (hint 274)
  GetStdHandle @ RVA 0x00002228 VA 0x140002228 (hint 621)
```

```
Example flow: GetStdHandle
0x140001009: call qword ptr [rip + 0x1001]
   → resolves to 0x140002010 (thunk entry, fixed at compile time)
   → thunk entry contains pointer to 0x140002218 (true IAT slot)
   → loader patches [0x140002218] with &kernel32!GetStdHandle
   → final call jumps into kernel32.dll
```

Lastly

```
0x14000100F: mov        qword ptr [rsp + 0x20], 0
0x140001018: lea        rdx, [rip + 0x1009]  ; target = 0x140002028
0x14000101F: mov        rcx, rax
0x140001022: mov        r8d, 0xc
0x140001028: xor        r9d, r9d
0x14000102B: call       qword ptr [rip + 0xfcf] ; target = 0x140002000
```

Microsoft x64 calling convention.
rcx → 1st argument
rdx → 2nd argument
r8 → 3rd
r9 → 4th
Additional stack space (shadow space) is already reserved at [rsp..rsp+0x20].

It is the caller's responsibility to allocate 32 bytes of "shadow space" on the stack right before calling the function (regardless of the actual number of parameters used), and to pop the stack after the call.

### Resources

- [Calling convention](https://gist.github.com/rtldg/91dd76b65748540717ed6f88d95a41b1)