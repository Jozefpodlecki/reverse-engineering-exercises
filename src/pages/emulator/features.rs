#[derive(Clone)]
pub struct EmulatorFeatures {
    pub dma: bool,
    pub iommu: bool,
    pub numa: bool,
}

impl Default for EmulatorFeatures {
    fn default() -> Self {
        Self {
            dma: false,   // Enable for disk/network emulation
            iommu: false, // Enable for VFIO/passthrough
            numa: false,  // Enable for multi-socket guests
        }
    }
}