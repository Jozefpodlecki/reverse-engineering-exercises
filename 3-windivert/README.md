# Rust Binary Analysis and Runtime Insights

High-level summary of the critical operations:

### 1. **Thread Spawning**

```
std::thread::Builder::spawn_unchecked((ulonglong *)&pointer,(byte *)&data_buffer,data_pointer);
```

### 2 ""MPSC Channel Data Reception""

```
std::sync::mpmc::list::Channel<T>::recv(&local_280, _Dst, (ulonglong)ppuVar1, 1000000000);
```

### 3. **Network Diverting and Blocking**

```
windivert::divert::WinDivert<>::network(&local_68,&local_140,0,5);
windivert::divert::blocking::_<>::recv(&local_118,&local_128,local_78,0xffff);
```

### 4. **Sending Data Through MPSC Channels**

After analysis, it's using the **List flavor** of the MPSC channel.

```
std::sync::mpmc::array::Channel<T>::send(&local_118,plVar1,(longlong *)&local_68,uVar2,1000000000);
std::sync::mpmc::list::Channel<T>::send(&local_118,(longlong)plVar1,&local_68);
std::sync::mpmc::zero::Channel<T>::send(&local_118,plVar1,(longlong *)&local_68,uVar2,1000000000);
```

### 5. **Memory Allocation**

```
std::alloc::__default_lib_allocator::__rust_alloc();
```