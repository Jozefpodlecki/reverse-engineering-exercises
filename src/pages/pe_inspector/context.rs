use yew::prelude::*;
use crate::pages::pe_inspector::{ParsedPe, PeState};

#[derive(Clone, PartialEq)]
pub struct PeContext {
    pub raw: PeState,
    pub parsed: ParsedPe,
}

impl PeContext {
    pub fn new(raw: PeState, parsed: ParsedPe) -> Self {
        Self { raw, parsed }
    }
}