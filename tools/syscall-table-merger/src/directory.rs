use anyhow::Result;
use tantivy::HasLen;
use tantivy::directory::*;
use tantivy::directory::error::{OpenReadError, OpenWriteError, DeleteError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Cursor, Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TarDirectory(HashMap<PathBuf, Arc<[u8]>>);

impl TarDirectory {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut archive = tar::Archive::new(file);
        let mut files = HashMap::new();
        
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into();
            let mut data = Vec::new();
            entry.read_to_end(&mut data)?;
            files.insert(path, data.into());
        }
        
        Ok(Self(files))
    }
    
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let cursor = Cursor::new(data);
        let mut archive = tar::Archive::new(cursor);
        let mut files = HashMap::new();
        
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into();
            let mut data = Vec::new();
            entry.read_to_end(&mut data)?;
            files.insert(path, data.into());
        }
        
        Ok(Self(files))
    }
}

impl Directory for TarDirectory {
    fn get_file_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>, OpenReadError> {

        let data = self.0.get(path).ok_or_else(|| OpenReadError::FileDoesNotExist(path.to_path_buf()))?;
        
        Ok(Arc::new(TarFile(data.clone())))
    }
    
    fn delete(&self, _: &Path) -> Result<(), DeleteError> {
        Ok(())
    }
    
    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        Ok(self.0.contains_key(path))
    }
    
    fn open_write(&self, _: &Path) -> Result<WritePtr, OpenWriteError> {
        let writer = Box::new(ReadOnlyWriter);
        Ok(BufWriter::new(writer))
    }
    
    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        self.0.get(path)
            .cloned()
            .map(|pr| pr.to_vec())
            .ok_or_else(|| OpenReadError::FileDoesNotExist(path.to_path_buf()))
    }
    
    fn atomic_write(&self, _: &Path, _data: &[u8]) -> io::Result<()> {
        Err(Error::new(ErrorKind::PermissionDenied, "read-only"))
    }
    
    fn sync_directory(&self) -> io::Result<()> {
        Ok(())
    }
    
    fn watch(&self, _watch_callback: WatchCallback) -> tantivy::Result<WatchHandle> {
        Ok(WatchHandle::empty())
    }
}

#[derive(Debug, Clone)]
pub struct TarFile(Arc<[u8]>);

impl HasLen for TarFile {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FileHandle for TarFile {
    fn read_bytes(&self, range: Range<usize>) -> io::Result<OwnedBytes> {
        if range.end > self.0.len() {
            return Err(Error::new(ErrorKind::UnexpectedEof, "end of file"));
        }
        
        Ok(OwnedBytes::new(self.0[range].to_vec()))
    }
}

pub struct ReadOnlyWriter;

impl Write for ReadOnlyWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "read-only"))
    }
    
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for ReadOnlyWriter {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "read-only"))
    }
}

impl TerminatingWrite for ReadOnlyWriter {
    fn terminate_ref(&mut self, _: AntiCallToken) -> io::Result<()> {
        Ok(())
    }
}