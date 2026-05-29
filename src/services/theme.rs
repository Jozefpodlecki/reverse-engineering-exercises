use std::{cell::{Ref, RefCell}, rc::Rc};

use log::*;
use yew::prelude::*;
use web_sys::{window, DomTokenList};

#[derive(Clone, PartialEq)]
pub struct ClassToggle(DomTokenList, Box<str>);

impl ClassToggle {
    pub fn new(list: DomTokenList, token: Box<str>) -> Self {
        Self(list, token)
    }

    pub fn contains(&self) -> bool {
        self.0.contains(&self.1)
    }

    pub fn toggle(&self) {
        unsafe { self.0.toggle(&self.1).unwrap_unchecked() };
    }
}

#[derive(Clone, PartialEq)]
pub struct ThemeController(ClassToggle, UseStateHandle<bool>);

impl ThemeController {
    pub fn new(toggle: ClassToggle, setter: UseStateHandle<bool>) -> Self {
        Self(toggle, setter)
    }

    pub fn contains(&self) -> bool {
        self.0.contains()
    }

    pub fn toggle(&self) {
        self.0.toggle();
        let new = *self.1;
        self.1.set(!new);
    }
}
