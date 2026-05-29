use std::{error::Error, fmt::{self, Debug, Display, Formatter}};

use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::models::AppError;

// #[derive(Clone, Default, Debug, PartialEq)]
// pub enum AppState {
//     #[default]
//     Loading,
//     Transitioning(Social),
//     Error(AppError),
//     Loaded(Social)
// }