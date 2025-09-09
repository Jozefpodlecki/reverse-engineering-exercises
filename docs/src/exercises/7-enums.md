
This program generates a random creature variant and prints its name using a precomputed lookup table.
The Rust enum is transmuted from a random byte, and a match on the enum is replaced by a table-based string lookup in release mode.

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/enums/src/main.rs)
       
```x86asm
call rand::rngs::thread::rng
mov rsi, rax
mov [rbp+local_20], rax
```

Initializes ThreadRng and stores its pointer in rsi and a local variable.

```x86asm
mov rcx, [rsi + 0x150]
cmp rcx, 0x40
jc SHORT LAB_BUFFER_OK
```

Loads the RNG buffer index and checks if it needs a refill.

```x86asm
lea r8, [rax + 0x10]
add rax, 0x110
mov rcx, [rsi + 0x148]
test rcx, rcx
jle SHORT LAB_REPLENISH_FALLBACK
add rcx, -0x100
mov [rsi + 0x148], rcx
mov rcx, rax
mov edx, 0x6
call rand_chacha::guts::refill_wide
jmp SHORT LAB_CONTINUE
```

```x86asm
LAB_REPLENISH_FALLBACK:
mov rcx, rax
mov rdx, r8
call FUN_140001050
```

Refills the ChaCha RNG buffer if exhausted.
Fallback path for insufficient buffer, calls internal RNG helper.

```x86asm
lea rcx, [DAT_STRING_OFFSET_TABLE]
movsxd rdx, [rcx + rax*4]
add rdx, rcx
lea rcx, [DAT_STRING_PTR_TABLE]
mov rax, [rcx + rax*8]
mov [rbp + local_30], rdx
mov [rbp + local_28], rax
```

Maps the enum variant to a string pointer using precomputed offset and pointer tables.

[rcx + rax*4]
- The offset table uses 32-bit entries (4 bytes each).
- rax holds the random enum variant (0–31).
- Multiplying rax by 4 gives the byte offset for the correct entry.
- MOVSXD rdx, [rcx + rax*4] loads the 32-bit offset for the variant and sign-extends it to 64-bit.
- This offset is relative to the base of the string table, so adding rdx + rcx produces the absolute string address.

[rcx + rax*8]
- The pointer table uses 64-bit entries (8 bytes each).
- Again, rax is the variant index.
- Multiplying by 8 gives the byte offset of the correct pointer.
- MOV rax, [rcx + rax*8] loads the full 64-bit pointer directly, no sign-extension needed.

```x86asm
lea rax, [rbp + local_30]
mov [rbp + local_40], rax
lea rax, [LAB_140001030]
mov [rbp + local_38], rax
lea rax, [DAT_CONST_1]
mov [rbp + local_70], rax
mov [rbp + local_68], 2
mov [rbp + local_50], 0
lea rax, [rbp + local_40]
mov [rbp + local_60], rax
mov [rbp + local_58], 1
lea rcx, [rbp + local_70]
call std::io::stdio::_print
```

Prepares arguments and calls _print to output the creature name.

```x86asm
mov rax, [rbp + local_20]
dec [rax]
jz SHORT LAB_DROP
add rsp, 0x88
pop rsi
pop rbp
ret
mov rax, [rbp + local_20]
```

Loads the pointer to the ThreadRng instance, which is stored in local_20.

`dec [rax]`

Decrements the reference count of the ThreadRng.
ThreadRng is behind an Rc (reference-counted smart pointer).

`jz SHORT LAB_DROP`

```x86asm
LAB_DROP:
lea rcx, [rbp + local_20]
call alloc::rc::Rc<T,A>::drop_slow
```

If the reference count reached zero, jump to drop_slow to free the memory.
If not zero, skip and just clean the stack.

`add rsp, 0x88; pop rsi; pop rbp; ret`

Standard stack cleanup and return from main if the RNG instance is still alive (ref count > 0).