

This exercise tauri app template.

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/tauri/src-tauri/src/lib.rs)

`tauri::plugin::Builder<R,C>::new`
`_ZN5tauri6plugin20Builder$LT$R$C$C$GT$3new`

```x86asm
140096e10 e8 5b 4d 4f 00 CALL std::hash::random::impl$0::new::KEYS::constant
140096e15 49 89 c6        MOV R14,RAX
140096e18 80 78 10 01     CMP byte ptr [RAX+0x10],0x1
```

- Part of HashMap/FxHasher initialization.
- Returns a pointer (RAX → R14) to the random seed/keys used internally by the plugin builder.

```x86asm
140096e22 49 8b 06        MOV RAX,qword ptr [R14]
140096e25 49 8b 56 08     MOV RDX,qword ptr [R14+0x8]
140096e29 48 8d 48 01     LEA RCX,[RAX+1]
140096e2d 49 89 0e        MOV qword ptr [R14],RCX
```

- Increments an internal counter stored in the random::KEYS structure.
- Likely used to ensure unique identifiers for each plugin instance.


`_ZN83_$LT$tauri_runtime_wry..Wry$LT$T$GT$$u20$_<>::run`

```
1400fc4a0 MOV RSI, qword ptr [RCX + 0x50]
1400fc4a4 INC.LOCK qword ptr [RSI]
1400fc4a8 JLE LAB_1400fc4c8
```

- Loads a pointer from the object (RCX points to self).
- Atomically increments the value pointed to by that pointer.
- Jumps if less than or equal to zero (JLE) to LAB_1400fc4c8.

```x86asm
1400fc4aa MOV RDI, qword ptr [RCX + 0x18]
1400fc4ae INC.LOCK qword ptr [RDI]
1400fc4b2 JLE LAB_1400fc4c8

1400fc4b4 MOV RBX, qword ptr [RCX + 0x48]
1400fc4b8 INC.LOCK qword ptr [RBX]
1400fc4bc JLE LAB_1400fc4c8

1400fc4be MOV RAX, qword ptr [RCX + 0x58]
1400fc4c2 INC.LOCK qword ptr [RAX]
1400fc4c6 JG LAB_1400fc4ca
```

- Each load gets a pointer from the struct and performs an atomic increment.
- If any of these increments result in <= 0, the code jumps to a UD2 instruction (illegal instruction) at LAB_1400fc4c8.
- The last increment uses JG (jump if greater) to continue normal execution.

