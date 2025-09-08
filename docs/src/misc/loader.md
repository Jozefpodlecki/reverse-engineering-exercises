```
C:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\um
```

`ntdll.LdrGetProcedureAddressForCaller`

- Internal loader function that resolves function pointers in DLLs.
- Equivalent to GetProcAddress, but with extra security / caller tracking.
- Used during import table processing or when dynamic function lookups occur.

`ntdll.RtlDeleteElementGenericTableAvlEx`

- Deletes an element from an AVL-based generic table.
- Windows often uses these tables internally to track loaded modules, TLS slots, or runtime resources.
- Seeing this in a loader trace usually indicates cleanup or deallocation during module setup or teardown.

`kernelbase.GetApplicationRestartSettings`

- Retrieves settings related to Windows application restart / recovery.
- Called internally during process initialization to see if the OS wants to auto-restart a crashed application.

`ntdll.RtlEncodeRemotePointer`
- Obfuscates or encodes pointers that may be shared across processes or stored in global tables.
- Prevents accidental or malicious misuse of raw pointers (security measure).
- Usually happens after resolving function addresses or storing thread-local data.

`ntdll.RtlAnsiStringToUnicodeString`

- Converts an ANSI string to a Unicode (UTF-16) string.
- Used during DLL loading or manifest/registry string parsing, because many Windows APIs expect Unicode internally.

`ntdll.RtlAcquireSRWLockExclusive`

- Acquires a Slim Reader/Writer lock exclusively.
- Prevents multiple threads from modifying shared resources at the same time.
- Often used in runtime and loader code for:
-- AVL tables
-- Loader metadata
-- Heap or TLS structures

`ntdll.RtlUTF8ToUnicode`

- Converts a UTF-8 string to Unicode (UTF-16).

`ntdll.TpCallbackMayRunLong`

- Part of Windows Thread Pool (Tp) API.
- Called internally when a thread pool callback might take a long time to execute.
- The runtime uses this to decide whether to inject additional threads or adjust scheduling.

`RtlInsertElementGenericTableFullAvl`

- Inserts an element into a generic AVL tree in memory.
- The “FullAvl” version handles both insertion and balancing automatically.

`ntdll.RtlRaiseStatus`
- Raises a Windows NTSTATUS exception.
- Converts a status code (like an error or system signal) into an exception that can be caught by SEH handlers.

`ntdll.RtlUserThreadStart`
- Entry point for new user-mode threads.
- Windows sets up a thread so that when it begins execution, it runs RtlUserThreadStart.
- Responsible for:
-- Setting up thread-local storage (TLS)
-- Calling your thread function (LPTHREAD_START_ROUTINE)
-- Handling structured exception handling (SEH) for the thread

`kernel32.BaseThreadInitThunk`
- Called by the OS when a thread starts.
- Sets up the thread environment before calling RtlUserThreadStart.
- Handles:
-- Register initialization
-- Stack setup
-- Exception frame setup

### `ntdll.NtMapViewOfSection`

```c
NTSYSAPI NTSTATUS ZwMapViewOfSection(
  [in]                HANDLE          SectionHandle,
  [in]                HANDLE          ProcessHandle,
  [in, out]           PVOID           *BaseAddress,
  [in]                ULONG_PTR       ZeroBits,
  [in]                SIZE_T          CommitSize,
  [in, out, optional] PLARGE_INTEGER  SectionOffset,
  [in, out]           PSIZE_T         ViewSize,
  [in]                SECTION_INHERIT InheritDisposition,
  [in]                ULONG           AllocationType,
  [in]                ULONG           Win32Protect
);
```

Maps a view of a section into the virtual address space of a subject process.

### `ntdll.LdrGetDllHandleByMapping`

Given a mapped memory region, tries to resolve if it corresponds to a loaded DLL.

Helps the loader avoid re-loading the same module.

### `ntdll.RtlDeactivateActivationContextUnsafeFast`

Cleans up activation contexts (side-by-side assemblies, manifests, COM contexts).

You see this around DLL load/unload sequences.

"UnsafeFast" = lightweight version without full safety checks.

### `ntdll.LdrGetDllHandleEx`

Higher-level helper for finding a DLL handle by name or characteristics.

Often used before LdrLoadDll.

### `ntdll.LdrLoadDll`

Main function to load a DLL (after checking handles/mappings).

Calls NtMapViewOfSection internally if the DLL isn’t already mapped.

Also processes imports, TLS callbacks, and entrypoints (DllMain).

### `ntdll.RtlFindClearBitsAndSet`

A low-level runtime routine that manipulates a bitmap (finds a sequence of 0s, flips them to 1s).

Used internally by the loader and memory manager to track free/used slots (e.g., TLS slots, heap allocations).

### `ntdll.EtwEventWriteNoRegistration`

Event Tracing for Windows (ETW).

Writes events even if no provider is registered.

Usually system bookkeeping — not critical to program flow, but shows the loader/system is logging activity.

### `ntdll.LdrInitializeThunk`

Runs when a new thread starts (especially the first thread of a process).
Final loader setup: initializes loader state, processes TLS, resolves imports.
Then jumps to program’s real entrypoint (e.g., main, WinMain, or DllMain for DLL entry).