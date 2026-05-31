use std::collections::HashMap;

#[derive(Clone)]
pub struct Iommu {
    pub enabled: bool,
    pub translations: HashMap<u64, IoTranslation>, // Device address -> Physical address
    pub domain_id: u32,
}

#[derive(Clone)]
pub struct IoTranslation {
    pub physical_address: u64,
    pub permissions: IommuPermissions,
    pub size: u64,
}

#[derive(Clone)]
pub struct IommuPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Iommu {
    pub fn new() -> Self {
        Self {
            enabled: false,
            translations: HashMap::new(),
            domain_id: 0,
        }
    }
    
    pub fn translate(&self, device_address: u64) -> Option<u64> {
        if !self.enabled {
            return Some(device_address); // Identity mapping when disabled
        }
        
        // Find translation for this address
        for (start, trans) in &self.translations {
            if device_address >= *start && device_address < *start + trans.size {
                let offset = device_address - *start;
                return Some(trans.physical_address + offset);
            }
        }
        
        None
    }
    
    pub fn map(&mut self, device_address: u64, physical_address: u64, size: u64, permissions: IommuPermissions) {
        self.translations.insert(device_address, IoTranslation {
            physical_address,
            permissions,
            size,
        });
    }
}