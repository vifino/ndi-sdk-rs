use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum NDIError {
    #[error("Failed to load NDI Library: {0}")]
    LoadingError(#[from] libloading::Error),

    #[error("NDIlib_v5_load() returned NULL - no NDIlib_v5 returned!")]
    LoadV5Failed,

    #[error("NDIlib_v5->initialize() failed, is the CPU supported?")]
    InitializeFailed,

    #[error("NDIlib_v5 struct is missing symbol: {0}")]
    MissingSymbolV5(&'static str),

    #[error("NDIlib_v5 function returned a NULL pointer unexpectedly: {0}")]
    UnexpectedNullPointer(&'static str),

    #[error("Failed to parse C String as valid UTF-8: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("String contains NULL bytes, cannot convert to C String")]
    InvalidCString,
}

pub type NDIResult<T> = std::result::Result<T, NDIError>;
