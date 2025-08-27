
# string-clone

This program demonstrates the use of the rand crate for random string generation and includes a String cloning operation, both of which we will examine during reverse engineering.

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/magic-string/src/main.rs)

0x140000240

### Function Prologue / Stack Setup

```x86asm
00007FF70D261240 | 55                               | push rbp                                
00007FF70D261241 | 41:57                            | push r15                                
00007FF70D261243 | 41:56                            | push r14                                
00007FF70D261245 | 41:55                            | push r13                                
00007FF70D261247 | 41:54                            | push r12                                
00007FF70D261249 | 56                               | push rsi                                
00007FF70D26124A | 57                               | push rdi                                
00007FF70D26124B | 53                               | push rbx                                
00007FF70D26124C | 48:81EC 98000000                 | sub rsp,98                              
00007FF70D261253 | 48:8DAC24 80000000               | lea rbp,qword ptr ss:[rsp+80]           
```

### Local Variable Initialization

```x86asm
00007FF70D26125B | 48:C745 10 FEFFFFFF              | mov qword ptr ss:[rbp+10],FFFFFFFFFFFFF 
00007FF70D261263 | 48:C745 F8 00000000              | mov qword ptr ss:[rbp-8],0              
00007FF70D26126B | 48:C745 00 08000000              | mov qword ptr ss:[rbp],8                
00007FF70D261273 | 48:C745 08 00000000              | mov qword ptr ss:[rbp+8],0        
```

Allocates 152 bytes (0x98) on the stack for:
- Vec internals (collector)
- String temporaries (str in the loop)
- RNG state / loop counters
- Establishes rbp as a base pointer to simplify access to local variables.

### RNG Initialization

```x86asm
00007FF70D26127B | E8 40040000                      | call <string-clone._$LT$rand..rngs..thr 
00007FF70D261280 | 48:89C6                          | mov rsi,rax                             
00007FF70D261283 | 48:8945 C8                       | mov qword ptr ss:[rbp-38],rax
```

`rsi` holds RNG state for later random sampling.

### Buffer check

```x86asm
00007FF70D261287 | 48:8D78 10                       | lea rdi,qword ptr ds:[rax+10]           
00007FF70D26128B | 48:8B88 50010000                 | mov rcx,qword ptr ds:[rax+150]          
00007FF70D261292 | 48:83F9 40                       | cmp rcx,40                              
00007FF70D261296 | 72 39                            | jb string-clone.7FF70D2612D1
```

### RNG Refill / Update

```x86asm
00007FF70D261298 | 48:8D8E 10010000                 | lea rcx,qword ptr ds:[rsi+110]          
00007FF70D26129F | 48:8B86 48010000                 | mov rax,qword ptr ds:[rsi+148]          
00007FF70D2612A6 | 48:85C0                          | test rax,rax                            
00007FF70D2612A9 | 7E 1C                            | jle string-clone.7FF70D2612C7           
00007FF70D2612AB | 48:05 00FFFFFF                   | add rax,FFFFFFFFFFFFFF00                
00007FF70D2612B1 | 48:8986 48010000                 | mov qword ptr ds:[rsi+148],rax          
00007FF70D2612B8 | BA 06000000                      | mov edx,6                               
00007FF70D2612BD | 49:89F8                          | mov r8,rdi                              
00007FF70D2612C0 | E8 6B040000                      | call <string-clone.rand_chacha::guts::refill_wide::hc43b38d823048efe>
00007FF70D2612C5 | EB 08                            | jmp string-clone.7FF70D2612CF           
00007FF70D2612C7 | 48:89FA                          | mov rdx,rdi                             
00007FF70D2612CA | E8 E1FDFFFF                      | call <string-clone.sub_7FF70D2610B0>    
```

### RNG Value Fetch / Scaling

```x86asm
00007FF70D2612CF | 31C9                             | xor ecx,ecx                             
00007FF70D2612D1 | 8B548E 10                        | mov edx,dword ptr ds:[rsi+rcx*4+10]
```

```x86asm
00007FF70D2612D5 | 48:8D41 01                       | lea rax,qword ptr ds:[rcx+1]            
00007FF70D2612D9 | 48:8986 50010000                 | mov qword ptr ds:[rsi+150],rax
```

```x86asm
00007FF70D2612E0 | 48:8D1C92                        | lea rbx,qword ptr ds:[rdx+rdx*4]        
00007FF70D2612E4 | 49:89DF                          | mov r15,rbx                             
00007FF70D2612E7 | 49:C1EF 20                       | shr r15,20
```

Scales RNG value to desired range

shr and lea are used for modulo/rejection to avoid bias

### RNG Rejection / Bias Check

```x86asm
00007FF70D2612EB | 83FB FC                          | cmp ebx,FFFFFFFC
00007FF70D2612EE | 72 5E                            | jb string-clone.7FF70D26134E            
```

### Loop Counter / Iteration Setup

```x86asm
00007FF70D2612F0 | 48:83F9 3F                       | cmp rcx,3F                              
00007FF70D2612F4 | 75 3C                            | jne string-clone.7FF70D261332           
00007FF70D2612F6 | 48:89F1                          | mov rcx,rsi                             
00007FF70D2612F9 | 48:81C1 10010000                 | add rcx,110                             
00007FF70D261300 | 48:8B86 48010000                 | mov rax,qword ptr ds:[rsi+148]          
00007FF70D261307 | 48:85C0                          | test rax,rax                            
00007FF70D26130A | 7E 1C                            | jle string-clone.7FF70D261328           
00007FF70D26130C | 48:05 00FFFFFF                   | add rax,FFFFFFFFFFFFFF00                
00007FF70D261312 | 48:8986 48010000                 | mov qword ptr ds:[rsi+148],rax          
00007FF70D261319 | BA 06000000                      | mov edx,6                               
00007FF70D26131E | 49:89F8                          | mov r8,rdi                              
00007FF70D261321 | E8 0A040000                      | call <string-clone.rand_chacha::guts::refill_wide::hc43b38d823048efe>
00007FF70D261326 | EB 08                            | jmp string-clone.7FF70D261330           
00007FF70D261328 | 48:89FA                          | mov rdx,rdi                             
00007FF70D26132B | E8 80FDFFFF                      | call <string-clone.sub_7FF70D2610B0>    
00007FF70D261330 | 31C0                             | xor eax,eax                             
00007FF70D261332 | 8B4C86 10                        | mov ecx,dword ptr ds:[rsi+rax*4+10]     
00007FF70D261336 | 48:FFC0                          | inc rax                                 
00007FF70D261339 | 48:8986 50010000                 | mov qword ptr ds:[rsi+150],rax          
00007FF70D261340 | 48:8D0489                        | lea rax,qword ptr ds:[rcx+rcx*4]        
00007FF70D261344 | 48:C1E8 20                       | shr rax,20                              
00007FF70D261348 | 01C3                             | add ebx,eax                             
00007FF70D26134A | 49:83D7 00                       | adc r15,0                               
00007FF70D26134E | 48:8B45 C8                       | mov rax,qword ptr ss:[rbp-38]           
00007FF70D261352 | 48:FF08                          | dec qword ptr ds:[rax]                  
00007FF70D261355 | 75 09                            | jne string-clone.7FF70D261360           
00007FF70D261357 | 48:8D4D C8                       | lea rcx,qword ptr ss:[rbp-38]           
00007FF70D26135B | E8 40030000                      | call <string-clone.alloc::rc::Rc$LT$T$C 
00007FF70D261360 | 41:83C7 05                       | add r15d,5                              
00007FF70D261364 | 45:31E4                          | xor r12d,r12d                           
00007FF70D261367 | BE 08000000                      | mov esi,8                               
00007FF70D26136C | 48:8D7D B0                       | lea rdi,qword ptr ss:[rbp-50]           
00007FF70D261370 | 48:8D5D C8                       | lea rbx,qword ptr ss:[rbp-38]           
00007FF70D261374 | 4C:8D6D A0                       | lea r13,qword ptr ss:[rbp-60]           
00007FF70D261378 | 45:31F6                          | xor r14d,r14d                           
00007FF70D26137B | EB 06                            | jmp string-clone.7FF70D261383           
00007FF70D26137D | 0F1F00                           | nop dword ptr ds:[rax],eax              
```
