# Environment Details

The exact output of compiled binaries will vary slightly depending on your compiler version, target architecture, and operating system. To make results reproducible, here are the specifics of the environment I’ll be using throughout this book:

```sh
rustc --version --verbose
rustc 1.88.0 (6b00bc388 2025-06-23)
binary: rustc
commit-hash: 6b00bc3880198600130e1cf62b8f8a93494488cc
commit-date: 2025-06-23
host: x86_64-pc-windows-msvc
release: 1.88.0
LLVM version: 20.1.5
```

> The host field (x86_64-pc-windows-msvc) indicates the target platform: 64-bit Intel/AMD (x86_64), Windows OS (windows), and MSVC toolchain (msvc). This affects calling conventions, linking, and debug info (like PDB files)

```sh
systeminfo | findstr /B /C:"OS"
OS Name:                       Microsoft Windows 11 Home
OS Version:                    10.0.26100 N/A Build 26100
OS Manufacturer:               Microsoft Corporation
OS Configuration:              Standalone Workstation
OS Build Type:                 Multiprocessor Free
```