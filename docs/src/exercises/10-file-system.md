

[source](https://github.com/Jozefpodlecki/reverse-engineering-exercises/blob/main/exercises/file-system/main.rs)

`fn Handle.synchronous_write(&self, buf: &[u8], offset: Option<u64>) -> io::Result<usize>`

```
NtWriteFile
WaitForSingleObject
```

`fn Handle.synchronous_read(&self, buf: *mut mem::MaybeUninit<u8>, len: usize, offset: Option<u64> ) -> io::Result<usize>`

```
NtReadFile
WaitForSingleObject
```