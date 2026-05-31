
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryError {
    PageFault(u64),
    ProtectionViolation(u64),
    UnalignedAccess(u64),
}