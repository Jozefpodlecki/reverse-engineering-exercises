# 1-no-std

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

```sh
cd target/release
peanalyse -i no-std.exe --disassemble-from-addr=0x140001000
0x140001000: sub        rsp, 0x28
0x140001004: mov        ecx, 0xfffffff5
0x140001009: call       qword ptr [rip + 0x1001]
0x14000100F: mov        qword ptr [rsp + 0x20], 0
0x140001018: lea        rdx, [rip + 0x1001]
0x14000101F: mov        rcx, rax
0x140001022: mov        r8d, 0xc
0x140001028: xor        r9d, r9d
0x14000102B: call       qword ptr [rip + 0xfcf]
0x140001031: xor        ecx, ecx
0x140001033: call       qword ptr [rip + 0xfcf]
0x140001039: int3
```

Reserve 0x28 bytes (40 bytes) on the stack for local variables / alignment.
0x140001009: call qword ptr [rip + 0x1001]

RIP-relative addressing:

Current RIP = 0x140001009 + instruction length (6 bytes typical for call) → actually, rip = 0x14000100F for displacement? Let’s calculate precisely.

Call target: [RIP + 0x1001] →

0x140001009 + 0x1001 = 0x14000200A


This is an import address table (IAT) entry, likely WriteConsoleA.