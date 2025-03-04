# Reverse Engineering Exercises

This repository is dedicated to reverse engineering exercises, focusing on using **Ghidra** for static analysis of binaries and **Rust** binaries for analysis.

## Tools

The following tools and versions are used in this exercise:

### 1. **Ghidra and Java**

- **Ghidra Version**: 11.3  
- **Java Version**: 23.0.2

Ghidra is used for disassembling and decompiling binaries to analyze their inner workings.

### 2. **Rust Compiler**

```sh
rustc 1.84.0 (9fc6b4312 2025-01-07)
binary: rustc
commit-hash: 9fc6b43126469e3858e2fe86cafb4f0fd5068869
commit-date: 2025-01-07
host: x86_64-pc-windows-msvc
release: 1.84.0
LLVM version: 19.1.5
```

### 3. System Info

`systeminfo | findstr /B /C:"OS"`

```sh
OS Name:                       Microsoft Windows 11 Home
OS Version:                    10.0.26100 N/A Build 26100
OS Manufacturer:               Microsoft Corporation
OS Configuration:              Standalone Workstation
OS Build Type:                 Multiprocessor Free
```

### Miscellaneous Resources

- [Ghidra Cheat Sheet: A helpful reference for using Ghidra in reverse engineering tasks.](https://ghidra-sre.org/CheatSheet.html)