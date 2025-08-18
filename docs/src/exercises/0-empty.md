# 0-empty

Navigate to the project folder and compile with optimizations:

```sh
cd 0-empty
rustc -C opt-level=3 -C debuginfo=0 -o main-o3.exe main.rs
```

The PE entry point, when decompiled in Ghidra, appears as:

```c
ulong __cdecl mainCRTStartup(void *param_1)

{
  ulong uVar1;
  
  __security_init_cookie();
  uVar1 = __scrt_common_main_seh();
  return uVar1;
}

```

`__security_init_cookie`

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

`__scrt_common_main_seh`

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

```c
int __cdecl main(int _Argc,char **_Argv,char **_Env)
{
  int extraout_EAX;
  
  std::rt::lang_start_internal();
  return extraout_EAX;
}

```

### Prologue / Stack Setup

- Saves registers (RBX, RSI, RDI) to stack.
- Allocates stack space for local variables.

### CRT Initialization

- Calls `__scrt_initialize_crt` to set up the CRT.
- If initialization fails, jumps to fastfail.

### Startup Lock

- Calls `__scrt_acquire_startup_lock` to ensure only one thread initializes CRT globally.
- Sets/updates startup state (`__scrt_current_native_startup_state`).

### Global and C++ Constructors

- `_initterm_e` → initializes constructors that can throw exceptions.
- `_initterm` → initializes constructors that cannot throw exceptions.

### Release Startup Lock

- Calls `__scrt_release_startup_lock` to allow other threads to proceed.

### Thread-Local Storage (TLS) Setup

- Calls `__scrt_get_dyn_tls_init_callback` and `_register_thread_local_exe_atexit_callback` to set up TLS initializers and destructors.

### Environment / Arguments Setup

- `_get_initial_narrow_environment` → fetches environment variables.
- `__p___argv` and `__p___argc` → get command-line arguments.

### Call User main()

- Sets up arguments (`argc`, `argv`, environment) and calls `main`.
- Stores `main`’s return value in EBX.

### CRT Cleanup / Exit

- Checks if the application is managed (.NET) or native.
- Calls `_cexit` or `_c_exit` to finalize CRT.
- Calls `__scrt_uninitialize_crt` to clean up runtime resources.

### Return

- Restores saved registers.
- Returns the `main()` return value in EAX.

### Failure Paths

- Calls `__scrt_fastfail` if CRT initialization fails or startup state is invalid.

---

## Summary

The PE entry → `mainCRTStartup` → `__scrt_common_main_seh` → `main()` chain handles runtime initialization, CRT setup, and exception handling before executing the user-defined `main()`.


**Summary:**  

The PE entry → mainCRTStartup → __scrt_common_main_seh → Rust main() chain handles runtime initialization, CRT setup, and exception handling before executing the user-defined main().

