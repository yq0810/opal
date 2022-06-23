
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error, Clone)]
pub enum QueryError {
    #[error("Unknown JavaScript Error")]
    QueryExecError(JsValue),

}
