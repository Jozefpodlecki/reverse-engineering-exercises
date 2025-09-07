
This program generates a random creature variant and prints its name using a precomputed lookup table.
The Rust enum is transmuted from a random byte, and a match on the enum is replaced by a table-based string lookup in release mode.

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/enums/src/main.rs)

```
       
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