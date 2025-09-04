[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/calling-conventions/src/main.rs)

peanalyse -i calling-conventions.exe --disassemble-from-addr=0x1400016B0

```
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
