use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error, Clone)]
pub enum ParserError {
    #[error("Duration Error")]
    DurationError,
}
