This exercise demonstrates boxing in Rust, i.e., allocating various types on the heap using Box. We analyze the assembly produced for three examples:

- `Box<i32>`
- `Box<(i32, i32, i32, i32, i32)>`
- `Box<dyn TTrait>`

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/boxing/main.rs)


```
CALL alloc::alloc::exchange_malloc
MOV qword ptr [local_78], RAX
```

- Calls Rust’s allocator to get heap memory for Box.
- RAX contains pointer to heap

```
MOV RAX, qword ptr [local_78]
MOV dword ptr [RAX], 0x2a
```

- Moves the value 42 into the heap location pointed by the Box.

```
MOV RAX, qword ptr [local_e0]
MOV dword ptr [RAX + 0x10], some_value
MOVUPS XMM0, xmmword ptr [local_bc]
MOVUPS xmmword ptr [RAX], XMM0
MOV qword ptr [local_c8], RAX
```

- Copies the tuple elements into the allocated heap memory.
- Uses XMM registers for 16-byte chunks (SIMD instructions) to move multiple integers efficiently.

```
MOV RAX, qword ptr [local_e8]
MOV qword ptr [RAX + 0x10], _Argc
MOVAPS XMM0, xmmword ptr [local_98]
MOVUPS xmmword ptr [RAX], XMM0
```

- Copies the trait object into heap memory.
- XMM instructions handle the String struct (pointer, length, capacity).

```
LEA RAX, [vtable$]
MOV qword ptr [local_a0], RAX
```

- Loads the vtable pointer for the dyn TTrait trait object.
- Rust fat pointer = {data pointer, vtable pointer}.