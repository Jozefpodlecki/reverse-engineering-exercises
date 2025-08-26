
# string-clone

This program demonstrates the use of the rand crate for random string generation and includes a String cloning operation, both of which we will examine during reverse engineering.

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/magic-string/src/main.rs)

0x140000240

### Function Prologue / Stack Setup

```x86asm
push rbp
push r15
push r14
push r13
push r12
push rsi
push rdi
push rbx
sub rsp,98
lea rbp,qword ptr ss:[rsp+80]
```

sub rsp,0x98 allocates 152 bytes for local variables, temporaries, and intermediate Rust structures.
lea rbp,[rsp+0x80] establishes a frame pointer for easier addressing of locals.

### Local Variable Initialization

```x86asm
mov qword ptr ss:[rbp+10],FFFFFFFFFFFFFFFE
mov qword ptr ss:[rbp-8],0
mov qword ptr ss:[rbp],8
mov qword ptr ss:[rbp+8],0
```

### RNG Initialization

```x86asm
call <string-clone._$LT$rand..rngs..thread..ThreadRng$u20$as$u20$core..default..Default$GT$::default::h6ef1eba5b37f19f7>
mov rsi,rax
mov qword ptr ss:[rbp-38],rax
```

### Random sampling / bounds checking

```x86asm
lea rdi,qword ptr ds:[rax+10]
mov rcx,qword ptr ds:[rax+150]
cmp rcx,40
jb string-clone.7FF70D2612D1
lea rcx,qword ptr ds:[rsi+110]
mov rax,qword ptr ds:[rsi+148]
test rax,rax
jle string-clone.7FF70D2612C7
add rax,FFFFFFFFFFFFFF00
mov qword ptr ds:[rsi+148],rax
mov edx,6
mov r8,rdi
call <string-clone.rand_chacha::guts::refill_wide::hc43b38d823048efe>
jmp string-clone.7FF70D2612CF
mov rdx,rdi
call string-clone.7FF70D2610B0
```

### Character Extraction & Indexing

```x86asm
xor ecx,ecx
mov edx,dword ptr ds:[rsi+rcx*4+10]
lea rax,qword ptr ds:[rcx+1]
mov qword ptr ds:[rsi+150],rax
lea rbx,qword ptr ds:[rdx+rdx*4]
mov r15,rbx
shr r15,20
cmp ebx,FFFFFFFC
jb string-clone.7FF70D26134E
cmp rcx,3F
jne string-clone.7FF70D261332
mov rcx,rsi
add rcx,110
mov rax,qword ptr ds:[rsi+148]
test rax,rax
jle string-clone.7FF70D261328
add rax,FFFFFFFFFFFFFF00
mov qword ptr ds:[rsi+148],rax
mov edx,6
mov r8,rdi
call <string-clone.rand_chacha::guts::refill_wide::hc43b38d823048efe>
jmp string-clone.7FF70D261330
mov rdx,rdi
call string-clone.7FF70D2610B0
xor eax,eax
mov ecx,dword ptr ds:[rsi+rax*4+10]
inc rax
mov qword ptr ds:[rsi+150],rax
lea rax,qword ptr ds:[rcx+rcx*4]
shr rax,20
add ebx,eax
adc r15,0
mov rax,qword ptr ss:[rbp-38]
dec qword ptr ds:[rax]
jne string-clone.7FF70D261360
lea rcx,qword ptr ss:[rbp-38]
call <string-clone.alloc::rc::Rc$LT$T$C$A$GT$::drop_slow::hc3331cc6b5b2df8f>
add r15d,5
xor r12d,r12d
mov esi,8
lea rdi,qword ptr ss:[rbp-50]
lea rbx,qword ptr ss:[rbp-38]
lea r13,qword ptr ss:[rbp-60]
xor r14d,r14d
jmp string-clone.7FF70D261383
nop dword ptr ds:[rax],eax
inc r14d
cmp r14d,r15d
jge string-clone.7FF70D26142D
mov rcx,rdi
call string-clone.7FF70D2794B0
```

### String Clone / Push

```x86asm
mov rcx,rbx
mov rdx,rdi
call <string-clone._$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone::h05994eb0d9419be8>
cmp r12,qword ptr ss:[rbp-8]
jne string-clone.7FF70D2613B2
lea rcx,qword ptr ss:[rbp-8]
call string-clone.7FF70D261160
mov rsi,qword ptr ss:[rbp]
lea rax,qword ptr ds:[r12+r12*2]
mov rcx,qword ptr ss:[rbp-28]
mov qword ptr ds:[rsi+rax*8+10],rcx
movups xmm0,xmmword ptr ss:[rbp-38]
movups xmmword ptr ds:[rsi+rax*8],xmm0
inc r12
mov qword ptr ss:[rbp+8],r12
mov qword ptr ss:[rbp-60],rdi
lea rax,qword ptr ds:[7FF70D261220]
mov qword ptr ss:[rbp-58],rax
lea rax,qword ptr ds:[7FF70D27C478]
mov qword ptr ss:[rbp-38],rax
mov qword ptr ss:[rbp-30],2
mov qword ptr ss:[rbp-18],0
mov qword ptr ss:[rbp-28],r13
mov qword ptr ss:[rbp-20],1
```

### Printing

```x86asm
mov rcx,rbx
call <string-clone.std::io::stdio::_print::h033ee6824b02a35e>
```

### Cleanup

```x86asm
mov rdx,qword ptr ss:[rbp-50]
test rdx,rdx
je string-clone.7FF70D261380
mov rcx,qword ptr ss:[rbp-48]
mov r8d,1
call <string-clone.__rustc::__rust_dealloc>
```

```x86asm
jmp string-clone.7FF70D261380
test r12,r12
je string-clone.7FF70D261462
lea rdi,qword ptr ds:[rsi+8]
jmp string-clone.7FF70D261449
nop dword ptr ds:[rax+rax],eax
add rdi,18
dec r12
je string-clone.7FF70D261462
mov rdx,qword ptr ds:[rdi-8]
test rdx,rdx
je string-clone.7FF70D261440
mov rcx,qword ptr ds:[rdi]
mov r8d,1
call <string-clone.__rustc::__rust_dealloc>
jmp string-clone.7FF70D261440
mov rax,qword ptr ss:[rbp-8]
test rax,rax
je string-clone.7FF70D261481
shl rax,3
lea rdx,qword ptr ds:[rax+rax*2]
mov r8d,8
mov rcx,rsi
call <string-clone.__rustc::__rust_dealloc>
```

### Function Epilogue

```x86asm
nop
add rsp,98
pop rbx
pop rdi
pop rsi
pop r12
pop r13
pop r14
pop r15
pop rbp
ret 
```