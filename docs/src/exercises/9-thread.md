

This exercise is spawning thread

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/thread/main.rs)

`std::thread::Builder::spawn_unchecked_<>(enum2$<> *__return_storage_ptr__,ulonglong *param_1,ulonglong param_3)`


```x86asm
MOV RAX,qword ptr [RDX + param_1->name]
MOV [RBP + local_360[0]], RAX
MOVUPS XMM0, [RBP + local_360[0]]
MOVAPS [RBP + local_328[0]], XMM0
```

- It is loading the closure environment into stack locals.
- Uses XMM registers for SSE-aligned data (likely a closure capturing f32/f64 or SIMD-like structure).
- This is preparation for passing the closure to the thread entry function.
- Conceptually: let closure_data = f but in raw memory copy form.

```x86asm
LEA RAX, [impl$<>::vtable$]
MOV [RBP + local_3d0], RAX
CALL core::ptr::non_null::impl$3::new_unchecked
```

- Rust sets up fat pointer for trait object: (data pointer, vtable).
- NonNull ensures no null pointer in the closure box.
- This is essentially creating a `Box<dyn FnOnce>` for the thread

`void __cdecl std::sys::pal::windows::thread::Thread::new(void)`

```x86asm
14001115f 48 c7 45 f8 fe ff ff ff   MOV  qword ptr [RBP + local_30],-2
140011167 4c 89 cf                    MOV  RDI,R9
14001116a 48 89 ce                    MOV  RSI,RCX
140011171 e8 3a 63 ff ff ff           CALL __rust_no_alloc_shim_is_unstable_v2
140011176 b9 10 00 00 00              MOV  ECX,0x10
14001117b ba 08 00 00 00              MOV  EDX,0x8
140011180 e8 db 62 ff ff ff           CALL __rust_alloc
140011185 48 85 c0                    TEST RAX,RAX
140011188 0f 84 ac 00 00 00           JZ   LAB_14001123a
```

- Moves parameters into registers (RDI = thread entry, RSI = data pointer).
- Allocates memory for the thread object (__rust_alloc).
- Checks if allocation succeeded.

```x86asm
14001118e 48 89 c3      MOV RBX,RAX
140011191 48 89 38      MOV [RAX],RDI
140011194 4c 89 70 08   MOV [RAX + 8],R14
```

Stores the thread entry function pointer and data pointer into the allocated memory.

```x86asm
140011198 48 c7 44 24 28 00 00 00 00  MOV [RSP + local_50],0
1400111a1 c7 44 24 20 00 01 00 00     MOV [RSP + local_58],0x10000
1400111a9 4c 8d 05 30 01 00 00        LEA R8,[thread start routine]
1400111b0 31 ff                          XOR EDI,EDI
1400111b2 31 c9                          XOR ECX,ECX
1400111b4 48 89 f2                       MOV RDX,RSI
1400111b7 49 89 c1                       MOV R9,RAX
1400111ba ff 15 20 ef 00 00 00          CALL [KERNEL32.DLL::CreateThread]
```

Prepares parameters for CreateThread:
- lpThreadAttributes = NULL
- dwStackSize = 0x10000 (64 KB)
- lpStartAddress = thread function
- lpParameter = pointer to Rust thread struct
- dwCreationFlags = 0 (start immediately)
- Calls Windows API CreateThread to start the thread.

```
1400111c8 48 8b 03        MOV RAX,[RBX]
1400111cb 48 89 45 f0     MOV [RBP + local_38],RAX
1400111cf 48 89 5d e0     MOV [RBP + local_48],RBX
1400111d3 48 8b 43 08     MOV RAX,[RBX + 8]
1400111d7 48 89 45 e8     MOV [RBP + local_40],RAX
1400111db 48 8b 00        MOV RAX,[RAX]
1400111de 48 85 c0        TEST RAX,RAX
1400111e1 74 06           JZ   LAB_1400111e9
1400111e3 48 8b 4d f0     MOV RCX,[RBP + local_38]
1400111e7 ff d0           CALL RAX
```

- Calls destructors for the Rust thread structure if CreateThread failed.
- Frees memory allocated for thread object and its data.