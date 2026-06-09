use std::collections::BTreeMap;

#[derive(Clone, PartialEq)]
pub struct Memory {
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
}

impl Memory {
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

    pub fn unmap_page(&mut self, base_address: u64) {
        let page_start = base_address & !(self.page_size - 1);
        self.pages.remove(&page_start);
    }

    fn get_page(&self, address: u64) -> Option<&Page> {
        let page_start = address & !(self.page_size - 1);
        self.pages.get(&page_start)
    }

    fn get_page_mut(&mut self, address: u64) -> Option<&mut Page> {
        let page_start = address & !(self.page_size - 1);
        self.pages.get_mut(&page_start)
    }

    pub fn read_u8(&self, address: u64) -> u8 {
        if let Some(page) = self.get_page(address) {
            let offset = (address & (self.page_size - 1)) as usize;
            if offset < page.data.len() && page.readable {
                return page.data[offset];
            }
        }
        0
    }

    pub fn write_u8(&mut self, address: u64, value: u8) {
        let page_size = self.page_size;
        if let Some(page) = self.get_page_mut(address) {
            let offset = (address & (page_size - 1)) as usize;
            if offset < page.data.len() && page.writable {
                page.data[offset] = value;
            }
        }
    }

    pub fn read_u16(&self, address: u64) -> u16 {
        let mut bytes = [0u8; 2];
        for i in 0..2 {
            bytes[i] = self.read_u8(address + i as u64);
        }
        u16::from_le_bytes(bytes)
    }

    pub fn write_u16(&mut self, address: u64, value: u16) {
        let bytes = value.to_le_bytes();
        for i in 0..2 {
            self.write_u8(address + i as u64, bytes[i]);
        }
    }

    pub fn read_u32(&self, address: u64) -> u32 {
        let mut bytes = [0u8; 4];
        for i in 0..4 {
            bytes[i] = self.read_u8(address + i as u64);
        }
        u32::from_le_bytes(bytes)
    }

    pub fn write_u32(&mut self, address: u64, value: u32) {
        let bytes = value.to_le_bytes();
        for i in 0..4 {
            self.write_u8(address + i as u64, bytes[i]);
        }
    }

    pub fn read_u64(&self, address: u64) -> u64 {
        let mut bytes = [0u8; 8];
        for i in 0..8 {
            bytes[i] = self.read_u8(address + i as u64);
        }
        u64::from_le_bytes(bytes)
    }

    pub fn write_u64(&mut self, address: u64, value: u64) {
        let bytes = value.to_le_bytes();
        for i in 0..8 {
            self.write_u8(address + i as u64, bytes[i]);
        }
    }

    pub fn get_dump(&self, address: u64, size: usize) -> Vec<u8> {
        let mut result = vec![0u8; size];
        for i in 0..size {
            result[i] = self.read_u8(address + i as u64);
        }
        result
    }

    pub fn allocate_stack(&mut self, base: u64, size: usize) {
        let pages_needed = (size + self.page_size as usize - 1) / self.page_size as usize;
        for i in 0..pages_needed {
            let addr = base - (i as u64 * self.page_size);
            self.map_page(addr);
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut mem = Self::new();
        mem.allocate_stack(0x7FFFFFFF0000, 0x100000);
        mem
    }
}