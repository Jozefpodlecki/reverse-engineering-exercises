
This exercise explores how Rust represents function trait objects at a low level. Specifically, we examine:

```
fn_trait: &(dyn Fn() -> i32 + Sync + RefUnwindSafe)
```

This is a trait object reference: a pointer to a function-like value that is thread-safe (Sync) and panic-safe (RefUnwindSafe).

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/fn_trait_object/src/main.rs)


### Setting up the Trait Object

```x86asm
00007FF7C1D914B0 | 48:83EC 38       | sub rsp,38          ; allocate 56 bytes stack
00007FF7C1D914B4 | 48:8D4C24 36     | lea rcx,[rsp+36]   ; closure environment pointer (data)
00007FF7C1D914B9 | 48:8D15 687F0100 | lea rdx,[vtable]   ; vtable pointer
00007FF7C1D914D2 | E8 69FEFFFF      | call call_fn_trait  ; call the wrapper function
```

### Loading the fat pointer

```x86asm
00007FF7C1D91425 | 48:8B4C24 20 | mov rcx,qword ptr ss:[rsp+20]  
00007FF7C1D9142A | 48:8B5424 28 | mov rdx,qword ptr ss:[rsp+28]  
00007FF7C1D9142F | FF52 28      | call qword ptr ds:[rdx+28]                                                        |
```

RCX = closure environment pointer (data)
RDX = vtable pointer (vtable)
call [rdx+28] = calls FnOnce::call_once via vtable.

### Trait Object Layout

```
struct FnVtable {
    void* drop_in_place;     // offset 0x00
    unsigned long long size; // offset 0x08
    unsigned long long align;// offset 0x10
    void* call;              // offset 0x18 (Fn::call)
    void* call_mut;          // offset 0x20 (FnMut::call_mut)
    void* call_once;         // offset 0x28 (FnOnce::call_once)
};

struct FnTraitObject {
    void* data;              // 0x00 (closure environment)
    void* vtable;            // 0x08 (pointer to FnVtable)
};
```

### Observing in x64dbg

Struct tab → Parse Header → Display Type

Define FnVtable

Map RCX → data and RDX → vtable.

Step into call_once via vtable to see the constant returned.

Return value appears in RAX:

### Closure Execution `void* call`

```x86asm
00007FF7C1D911C0 | 48:83EC 38                       | sub rsp,38                                                   | function.rs:250
00007FF7C1D911C4 | 48:894C24 30                     | mov qword ptr ss:[rsp+30],rcx                                |
00007FF7C1D911C9 | E8 32000000                      | call <fn_trait_object.core::ops::function::FnOnce::call_once |
00007FF7C1D911CE | 90                               | nop                                                          |
00007FF7C1D911CF | 48:83C4 38                       | add rsp,38                                                   |
00007FF7C1D911D3 | C3                               | ret                                                          |                            |
```


### Closure Execution `void* call_mut`, `void* call_once`

```x86asm
00007FF7C1D91500 | 50                               | push rax                                                     | main.rs:18
00007FF7C1D91501 | 48:890C24                        | mov qword ptr ss:[rsp],rcx                                   |
00007FF7C1D91505 | B8 87D61200                      | mov eax,12D687                                               |
00007FF7C1D9150A | 59                               | pop rcx                                                      |
00007FF7C1D9150B | C3                               | ret                                                          |                                                       |
```

```rust
let closure = || 1234567;
```

The closure simply loads the constant 1234567 into RAX

### Captured variables example

```rust
let a = 1234567i32;
let b = 99u64;
let closure = || {
    a + b as i32
};
```

It compiles the closure into a struct-like environment containing these captured values.

This environment is passed as a pointer (RCX) to the generated closure code when called via a trait object.

```x86asm
00007FF7FA7914C0 | 48:83EC 48           | sub rsp,48           ; allocate 72 bytes for closure
00007FF7FA7914C4 | C74424 2C 87D61200   | mov dword ptr [rsp+2C],12D687  ; store 'a' (1234567)
00007FF7FA7914CC | 48:C74424 30 63000000| mov qword ptr [rsp+30],63      ; store 'b' (99)
00007FF7FA7914D5 | 48:8D4424 2C         | lea rax,[rsp+2C]    ; load address of 'a'
00007FF7FA7914DA | 48:894424 38         | mov [rsp+38],rax    ; save pointer in closure env
00007FF7FA7914DF | 48:8D4424 30         | lea rax,[rsp+30]    ; load address of 'b'
00007FF7FA7914E4 | 48:894424 40         | mov [rsp+40],rax    ; save pointer in closure env
```

[rsp+38] = pointer to a

[rsp+40] = pointer to b

This closure environment is what RCX will point to when the closure is called.

#### Inside the Closure

```x86asm
00007FF7FA791544 | 48:894C24 30 | mov [rsp+30], rcx
```

Store RCX, which points to the closure environment, on the stack at [rsp+30].

```x86asm
00007FF7FA791549 | 48:8B4424 30 | mov rax, [rsp+30]
00007FF7FA79154E | 48:8B00      | mov rax, [rax]
00007FF7FA791551 | 48:894424 38 | mov [rsp+38], rax
```

Load the closure environment pointer from [rsp+30] into RAX.

Dereference it ([rax]) to get the first vtable pointer (often drop_in_place).

Store this in [rsp+38] — this is now the first field of the fat pointer/closure struct.

```x86asm
00007FF7FA791556 | 48:8B4424 30 | mov rax, [rsp+30]
00007FF7FA79155B | 48:8B40 08   | mov rax, [rax+8]
00007FF7FA79155F | 48:894424 40 | mov [rsp+40], rax
```

Reload closure env pointer from [rsp+30] into RAX.

Dereference [rax+8] — second field of the environment (often the pointer to captured data).

Store at [rsp+40].

```x86asm
00007FF7FA791564 | 48:8B01         | mov rax,[rcx]       ; load pointer to 'a'
00007FF7FA791567 | 8B00            | mov eax,[rax]       ; load value of 'a'
00007FF7FA791569 | 48:8B49 08      | mov rcx,[rcx+8]     ; load pointer to 'b'
00007FF7FA79156D | 48:8B09         | mov rcx,[rcx]       ; load value of 'b'
00007FF7FA791570 | 01C8            | add eax,ecx         ; perform addition a + b
```

RCX points to the closure environment.

[RCX] = pointer to a, [RCX+8] = pointer to b.

mov eax,[rax] and mov rcx,[rcx] dereference the pointers to get the actual values.

add eax, ecx computes the closure result.