[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/calling-conventions/src/main.rs)

```sh
peanalyse -i calling-conventions.exe --entry
Architecture: Pe64
AddressOfEntryPoint (RVA): 0x000168A0
ImageBase:                0x140000000
EntryPoint (VA):          0x1400168A0
```


```sh
peanalyse -i calling-conventions.exe --disassemble-from-addr=0x1400016B0
0x1400016EA: mov        dword ptr [rsp + 0x38], eax
0x1400016EE: mov        ecx, 1
0x1400016F3: mov        edx, 2
0x1400016F8: mov        r8d, 3
0x1400016FE: mov        r9d, 4
0x140001704: mov        dword ptr [rsp + 0x20], 5
0x14000170C: mov        dword ptr [rsp + 0x28], 6
0x140001714: mov        dword ptr [rsp + 0x30], 7
0x14000171C: call       0x1400012c0
```

Windows x64 calling convention:
- Registers: RCX, RDX, R8, R9 for the first 4 parameters.
- Stack: parameters 5, 6, 7 go on the stack at offsets from RSP.

```
0x14000188F: mov        dword ptr [rsp + 0x44], eax
0x140001893: mov        dword ptr [rsp + 0x48], 1
0x14000189B: mov        dword ptr [rsp + 0x4c], 2
0x1400018A3: mov        dword ptr [rsp + 0x50], 3
0x1400018AB: mov        dword ptr [rsp + 0x54], 4
0x1400018B3: mov        dword ptr [rsp + 0x58], 5
0x1400018BB: mov        dword ptr [rsp + 0x5c], 6
0x1400018C3: mov        dword ptr [rsp + 0x60], 7
0x1400018CB: mov        dword ptr [rsp + 0x64], 8
0x1400018D3: mov        rax, qword ptr [rsp + 0x48]
0x1400018D8: mov        qword ptr [rsp + 0x1a8], rax
0x1400018E0: mov        rax, qword ptr [rsp + 0x50]
0x1400018E5: mov        qword ptr [rsp + 0x1b0], rax
0x1400018ED: mov        rax, qword ptr [rsp + 0x58]
0x1400018F2: mov        qword ptr [rsp + 0x1b8], rax
0x1400018FA: mov        rax, qword ptr [rsp + 0x60]
0x1400018FF: mov        qword ptr [rsp + 0x1c0], rax
0x140001907: lea        rcx, [rsp + 0x1a8]
0x14000190F: call       0x1400011b0
```

On x86-64 Windows (MSVC calling convention):
- If the struct fits in 1–2 registers (≤16 bytes), it is passed in registers.
- If the struct is larger than 16 bytes, it is passed by reference: the caller allocates space on the stack and passes a pointer.
- The callee then reads fields via that pointer.
- `BigStruct` is 32 bytes, so rust will pass a pointer to struct rather than copying all fields into registers.
-- `lea rcx, [rsp + ...] or mov rcx, &struct_on_stack` before the call.