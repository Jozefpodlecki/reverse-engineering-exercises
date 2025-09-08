# What Happens When You Double-Click an Executable in Windows

## 1. User Action and Shell Event

- The user double-clicks the `.exe` file icon in **Windows Explorer**.  
- Windows Explorer interprets this as a request to **“open” the file using the default verb**.  
- Internally, Explorer calls the `ShellExecuteEx` API to handle the action:

### Registry

```
Computer\HKEY_CLASSES_ROOT\exefile
FriendlyTypeName @%SystemRoot%\System32\shell32.dll,-10156
```

### Api

```c
ShellExecuteEx(&sei);
```

```cpp
typedef struct _SHELLEXECUTEINFOA {
  DWORD     cbSize;
  ULONG     fMask;
  HWND      hwnd;
  LPCSTR    lpVerb;
  LPCSTR    lpFile;
  LPCSTR    lpParameters;
  LPCSTR    lpDirectory;
  int       nShow;
  HINSTANCE hInstApp;
  void      *lpIDList;
  LPCSTR    lpClass;
  HKEY      hkeyClass;
  DWORD     dwHotKey;
  union {
    HANDLE hIcon;
    HANDLE hMonitor;
  } DUMMYUNIONNAME;
  HANDLE    hProcess;
} SHELLEXECUTEINFOA, *LPSHELLEXECUTEINFOA;
```

### EditFlags

EditFlags for `exefile`

```
38 07 00 00
```

```
3	0x08	FTA_NoEdit	Cannot rename in Explorer
4	0x10	FTA_NoRemove	Cannot delete
5	0x20	FTA_NoNewVerb	Hidden from “New” menu
8	0x100	FTA_NoEditDesc	Cannot edit description
9	0x200	FTA_NoEditIcon	Cannot edit icon
10	0x400	FTA_NoEditDflt	Cannot edit default verb
```