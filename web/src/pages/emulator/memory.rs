use std::collections::BTreeMap;

use crate::pages::emulator::error::MemoryError;

#[derive(Clone, PartialEq)]
pub struct MemoryManager {
    pages: BTreeMap<u64, Page>,
    page_size: u64,
}

#[derive(Clone, PartialEq)]
pub struct Page {
    data: Vec<u8>,
    readable: bool,
    writable: bool,
    executable: bool,
}

impl Page {
    fn new() -> Self {
        Self {
            data: vec![0; 4096],
            readable: true,
            writable: true,
            executable: true,
        }
    }
    
    fn new_protected(readable: bool, writable: bool, executable: bool) -> Self {
        Self {
            data: vec![0; 4096],
            readable,
            writable,
            executable,
        }
    }
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            pages: BTreeMap::new(),
            page_size: 4096,
        }
    }
    
    pub fn map_page(&mut self, base_address: u64) {
        let page_start = base_address & !(self.page_size - 1);
        
        if !self.pages.contains_key(&page_start) {
            self.pages.insert(page_start, Page::new());
        }
    }
    
    pub fn map_page_with_protection(&mut self, base_address: u64, readable: bool, writable: bool, executable: bool) {
        let page_start = base_address & !(self.page_size - 1);
        
        if !self.pages.contains_key(&page_start) {
            self.pages.insert(page_start, Page::new_protected(readable, writable, executable));
        }
    }
    
    pub fn unmap_page(&mut self, base_address: u64) {
        let page_start = base_address & !(self.page_size - 1);
        self.pages.remove(&page_start);
    }
    
    pub fn protect_page(&mut self, base_address: u64, readable: bool, writable: bool, executable: bool) -> Result<(), MemoryError> {
        let page_start = base_address & !(self.page_size - 1);
        
        if let Some(page) = self.pages.get_mut(&page_start) {
            page.readable = readable;
            page.writable = writable;
            page.executable = executable;
            Ok(())
        } else {
            Err(MemoryError::PageFault(base_address))
        }
    }
    
    pub fn load_bytes(&mut self, address: u64, data: &[u8]) -> Result<(), MemoryError> {
        self.write_bytes(address, data)
    }
    
    fn get_page_mut(&mut self, address: u64) -> Option<&mut Page> {
        let page_start = address & !(self.page_size - 1);
        self.pages.get_mut(&page_start)
    }
    
    fn get_page(&self, address: u64) -> Option<&Page> {
        let page_start = address & !(self.page_size - 1);
        self.pages.get(&page_start)
    }
    
    pub fn read_u8(&self, address: u64) -> Result<u8, MemoryError> {
        let page_size = self.page_size;
        
        if let Some(page) = self.get_page(address) {
            if !page.readable {
                return Err(MemoryError::ProtectionViolation(address));
            }
            
            let offset = (address & (page_size - 1)) as usize;
            
            if offset < page.data.len() {
                return Ok(page.data[offset]);
            }
        }
        
        Err(MemoryError::PageFault(address))
    }
    
    pub fn write_u8(&mut self, address: u64, value: u8) -> Result<(), MemoryError> {
        let page_size = self.page_size;
        
        if let Some(page) = self.get_page_mut(address) {
            if !page.writable {
                return Err(MemoryError::ProtectionViolation(address));
            }
            
            let offset = (address & (page_size - 1)) as usize;
            
            if offset < page.data.len() {
                page.data[offset] = value;
                return Ok(());
            }
        }
        
        Err(MemoryError::PageFault(address))
    }
    
    pub fn read_u16(&self, address: u64) -> Result<u16, MemoryError> {
        let mut bytes = [0u8; 2];
        
        for i in 0..2 {
            bytes[i] = self.read_u8(address + i as u64)?;
        }
        
        Ok(u16::from_le_bytes(bytes))
    }
    
    pub fn write_u16(&mut self, address: u64, value: u16) -> Result<(), MemoryError> {
        let bytes = value.to_le_bytes();
        
        for i in 0..2 {
            self.write_u8(address + i as u64, bytes[i])?;
        }
        
        Ok(())
    }
    
    pub fn read_u32(&self, address: u64) -> Result<u32, MemoryError> {
        let mut bytes = [0u8; 4];
        
        for i in 0..4 {
            bytes[i] = self.read_u8(address + i as u64)?;
        }
        
        Ok(u32::from_le_bytes(bytes))
    }
    
    pub fn write_u32(&mut self, address: u64, value: u32) -> Result<(), MemoryError> {
        let bytes = value.to_le_bytes();
        
        for i in 0..4 {
            self.write_u8(address + i as u64, bytes[i])?;
        }
        
        Ok(())
    }
    
    pub fn read_u64(&self, address: u64) -> Result<u64, MemoryError> {
        let mut bytes = [0u8; 8];
        
        for i in 0..8 {
            bytes[i] = self.read_u8(address + i as u64)?;
        }
        
        Ok(u64::from_le_bytes(bytes))
    }
    
    pub fn write_u64(&mut self, address: u64, value: u64) -> Result<(), MemoryError> {
        let bytes = value.to_le_bytes();
        
        for i in 0..8 {
            self.write_u8(address + i as u64, bytes[i])?;
        }
        
        Ok(())
    }
    
    pub fn read_u128(&self, address: u64) -> Result<u128, MemoryError> {
        let mut bytes = [0u8; 16];
        
        for i in 0..16 {
            bytes[i] = self.read_u8(address + i as u64)?;
        }
        
        Ok(u128::from_le_bytes(bytes))
    }
    
    pub fn write_u128(&mut self, address: u64, value: u128) -> Result<(), MemoryError> {
        let bytes = value.to_le_bytes();
        
        for i in 0..16 {
            self.write_u8(address + i as u64, bytes[i])?;
        }
        
        Ok(())
    }
    
    pub fn read_u256(&self, address: u64) -> Result<[u8; 32], MemoryError> {
        let mut bytes = [0u8; 32];
        
        for i in 0..32 {
            bytes[i] = self.read_u8(address + i as u64)?;
        }
        
        Ok(bytes)
    }
    
    pub fn write_u256(&mut self, address: u64, value: &[u8; 32]) -> Result<(), MemoryError> {
        for i in 0..32 {
            self.write_u8(address + i as u64, value[i])?;
        }
        
        Ok(())
    }
    
    pub fn read_bytes(&self, address: u64, buffer: &mut [u8]) -> Result<(), MemoryError> {
        for i in 0..buffer.len() {
            buffer[i] = self.read_u8(address + i as u64)?;
        }
        
        Ok(())
    }
    
    pub fn write_bytes(&mut self, address: u64, data: &[u8]) -> Result<(), MemoryError> {
        for i in 0..data.len() {
            self.write_u8(address + i as u64, data[i])?;
        }
        
        Ok(())
    }
    
    pub fn get_dump(&self, address: u64, size: usize) -> Result<Vec<u8>, MemoryError> {
        let mut result = vec![0u8; size];
        self.read_bytes(address, &mut result)?;
        Ok(result)
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}