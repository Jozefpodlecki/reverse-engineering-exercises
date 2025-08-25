
```
peanalyse -i magic-string.exe --entry
Architecture: Pe64
AddressOfEntryPoint (RVA): 0x000177E0
ImageBase:                0x140000000
EntryPoint (VA):          0x1400177E0
```

### `mainCRTStartup 0x1400177E0`

```x86asm
sub rsp,28
call <magic-string.__security_init_cookie>
add rsp,28
jmp <magic-string.__scrt_common_main_seh> # 0x140017664
```

### `__scrt_common_main_seh 0x140017664`

```x86asm
mov qword ptr ss:[rsp+8],rbx
mov qword ptr ss:[rsp+10],rsi
push rdi
sub rsp,30
mov ecx,1
call <magic-string.__scrt_initialize_crt>
test al,al
...
mov r8,rdi
mov rdx,rbx
mov ecx,dword ptr ds:[rax]
call <magic-string.main> # 0x140001310
...
add rsp,30
pop rdi
ret 
```

Initializes CRT and calls main..<br/>
Standard SEH (Structured Exception Handling) setup.

### `main 0x140001310`

```x86asm
sub rsp,38
mov r9,rdx
movsxd r8,ecx
lea rax,qword ptr ds:[<sub_7FF6AB8E1100>] ; function pointer to Rust closure
mov qword ptr ss:[rsp+30],rax
mov byte ptr ss:[rsp+20],0
lea rdx,qword ptr ds:[7FF6AB8FA3A0]
lea rcx,qword ptr ss:[rsp+30]
call <magic-string.std::rt::lang_start_internal::h5830dd6b2696abe8> # 0x140003080
nop 
add rsp,38
ret 
```

Converts main to a trait object `(dyn Fn() -> i32)`.<br/>
Calls lang_start_internal with a fat pointer.

### `lang_start_internal 0x140003080`

```x86asm
push rbp
push rsi
push rdi
sub rsp,A0
lea rbp,qword ptr ss:[rsp+80]
mov qword ptr ss:[rbp],FFFFFFFFFFFFFFFE
mov rsi,rdx
mov rdi,rcx
...
mov eax,dword ptr ds:[<_tls_index>]
mov rdx,qword ptr gs:[58]
mov rax,qword ptr ds:[rdx+rax*8]
mov qword ptr ds:[rax+8],rcx
mov qword ptr ds:[7FF6AB904220],rcx
mov rcx,rdi
call qword ptr ds:[rsi+28]
...
ret 
```

Implements Rust runtime startup and panic handling.<br/>
Calls the main closure via vtable ([rsi+28]).

### `Fat pointer (dyn Fn() -> i32 + Sync)`

```x86asm
sub rsp,28
mov rcx,qword ptr ds:[rcx]
call <magic-string.sub_7FF6AB8E1020> # 0x140001020
xor eax,eax
add rsp,28
ret 
```

First stub: loads data pointer from fat pointer.

### `0x140001020`

```x86asm
sub rsp,28
call rcx # 0x140001100
nop 
add rsp,28
ret 
```

Second stub: trampoline calls the real `rust_main`.

### `rust_main`

```x86asm
push rbp
sub rsp,80
lea rbp,qword ptr ss:[rsp+80]
mov qword ptr ss:[rbp-8],FFFFFFFFFFFFFFFE
mov qword ptr ss:[rbp-28],0
mov qword ptr ss:[rbp-20],1
mov qword ptr ss:[rbp-18],0
call <magic-string.std::io::stdio::stdin::hb37ee29cfde5d31f>
mov qword ptr ss:[rbp-10],rax
lea rcx,qword ptr ss:[rbp-10]
lea rdx,qword ptr ss:[rbp-28]
call <magic-string.std::io::stdio::Stdin::read_line::h9f941692afe5412c>
test al,1
jne magic-string.7FF6AB8E1219
cmp qword ptr ss:[rbp-18],14
jne magic-string.7FF6AB8E1181
mov rax,qword ptr ss:[rbp-20]
movdqu xmm0,xmmword ptr ds:[rax]
movd xmm1,dword ptr ds:[rax+10]
pcmpeqb xmm1,xmmword ptr ds:[<__xmm@00000000000000000000000077626e41>]
pcmpeqb xmm0,xmmword ptr ds:[<__xmm@233d7138577448503841673436394c75>]
pand xmm0,xmm1
pmovmskb eax,xmm0
cmp eax,FFFF
je magic-string.7FF6AB8E11EA
lea rax,qword ptr ds:[7FF6AB8FA460]
mov qword ptr ss:[rbp-58],rax
mov qword ptr ss:[rbp-50],1
mov qword ptr ss:[rbp-48],8
pxor xmm0,xmm0
movdqu xmmword ptr ss:[rbp-40],xmm0
lea rcx,qword ptr ss:[rbp-58]
call <magic-string.std::io::stdio::_print::h033ee6824b02a35e>
call <magic-string.std::io::stdio::stdin::hb37ee29cfde5d31f>
mov qword ptr ss:[rbp-10],rax
lea rcx,qword ptr ss:[rbp-10]
lea rdx,qword ptr ss:[rbp-28]
call <magic-string.std::io::stdio::Stdin::read_line::h9f941692afe5412c>
test al,1
jne magic-string.7FF6AB8E1247
mov rdx,qword ptr ss:[rbp-28]
test rdx,rdx
je magic-string.7FF6AB8E11E0
mov rcx,qword ptr ss:[rbp-20]
mov r8d,1
call <magic-string.__rustc::__rust_dealloc>
nop 
add rsp,80
pop rbp
ret 
```

### Read Line

```x86asm
lea rcx,qword ptr ss:[rbp-10]
lea rdx,qword ptr ss:[rbp-28]
call <magic-string.std::io::stdio::Stdin::read_line::h9f941692afe5412c>
test al,1
jne magic-string.7FF6AB8E1219
```

- `read_line` writes the input directly into the stack-allocated buffer at `[rbp-28]`
- The number of bytes read is stored at `[rbp-18]`.

### Input Length Check

```x86asm
cmp qword ptr ss:[rbp-18],14
jne magic-string.7FF6AB8E1181
```

- rbp-18 stores the number of bytes read from stdin.
- Compares it against 0x14 (20 decimal).
- If input length ≠ 20 → jump to “Denied” handler.

### String Comparison (SIMD)

```x86asm
mov rax,[rbp-20]       ; pointer to buffer
movdqu xmm0,[rax]       ; first 16 bytes
movd xmm1,[rax+16]      ; next 4 bytes
pcmpeqb xmm1,[__xmm@000000…] ; compare bytes with constant1
pcmpeqb xmm0,[__xmm@233d…]   ; compare bytes with constant2
pand xmm0,xmm1
pmovmskb eax,xmm0
cmp eax,FFFF
```

Load input into XMM registers
- xmm0 = first 16 bytes of user input.
- xmm1 = next 4 bytes of user input (loaded with movd).

Compare bytes against constants
- pcmpeqb xmm0, [pattern2] → sets each byte in xmm0 to 0xFF if equal, 0x00 if not.
- pcmpeqb xmm1, [pattern1] → same for xmm1.

Combine results
- pand xmm0, xmm1 → bitwise AND of the results.

Ensures all bytes must match across both chunks.

Extract mask
- pmovmskb eax, xmm0 → extract the MSB of each byte into a 16-bit mask in eax.

Check full match
- cmp eax, 0xFFFF → all 16 bytes in xmm0 matched.

If not all match → jump to Denied handler.

| Label                  | Value                    |
| ---------------------- | ------------------------ |
| `__xmm@000...77626e41` | `"uL964gA8PHtW8q=#"`     |
| `__xmm@233d...34639…`  | `"Anbw"`                 |
| Full target string     | `"uL964gA8PHtW8q=#Anbw"` |


Resources
- https://binarydefense.com/resources/blog/digging-through-rust-to-find-gold-extracting-secrets-from-rust-malware/