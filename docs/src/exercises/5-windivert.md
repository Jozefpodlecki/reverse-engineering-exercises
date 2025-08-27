
This exercise explores how 

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/windivert/src/main.rs)

### Function Prologue

```x86asm
00007FF7C3EE8C70 | 55           | push rbp
...
00007FF7C3EE8C7C | 48:81EC B8020000 | sub rsp,2B8
```

Allocate 696 bytes on stack for
- Local variables (like buffer, windivert, tx, etc.)
- Rust runtime bookkeeping (Vec metadata, thread closure captures)

### Zeroing / Initializing memory

```
00007FF7C3EE8CC1 | 0F57C0        | xorps xmm0,xmm0
00007FF7C3EE8CC4 | 0F2983 80000000 | movaps xmmword ptr ds:[rbx+80],xmm0
00007FF7C3EE8CCB | 0F2983 00010000 | movaps xmmword ptr ds:[rbx+100],xmm0
00007FF7C3EE8CD2 | 66:C783 80010000 0000 | mov word ptr ds:[rbx+180],0
00007FF7C3EE8CDB | 48:C783 88010000 00000000 | mov qword ptr ds:[rbx+188],0
```

### Stack alignment & frame pointers

```
00007FF7C3EE8C83 | 48:8DAC24 80000000 | lea rbp,qword ptr ss:[rsp+80]
00007FF7C3EE8C8B | 48:83E4 80        | and rsp,FFFFFFFFFFFFFF80
00007FF7C3EE8C8F | 48:89E3           | mov rbx,rsp
```

### Memory allocation

```x86asm
00007FF7C3EE8D15 | 0FB605 25D40200 | movzx eax,byte ptr ds:[<__rust_no_alloc_shim_is_unstable>]
00007FF7C3EE8D1C | B9 00020000      | mov ecx,200
00007FF7C3EE8D21 | BA 80000000      | mov edx,80
00007FF7C3EE8D26 | E8 A5260000      | call <windivert.__rustc::__rust_alloc>
00007FF7C3EE8D2B | 48:85C0         | test rax,rax
```

Rust runtime is calling __rust_alloc to allocate heap memory:

rcx, rdx = allocation size and alignment.

rax = pointer to allocated memory.

test rax, rax checks if allocation failed (None if null).

### Copying strings / data

```x86asm
00007FF7C3EE8D37 | 48:8D93 80000000 | lea rdx,qword ptr ds:[rbx+80]
00007FF7C3EE8D44 | 48:89C1         | mov rcx,rax
00007FF7C3EE8D47 | E8 10BE0100     | call <windivert.memcpy>
```

### Channel setup and threads

```x86asm
00007FF7C3EE8D99 | 4C:8D43 68 | lea r8,qword ptr ds:[rbx+68]
00007FF7C3EE8D9D | E8 CE130000 | call <windivert.std::thread::Builder::spawn_unchecked>
```

This is spawning a thread in Rust.

Arguments: stack pointers, local buffer, and channel sender (tx) are passed as thread closure environment.

Rust runtime handles closure captures in memory and passes a pointer to spawn_unchecked.

```
00007FF7C3EE83E0 | 55                               | push rbp                                                                                     |
00007FF7C3EE83E1 | 41:57                            | push r15                                                                                     |
00007FF7C3EE83E3 | 41:56                            | push r14                                                                                     |
00007FF7C3EE83E5 | 41:55                            | push r13                                                                                     |
00007FF7C3EE83E7 | 41:54                            | push r12                                                                                     |
00007FF7C3EE83E9 | 56                               | push rsi                                                                                     |
00007FF7C3EE83EA | 57                               | push rdi                                                                                     |
00007FF7C3EE83EB | 53                               | push rbx                                                                                     |
00007FF7C3EE83EC | 48:81EC 28010000                 | sub rsp,128                                                                                  |
00007FF7C3EE83F3 | 48:8DAC24 80000000               | lea rbp,qword ptr ss:[rsp+80]                                                                |
00007FF7C3EE83FB | 48:C785 A0000000 FEFFFFFF        | mov qword ptr ss:[rbp+A0],FFFFFFFFFFFFFFFE                                                   |
00007FF7C3EE8406 | 48:894D 68                       | mov qword ptr ss:[rbp+68],rcx                                                                |
00007FF7C3EE840A | 48:8D41 10                       | lea rax,qword ptr ds:[rcx+10]                                                                | rax:&"tcp.SrcPort == "
00007FF7C3EE840E | 48:8945 40                       | mov qword ptr ss:[rbp+40],rax                                                                |
00007FF7C3EE8412 | 48:8D05 77B60100                 | lea rax,qword ptr ds:[<core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32 | rax:&"tcp.SrcPort == "
00007FF7C3EE8419 | 48:8945 48                       | mov qword ptr ss:[rbp+48],rax                                                                | [rbp+48]:core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt::h2816672a2b9a7a5a
00007FF7C3EE841D | 48:8D05 84FD0100                 | lea rax,qword ptr ds:[7FF7C3F081A8]                                                          | rax:&"tcp.SrcPort == ", 00007FF7C3F081A8:&"tcp.SrcPort == "
00007FF7C3EE8424 | 48:8945 C0                       | mov qword ptr ss:[rbp-40],rax                                                                | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE8428 | 48:C745 C8 01000000              | mov qword ptr ss:[rbp-38],1                                                                  |
00007FF7C3EE8430 | 48:C745 E0 00000000              | mov qword ptr ss:[rbp-20],0                                                                  |
00007FF7C3EE8438 | 48:8D45 40                       | lea rax,qword ptr ss:[rbp+40]                                                                |
00007FF7C3EE843C | 48:8945 D0                       | mov qword ptr ss:[rbp-30],rax                                                                |
00007FF7C3EE8440 | 48:C745 D8 01000000              | mov qword ptr ss:[rbp-28],1                                                                  |
00007FF7C3EE8448 | 48:8D8D 80000000                 | lea rcx,qword ptr ss:[rbp+80]                                                                |
00007FF7C3EE844F | 48:8D55 C0                       | lea rdx,qword ptr ss:[rbp-40]                                                                | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE8453 | E8 886F0100                      | call <windivert.alloc::fmt::format::format_inner::hfc6b6ff323357fe5>                         |
00007FF7C3EE8458 | 0F1085 80000000                  | movups xmm0,xmmword ptr ss:[rbp+80]                                                          |
00007FF7C3EE845F | 0F2945 40                        | movaps xmmword ptr ss:[rbp+40],xmm0                                                          |
00007FF7C3EE8463 | 48:8B85 90000000                 | mov rax,qword ptr ss:[rbp+90]                                                                |
00007FF7C3EE846A | 48:8945 50                       | mov qword ptr ss:[rbp+50],rax                                                                |
00007FF7C3EE846E | 48:8D8D 80000000                 | lea rcx,qword ptr ss:[rbp+80]                                                                |
00007FF7C3EE8475 | 48:8D55 40                       | lea rdx,qword ptr ss:[rbp+40]                                                                |
00007FF7C3EE8479 | 41:B9 05000000                   | mov r9d,5                                                                                    |
00007FF7C3EE847F | 45:31C0                          | xor r8d,r8d                                                                                  |
00007FF7C3EE8482 | E8 391B0000                      | call <windivert.windivert::divert::WinDivert$LT$windivert..layer..NetworkLayer$GT$::network: |
00007FF7C3EE8487 | 48:B8 0700000000000080           | mov rax,8000000000000007                                                                     | rax:&"tcp.SrcPort == "
00007FF7C3EE8491 | 48:3985 80000000                 | cmp qword ptr ss:[rbp+80],rax                                                                |
00007FF7C3EE8498 | 0F85 A5020000                    | jne windivert.7FF7C3EE8743                                                                   |
00007FF7C3EE849E | 48:8B85 88000000                 | mov rax,qword ptr ss:[rbp+88]                                                                |
00007FF7C3EE84A5 | 8B8D 90000000                    | mov ecx,dword ptr ss:[rbp+90]                                                                |
00007FF7C3EE84AB | 48:8945 B0                       | mov qword ptr ss:[rbp-50],rax                                                                |
00007FF7C3EE84AF | 894D B8                          | mov dword ptr ss:[rbp-48],ecx                                                                |
00007FF7C3EE84B2 | 0FB605 88DC0200                  | movzx eax,byte ptr ds:[<__rust_no_alloc_shim_is_unstable>]                                   |
00007FF7C3EE84B9 | B9 FFFF0000                      | mov ecx,FFFF                                                                                 |
00007FF7C3EE84BE | BA 01000000                      | mov edx,1                                                                                    |
00007FF7C3EE84C3 | E8 382F0000                      | call <windivert.__rustc::__rust_alloc_zeroed>                                                |
00007FF7C3EE84C8 | 48:8945 70                       | mov qword ptr ss:[rbp+70],rax                                                                |
00007FF7C3EE84CC | 48:85C0                          | test rax,rax                                                                                 | rax:&"tcp.SrcPort == "
00007FF7C3EE84CF | 75 1B                            | jne windivert.7FF7C3EE84EC                                                                   |
00007FF7C3EE84D1 | 4C:8D05 08FD0100                 | lea r8,qword ptr ds:[7FF7C3F081E0]                                                           | 00007FF7C3F081E0:&"src\\main.rs"
00007FF7C3EE84D8 | B9 01000000                      | mov ecx,1                                                                                    |
00007FF7C3EE84DD | BA FFFF0000                      | mov edx,FFFF                                                                                 |
00007FF7C3EE84E2 | E8 6CDB0100                      | call <windivert.alloc::raw_vec::handle_error::h5d55154af761dff4>                             |
00007FF7C3EE84E7 | E9 B1020000                      | jmp windivert.7FF7C3EE879D                                                                   |
00007FF7C3EE84EC | 48:8D75 C0                       | lea rsi,qword ptr ss:[rbp-40]                                                                | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE84F0 | 48:8D7D B0                       | lea rdi,qword ptr ss:[rbp-50]                                                                |
00007FF7C3EE84F4 | 49:BD 0100000000000080           | mov r13,8000000000000001                                                                     |
00007FF7C3EE84FE | 48:8D9D 80000000                 | lea rbx,qword ptr ss:[rbp+80]                                                                |
00007FF7C3EE8505 | 41:BC 01000000                   | mov r12d,1                                                                                   |
00007FF7C3EE850B | EB 14                            | jmp windivert.7FF7C3EE8521                                                                   |
00007FF7C3EE850D | 0F1F00                           | nop dword ptr ds:[rax],eax                                                                   |
00007FF7C3EE8510 | 48:8D0455 00000000               | lea rax,qword ptr ds:[rdx*2]                                                                 | rax:&"tcp.SrcPort == "
00007FF7C3EE8518 | 48:85C0                          | test rax,rax                                                                                 | rax:&"tcp.SrcPort == "
00007FF7C3EE851B | 0F85 5F010000                    | jne windivert.7FF7C3EE8680                                                                   |
00007FF7C3EE8521 | 41:B9 FFFF0000                   | mov r9d,FFFF                                                                                 |
00007FF7C3EE8527 | 48:89F1                          | mov rcx,rsi                                                                                  |
00007FF7C3EE852A | 48:89FA                          | mov rdx,rdi                                                                                  |
00007FF7C3EE852D | 4C:8B45 70                       | mov r8,qword ptr ss:[rbp+70]                                                                 |
00007FF7C3EE8531 | E8 EA2E0000                      | call <windivert.windivert::divert::blocking::_$LT$impl$u20$windivert..divert..WinDivert$LT$w |
00007FF7C3EE8536 | 48:8B45 C0                       | mov rax,qword ptr ss:[rbp-40]                                                                | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE853A | 4C:39E8                          | cmp rax,r13                                                                                  | rax:&"tcp.SrcPort == "
00007FF7C3EE853D | 0F84 51010000                    | je windivert.7FF7C3EE8694                                                                    |
00007FF7C3EE8543 | 48:8945 60                       | mov qword ptr ss:[rbp+60],rax                                                                |
00007FF7C3EE8547 | 48:8B45 C8                       | mov rax,qword ptr ss:[rbp-38]                                                                |
00007FF7C3EE854B | 48:8945 78                       | mov qword ptr ss:[rbp+78],rax                                                                |
00007FF7C3EE854F | 4C:8B7D D0                       | mov r15,qword ptr ss:[rbp-30]                                                                |
00007FF7C3EE8553 | 4D:85FF                          | test r15,r15                                                                                 |
00007FF7C3EE8556 | 0F88 27020000                    | js windivert.7FF7C3EE8783                                                                    |
00007FF7C3EE855C | 74 22                            | je windivert.7FF7C3EE8580                                                                    |
00007FF7C3EE855E | 0FB605 DCDB0200                  | movzx eax,byte ptr ds:[<__rust_no_alloc_shim_is_unstable>]                                   |
00007FF7C3EE8565 | BA 01000000                      | mov edx,1                                                                                    |
00007FF7C3EE856A | 4C:89F9                          | mov rcx,r15                                                                                  |
00007FF7C3EE856D | E8 5E2E0000                      | call <windivert.__rustc::__rust_alloc>                                                       |
00007FF7C3EE8572 | 48:85C0                          | test rax,rax                                                                                 | rax:&"tcp.SrcPort == "
00007FF7C3EE8575 | 0F84 0D020000                    | je windivert.7FF7C3EE8788                                                                    |
00007FF7C3EE857B | 49:89C6                          | mov r14,rax                                                                                  | rax:&"tcp.SrcPort == "
00007FF7C3EE857E | EB 06                            | jmp windivert.7FF7C3EE8586                                                                   |
00007FF7C3EE8580 | 41:BE 01000000                   | mov r14d,1                                                                                   |
00007FF7C3EE8586 | 4C:89F1                          | mov rcx,r14                                                                                  |
00007FF7C3EE8589 | 48:8B55 78                       | mov rdx,qword ptr ss:[rbp+78]                                                                |
00007FF7C3EE858D | 4D:89F8                          | mov r8,r15                                                                                   |
00007FF7C3EE8590 | E8 C7C50100                      | call <windivert.memcpy>                                                                      |
00007FF7C3EE8595 | 48:8B4D 68                       | mov rcx,qword ptr ss:[rbp+68]                                                                |
00007FF7C3EE8599 | 48:8B01                          | mov rax,qword ptr ds:[rcx]                                                                   | rax:&"tcp.SrcPort == "
00007FF7C3EE859C | 48:8B51 08                       | mov rdx,qword ptr ds:[rcx+8]                                                                 |
00007FF7C3EE85A0 | 48:83F8 02                       | cmp rax,2                                                                                    | rax:&"tcp.SrcPort == "
00007FF7C3EE85A4 | 74 3A                            | je windivert.7FF7C3EE85E0                                                                    |
00007FF7C3EE85A6 | 83F8 01                          | cmp eax,1                                                                                    |
00007FF7C3EE85A9 | 75 65                            | jne windivert.7FF7C3EE8610                                                                   |
00007FF7C3EE85AB | 4C:89BD 80000000                 | mov qword ptr ss:[rbp+80],r15                                                                |
00007FF7C3EE85B2 | 4C:89B5 88000000                 | mov qword ptr ss:[rbp+88],r14                                                                |
00007FF7C3EE85B9 | 4C:89BD 90000000                 | mov qword ptr ss:[rbp+90],r15                                                                |
00007FF7C3EE85C0 | C74424 20 00CA9A3B               | mov dword ptr ss:[rsp+20],3B9ACA00                                                           |
00007FF7C3EE85C8 | 48:89F1                          | mov rcx,rsi                                                                                  |
00007FF7C3EE85CB | 49:89D8                          | mov r8,rbx                                                                                   |
00007FF7C3EE85CE | E8 2D9DFFFF                      | call <windivert.std::sync::mpmc::list::Channel$LT$T$GT$::send::hccb9013442662364>            |
00007FF7C3EE85D3 | EB 63                            | jmp windivert.7FF7C3EE8638                                                                   |
00007FF7C3EE85D5 | 66662E:0F1F8400 00000000         | nop word ptr ds:[rax+rax],ax                                                                 |
00007FF7C3EE85E0 | 4C:89BD 80000000                 | mov qword ptr ss:[rbp+80],r15                                                                |
00007FF7C3EE85E7 | 4C:89B5 88000000                 | mov qword ptr ss:[rbp+88],r14                                                                |
00007FF7C3EE85EE | 4C:89BD 90000000                 | mov qword ptr ss:[rbp+90],r15                                                                |
00007FF7C3EE85F5 | C74424 20 00CA9A3B               | mov dword ptr ss:[rsp+20],3B9ACA00                                                           |
00007FF7C3EE85FD | 48:89F1                          | mov rcx,rsi                                                                                  |
00007FF7C3EE8600 | 49:89D8                          | mov r8,rbx                                                                                   |
00007FF7C3EE8603 | E8 28D3FFFF                      | call <windivert.std::sync::mpmc::zero::Channel$LT$T$GT$::send::h6f8bab36c27d44da>            |
00007FF7C3EE8608 | EB 2E                            | jmp windivert.7FF7C3EE8638                                                                   |
00007FF7C3EE860A | 66:0F1F4400 00                   | nop word ptr ds:[rax+rax],ax                                                                 |
00007FF7C3EE8610 | 4C:89BD 80000000                 | mov qword ptr ss:[rbp+80],r15                                                                |
00007FF7C3EE8617 | 4C:89B5 88000000                 | mov qword ptr ss:[rbp+88],r14                                                                |
00007FF7C3EE861E | 4C:89BD 90000000                 | mov qword ptr ss:[rbp+90],r15                                                                |
00007FF7C3EE8625 | C74424 20 00CA9A3B               | mov dword ptr ss:[rsp+20],3B9ACA00                                                           |
00007FF7C3EE862D | 48:89F1                          | mov rcx,rsi                                                                                  |
00007FF7C3EE8630 | 49:89D8                          | mov r8,rbx                                                                                   |
00007FF7C3EE8633 | E8 C8ADFFFF                      | call <windivert.std::sync::mpmc::array::Channel$LT$T$GT$::send::h3f0d389a1efe5b98>           |
00007FF7C3EE8638 | 48:8B45 C0                       | mov rax,qword ptr ss:[rbp-40]                                                                | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE863C | 48:83F8 02                       | cmp rax,2                                                                                    | rax:&"tcp.SrcPort == "
00007FF7C3EE8640 | 48:8B55 60                       | mov rdx,qword ptr ss:[rbp+60]                                                                |
00007FF7C3EE8644 | 0F84 C6FEFFFF                    | je windivert.7FF7C3EE8510                                                                    |
00007FF7C3EE864A | 48:8B4D C8                       | mov rcx,qword ptr ss:[rbp-38]                                                                |
00007FF7C3EE864E | A8 01                            | test al,1                                                                                    |
00007FF7C3EE8650 | 0F84 C3000000                    | je windivert.7FF7C3EE8719                                                                    |
00007FF7C3EE8656 | 48:8D45 C8                       | lea rax,qword ptr ss:[rbp-38]                                                                |
00007FF7C3EE865A | 0F1040 08                        | movups xmm0,xmmword ptr ds:[rax+8]                                                           | rax+08:anon.95dd94260625f633aaffbc1dd64e9bca.29.llvm.18142701166963205077+3F8
00007FF7C3EE865E | 0F2985 80000000                  | movaps xmmword ptr ss:[rbp+80],xmm0                                                          |
00007FF7C3EE8665 | 48:89C8                          | mov rax,rcx                                                                                  | rax:&"tcp.SrcPort == "
00007FF7C3EE8668 | 48:F7D8                          | neg rax                                                                                      | rax:&"tcp.SrcPort == "
00007FF7C3EE866B | 0F80 9FFEFFFF                    | jo windivert.7FF7C3EE8510                                                                    |
00007FF7C3EE8671 | EB 6A                            | jmp windivert.7FF7C3EE86DD                                                                   |
00007FF7C3EE8673 | 666666662E:0F1F8400 00000000     | nop word ptr ds:[rax+rax],ax                                                                 |
00007FF7C3EE8680 | 41:B8 01000000                   | mov r8d,1                                                                                    |
00007FF7C3EE8686 | 48:8B4D 78                       | mov rcx,qword ptr ss:[rbp+78]                                                                |
00007FF7C3EE868A | E8 512D0000                      | call <windivert.__rustc::__rust_dealloc>                                                     |
00007FF7C3EE868F | E9 8DFEFFFF                      | jmp windivert.7FF7C3EE8521                                                                   |
00007FF7C3EE8694 | 48:8D45 C8                       | lea rax,qword ptr ss:[rbp-38]                                                                |
00007FF7C3EE8698 | 0F1000                           | movups xmm0,xmmword ptr ds:[rax]                                                             | rax:&"tcp.SrcPort == "
00007FF7C3EE869B | 0F1048 10                        | movups xmm1,xmmword ptr ds:[rax+10]                                                          | rax+10:"src\\main.rs"
00007FF7C3EE869F | 0F298D 90000000                  | movaps xmmword ptr ss:[rbp+90],xmm1                                                          |
00007FF7C3EE86A6 | 0F2985 80000000                  | movaps xmmword ptr ss:[rbp+80],xmm0                                                          |
00007FF7C3EE86AD | 48:8D05 44FB0100                 | lea rax,qword ptr ds:[7FF7C3F081F8]                                                          | rax:&"tcp.SrcPort == ", 00007FF7C3F081F8:&"src\\main.rs"
00007FF7C3EE86B4 | 48:894424 20                     | mov qword ptr ss:[rsp+20],rax                                                                |
00007FF7C3EE86B9 | 48:8D0D 90F70100                 | lea rcx,qword ptr ds:[7FF7C3F07E50]                                                          | 00007FF7C3F07E50:"called `Result::unwrap()` on an `Err` value"
00007FF7C3EE86C0 | 4C:8D0D 69F70100                 | lea r9,qword ptr ds:[7FF7C3F07E30]                                                           |
00007FF7C3EE86C7 | 4C:8D85 80000000                 | lea r8,qword ptr ss:[rbp+80]                                                                 |
00007FF7C3EE86CE | BA 2B000000                      | mov edx,2B                                                                                   | 2B:'+'
00007FF7C3EE86D3 | E8 28DE0100                      | call <windivert.core::result::unwrap_failed::h70751bb42e9051bd>                              |
00007FF7C3EE86D8 | E9 C0000000                      | jmp windivert.7FF7C3EE879D                                                                   |
00007FF7C3EE86DD | 48:894D C0                       | mov qword ptr ss:[rbp-40],rcx                                                                | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE86E1 | 0F2885 80000000                  | movaps xmm0,xmmword ptr ss:[rbp+80]                                                          |
00007FF7C3EE86E8 | 0F1145 C8                        | movups xmmword ptr ss:[rbp-38],xmm0                                                          |
00007FF7C3EE86EC | 48:8D05 1DFB0100                 | lea rax,qword ptr ds:[7FF7C3F08210]                                                          | rax:&"tcp.SrcPort == ", 00007FF7C3F08210:&"src\\main.rs"
00007FF7C3EE86F3 | 48:894424 20                     | mov qword ptr ss:[rsp+20],rax                                                                |
00007FF7C3EE86F8 | 48:8D0D 51F70100                 | lea rcx,qword ptr ds:[7FF7C3F07E50]                                                          | 00007FF7C3F07E50:"called `Result::unwrap()` on an `Err` value"
00007FF7C3EE86FF | 4C:8D0D 7AF70100                 | lea r9,qword ptr ds:[<&sub_7FF7C3EE7D30>]                                                    |
00007FF7C3EE8706 | 4C:8D45 C0                       | lea r8,qword ptr ss:[rbp-40]                                                                 | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE870A | BA 2B000000                      | mov edx,2B                                                                                   | 2B:'+'
00007FF7C3EE870F | E8 ECDD0100                      | call <windivert.core::result::unwrap_failed::h70751bb42e9051bd>                              |
00007FF7C3EE8714 | E9 84000000                      | jmp windivert.7FF7C3EE879D                                                                   |
00007FF7C3EE8719 | 48:8945 30                       | mov qword ptr ss:[rbp+30],rax                                                                |
00007FF7C3EE871D | 48:894D 28                       | mov qword ptr ss:[rbp+28],rcx                                                                |
00007FF7C3EE8721 | 48:8B45 D0                       | mov rax,qword ptr ss:[rbp-30]                                                                |
00007FF7C3EE8725 | 48:8945 38                       | mov qword ptr ss:[rbp+38],rax                                                                | [rbp+38]:std::thread::spawnhook::ChildSpawnHooks::run+17C
00007FF7C3EE8729 | 48:8D0D 60FE0100                 | lea rcx,qword ptr ds:[<anon.70de80080aa0578d8e2c0b1b642816f1.8.llvm.9175985875024411532>]    | 00007FF7C3F08590:"internal error: entered unreachable code"
00007FF7C3EE8730 | 4C:8D05 29FF0100                 | lea r8,qword ptr ds:[<anon.70de80080aa0578d8e2c0b1b642816f1.11.llvm.9175985875024411532>]    | 00007FF7C3F08660:&"C:\\Users\\jozef\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\std\\src\\sync\\mpmc\\mod.rs"
00007FF7C3EE8737 | BA 28000000                      | mov edx,28                                                                                   | 28:'('
00007FF7C3EE873C | E8 3FDB0100                      | call <windivert.core::panicking::panic::h0cc6c70cb61d6977>                                   |
00007FF7C3EE8741 | EB 5A                            | jmp windivert.7FF7C3EE879D                                                                   |
00007FF7C3EE8743 | 0F1085 80000000                  | movups xmm0,xmmword ptr ss:[rbp+80]                                                          |
00007FF7C3EE874A | 0F108D 90000000                  | movups xmm1,xmmword ptr ss:[rbp+90]                                                          |
00007FF7C3EE8751 | 0F294D D0                        | movaps xmmword ptr ss:[rbp-30],xmm1                                                          |
00007FF7C3EE8755 | 0F2945 C0                        | movaps xmmword ptr ss:[rbp-40],xmm0                                                          |
00007FF7C3EE8759 | 48:8D05 68FA0100                 | lea rax,qword ptr ds:[7FF7C3F081C8]                                                          | rax:&"tcp.SrcPort == ", 00007FF7C3F081C8:&"src\\main.rs"
00007FF7C3EE8760 | 48:894424 20                     | mov qword ptr ss:[rsp+20],rax                                                                |
00007FF7C3EE8765 | 48:8D0D E4F60100                 | lea rcx,qword ptr ds:[7FF7C3F07E50]                                                          | 00007FF7C3F07E50:"called `Result::unwrap()` on an `Err` value"
00007FF7C3EE876C | 4C:8D0D BDF60100                 | lea r9,qword ptr ds:[7FF7C3F07E30]                                                           |
00007FF7C3EE8773 | 4C:8D45 C0                       | lea r8,qword ptr ss:[rbp-40]                                                                 | [rbp-40]:&"tcp.SrcPort == "
00007FF7C3EE8777 | BA 2B000000                      | mov edx,2B                                                                                   | 2B:'+'
00007FF7C3EE877C | E8 7FDD0100                      | call <windivert.core::result::unwrap_failed::h70751bb42e9051bd>                              |
00007FF7C3EE8781 | EB 1A                            | jmp windivert.7FF7C3EE879D                                                                   |
00007FF7C3EE8783 | 45:31E4                          | xor r12d,r12d                                                                                |
00007FF7C3EE8786 | EB 03                            | jmp windivert.7FF7C3EE878B                                                                   |
00007FF7C3EE8788 | 4D:89FE                          | mov r14,r15                                                                                  |
00007FF7C3EE878B | 4C:8D05 96F90100                 | lea r8,qword ptr ds:[7FF7C3F08128]                                                           | 00007FF7C3F08128:&"C:\\Users\\jozef\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib/rustlib/src/rust\\library\\alloc\\src\\slice.rs"
00007FF7C3EE8792 | 4C:89E1                          | mov rcx,r12                                                                                  |
00007FF7C3EE8795 | 4C:89F2                          | mov rdx,r14                                                                                  |
00007FF7C3EE8798 | E8 B6D80100                      | call <windivert.alloc::raw_vec::handle_error::h5d55154af761dff4>                             |
00007FF7C3EE879D | 0F0B                             | ud2                                                                                          |
00007FF7C3EE879F | 90                               | nop                                                                                          |
00007FF7C3EE87A0 | 48:895424 10                     | mov qword ptr ss:[rsp+10],rdx                                                                |
00007FF7C3EE87A5 | 55                               | push rbp                                                                                     |
00007FF7C3EE87A6 | 41:57                            | push r15                                                                                     |
00007FF7C3EE87A8 | 41:56                            | push r14                                                                                     |
00007FF7C3EE87AA | 41:55                            | push r13                                                                                     |
00007FF7C3EE87AC | 41:54                            | push r12                                                                                     |
00007FF7C3EE87AE | 56                               | push rsi                                                                                     |
00007FF7C3EE87AF | 57                               | push rdi                                                                                     |
00007FF7C3EE87B0 | 53                               | push rbx                                                                                     |
00007FF7C3EE87B1 | 48:83EC 28                       | sub rsp,28                                                                                   |
00007FF7C3EE87B5 | 48:8DAA 80000000                 | lea rbp,qword ptr ds:[rdx+80]                                                                |
00007FF7C3EE87BC | 48:8B4D 68                       | mov rcx,qword ptr ss:[rbp+68]                                                                |
00007FF7C3EE87C0 | 48:8B01                          | mov rax,qword ptr ds:[rcx]                                                                   | rax:&"tcp.SrcPort == "
00007FF7C3EE87C3 | 48:83C1 08                       | add rcx,8                                                                                    |
00007FF7C3EE87C7 | 48:85C0                          | test rax,rax                                                                                 | rax:&"tcp.SrcPort == "
00007FF7C3EE87CA | 74 0C                            | je windivert.7FF7C3EE87D8                                                                    |
00007FF7C3EE87CC | 83F8 01                          | cmp eax,1                                                                                    |
00007FF7C3EE87CF | 75 6B                            | jne windivert.7FF7C3EE883C                                                                   |
00007FF7C3EE87D1 | E8 AAE7FFFF                      | call <windivert.std::sync::mpmc::counter::Sender$LT$C$GT$::release::h556aa944d0fa04c6>       |
00007FF7C3EE87D6 | EB 69                            | jmp windivert.7FF7C3EE8841                                                                   |
00007FF7C3EE87D8 | 48:8B31                          | mov rsi,qword ptr ds:[rcx]                                                                   |
00007FF7C3EE87DB | F048:FF8E 00020000               | lock dec qword ptr ds:[rsi+200]                                                              |
00007FF7C3EE87E3 | 75 5C                            | jne windivert.7FF7C3EE8841                                                                   |
00007FF7C3EE87E5 | 48:8B86 80000000                 | mov rax,qword ptr ds:[rsi+80]                                                                | rax:&"tcp.SrcPort == "
00007FF7C3EE87EC | 48:8B8E 90010000                 | mov rcx,qword ptr ds:[rsi+190]                                                               |
00007FF7C3EE87F3 | 666666662E:0F1F8400 00000000     | nop word ptr ds:[rax+rax],ax                                                                 |
00007FF7C3EE8800 | 48:89C2                          | mov rdx,rax                                                                                  | rax:&"tcp.SrcPort == "
00007FF7C3EE8803 | 48:09CA                          | or rdx,rcx                                                                                   |
00007FF7C3EE8806 | F048:0FB196 80000000             | lock cmpxchg qword ptr ds:[rsi+80],rdx                                                       |
00007FF7C3EE880F | 75 EF                            | jne windivert.7FF7C3EE8800                                                                   |
00007FF7C3EE8811 | 48:8586 90010000                 | test qword ptr ds:[rsi+190],rax                                                              | rax:&"tcp.SrcPort == "
00007FF7C3EE8818 | 75 0C                            | jne windivert.7FF7C3EE8826                                                                   |
00007FF7C3EE881A | 48:8D8E 40010000                 | lea rcx,qword ptr ds:[rsi+140]                                                               |
00007FF7C3EE8821 | E8 CAB2FFFF                      | call <windivert.std::sync::mpmc::waker::SyncWaker::disconnect::h6ad45fdb3df37a47 (.llvm.9322 |
00007FF7C3EE8826 | B0 01                            | mov al,1                                                                                     |
00007FF7C3EE8828 | 8686 10020000                    | xchg byte ptr ds:[rsi+210],al                                                                |
00007FF7C3EE882E | 84C0                             | test al,al                                                                                   |
00007FF7C3EE8830 | 74 0F                            | je windivert.7FF7C3EE8841                                                                    |
00007FF7C3EE8832 | 48:89F1                          | mov rcx,rsi                                                                                  |
00007FF7C3EE8835 | E8 E6EDFFFF                      | call <windivert.core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$std..sync..mpmc..counter..C |
00007FF7C3EE883A | EB 05                            | jmp windivert.7FF7C3EE8841                                                                   |
00007FF7C3EE883C | E8 8FE8FFFF                      | call <windivert.std::sync::mpmc::counter::Sender$LT$C$GT$::release::h9ff499d21cc3324b>       |
00007FF7C3EE8841 | 90                               | nop                                                                                          |
00007FF7C3EE8842 | 48:83C4 28                       | add rsp,28                                                                                   |
00007FF7C3EE8846 | 5B                               | pop rbx                                                                                      |
00007FF7C3EE8847 | 5F                               | pop rdi                                                                                      |
00007FF7C3EE8848 | 5E                               | pop rsi                                                                                      |
00007FF7C3EE8849 | 41:5C                            | pop r12                                                                                      |
00007FF7C3EE884B | 41:5D                            | pop r13                                                                                      |
00007FF7C3EE884D | 41:5E                            | pop r14                                                                                      |
00007FF7C3EE884F | 41:5F                            | pop r15                                                                                      |
00007FF7C3EE8851 | 5D                               | pop rbp                                                                                      |
00007FF7C3EE8852 | C3                               | ret                                                                                          |
```