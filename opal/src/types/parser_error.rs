use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ParserError {
    #[error("Duration Error")]
    DurationError,
    #[error("Bool Error")]
    BoolError,
}
