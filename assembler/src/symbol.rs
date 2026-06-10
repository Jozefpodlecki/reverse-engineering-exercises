#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;

pub trait SymbolResolver {
    fn lookup(&self, name: &str) -> Option<usize>;
}

#[cfg(feature = "alloc")]
impl SymbolResolver for BTreeMap<String, usize> {
    fn lookup(&self, name: &str) -> Option<usize> {
        self.get(name).copied()
    }
}