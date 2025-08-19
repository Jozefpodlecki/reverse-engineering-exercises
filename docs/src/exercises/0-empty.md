# 0-empty

Navigate to the project folder and compile with optimizations:

```sh
cd 0-empty
rustc -C opt-level=3 -C debuginfo=0 -o main-o3.exe main.rs
```

Entry point of a PE executable:

```sh
(objdump -x main-o3.exe) | Select-Object -First 25

main-o3.exe:     file format pei-x86-64
main-o3.exe
architecture: i386:x86-64, flags 0x0000012f:
HAS_RELOC, EXEC_P, HAS_LINENO, HAS_DEBUG, HAS_LOCALS, D_PAGED
start address 0x0000000140014280

Characteristics 0x22
        executable
        large address aware

Time/Date               Tue Aug 19 22:58:47 2025
Magic                   020b    (PE32+)
MajorLinkerVersion      14
MinorLinkerVersion      44
SizeOfCode              0000000000014e00
SizeOfInitializedData   0000000000008c00
SizeOfUninitializedData 0000000000000000
AddressOfEntryPoint     0000000000014280
BaseOfCode              0000000000001000
ImageBase               0000000140000000
SectionAlignment        00001000
FileAlignment           00000200
MajorOSystemVersion     6
MinorOSystemVersion     0
```

Examine the first few instructions executed.

```sh
objdump -D main-o3.exe --start-address=0x140014280 --stop-address=0x140014290

main-o3.exe:     file format pei-x86-64


Disassembly of section .text:

0000000140014280 <.text+0x13280>:
   140014280:   48 83 ec 28             sub    $0x28,%rsp
   140014284:   e8 e3 02 00 00          call   0x14001456c
   140014289:   48 83 c4 28             add    $0x28,%rsp
   14001428d:   e9 72 fe ff ff          jmp    0x140014104
```

When decompiled in Ghidra, it appears as:

```c
ulong __cdecl mainCRTStartup(void *param_1)

{
  ulong uVar1;
  
  __security_init_cookie();
  uVar1 = __scrt_common_main_seh();
  return uVar1;
}

```

`__security_init_cookie` initializes the compiler's stack buffer overflow protection cookie by generating a pseudo-random value using the system time, thread ID, process ID, and performance counter; it ensures the cookie is never the default sentinel value and stores both the cookie and its complement for use in stack security checks.

[source](https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/security-init-cookie?view=msvc-170)

```c
void __cdecl __security_init_cookie(void)

{
  DWORD DVar1;
  _FILETIME local_res8;
  LARGE_INTEGER local_res10;
  _FILETIME local_18 [2];
  
  if (__security_cookie == 0x2b992ddfa232) {
    local_res8.dwLowDateTime = 0;
    local_res8.dwHighDateTime = 0;
    GetSystemTimeAsFileTime(&local_res8);
    local_18[0] = local_res8;
    DVar1 = GetCurrentThreadId();
    local_18[0] = (_FILETIME)((ulonglong)local_18[0] ^ (ulonglong)DVar1);
    DVar1 = GetCurrentProcessId();
    local_18[0] = (_FILETIME)((ulonglong)local_18[0] ^ (ulonglong)DVar1);
    QueryPerformanceCounter(&local_res10);
    __security_cookie =
         ((ulonglong)local_res10.s.LowPart << 0x20 ^
          CONCAT44(local_res10.s.HighPart,local_res10.s.LowPart) ^ (ulonglong)local_18[0] ^
         (ulonglong)local_18) & 0xffffffffffff;
    if (__security_cookie == 0x2b992ddfa232) {
      __security_cookie = 0x2b992ddfa233;
    }
  }
  __security_cookie_complement = ~__security_cookie;
  return;
}

```

`__scrt_common_main_seh` initializes the C runtime, acquires a startup lock to ensure thread-safe setup, runs global constructors via `_initterm_e` and `_initterm`, executes dynamic TLS init and destructor callbacks if present, retrieves `argc`, `argv`, and the environment, calls `main()` with these values, and then performs CRT and OS cleanup by calling `_cexit()` or `exit()` depending on whether the application is managed.

`__scrt_fastfail` triggers an immediate program termination using a **low-level fast-fail mechanism**. It checks for CPU support for the `fast fail` instruction, optionally invokes the debugger hook, captures the CPU context, performs stack unwinding if possible, and invokes `UnhandledExceptionFilter`. If no debugger handles the exception, it ensures the process terminates without normal cleanup, signaling a critical unrecoverable error.

`__scrt_initialize_crt` sets up CRT state for the module, marks DLL initialization if needed, detects CPU features via `__isa_available_init`, and determines whether the runtime environment should be initialized.


```c
int __cdecl __scrt_common_main_seh(void)

{
  char **_Argv;
  bool bVar1;
  bool bVar2;
  int iVar3;
  _func___cdecl_void_void_ptr_ulong_void_ptr **pp_Var4;
  char **_Env;
  undefined8 *puVar5;
  int *piVar6;
  
  bVar1 = __scrt_initialize_crt(exe);
  if (!bVar1) {
                    /* WARNING: Subroutine does not return */
    __scrt_fastfail(7);
  }
  bVar1 = false;
  bVar2 = __scrt_acquire_startup_lock();
  if (__scrt_current_native_startup_state == initializing) {
                    /* WARNING: Subroutine does not return */
    __scrt_fastfail(7);
  }
  if (__scrt_current_native_startup_state == uninitialized) {
    __scrt_current_native_startup_state = initializing;
    iVar3 = _initterm_e(&__xi_a,&__xi_z);
    if (iVar3 != 0) {
      return 0xff;
    }
    _initterm(&__xc_a,&__xc_z);
    __scrt_current_native_startup_state = initialized;
  }
  else {
    bVar1 = true;
  }
  __scrt_release_startup_lock(bVar2);
  pp_Var4 = __scrt_get_dyn_tls_init_callback();
  if ((*pp_Var4 != (_func___cdecl_void_void_ptr_ulong_void_ptr *)0x0) &&
     (bVar2 = __scrt_is_nonwritable_in_current_image(pp_Var4), bVar2)) {
    _guard_dispatch_icall_nop();
  }
  pp_Var4 = __scrt_get_dyn_tls_dtor_callback();
  if ((*pp_Var4 != (_func___cdecl_void_void_ptr_ulong_void_ptr *)0x0) &&
     (bVar2 = __scrt_is_nonwritable_in_current_image(pp_Var4), bVar2)) {
    _register_thread_local_exe_atexit_callback(*pp_Var4);
  }
  _Env = (char **)_get_initial_narrow_environment();
  puVar5 = (undefined8 *)__p___argv();
  _Argv = (char **)*puVar5;
  piVar6 = (int *)__p___argc();
  iVar3 = main(*piVar6,_Argv,_Env);
  bVar2 = __scrt_is_managed_app();
  if (bVar2) {
    if (!bVar1) {
      _cexit();
    }
    __scrt_uninitialize_crt(true,false);
    return iVar3;
  }

  exit(iVar3);
}

```

[lang_start_internal](https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs#L173)
[init](https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs#L111)

```c
int __cdecl main(int _Argc,char **_Argv,char **_Env)
{
  int extraout_EAX;
  
  std::rt::lang_start_internal();
  return extraout_EAX;
}

```

## Summary

The PE entry -> `mainCRTStartup` -> `__scrt_common_main_seh` -> `lang_start_internal` -> `main()` chain handles runtime initialization, CRT setup, and exception handling before executing the user-defined `main()`.
