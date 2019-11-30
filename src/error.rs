use std::{error, fmt};

use crate::platform;

pub use crate::platform::OsError;

/// An error whose cause is outside of the crate's control.
#[derive(Debug)]
pub struct ExternalError {
    line: u32,
    file: String,
    ty: ErrorType,
}

impl ExternalError {
    pub fn new(line: u32, file: String, ty: ErrorType) -> Self {
        ExternalError { line, file, ty }
    }
}

/// The type of error.
#[derive(Debug)]
pub enum ErrorType {
    /// The operation is not supported by the backend.
    NotSupported,
    /// The OS cannot perform the operation.
    OsError(OsErrorWrapper),
    /// The requested config was not available.
    NoAvailableConfig,
    /// This crate's API was used in an invalid manner.
    BadApiUsage,
    /// The context you were using for this operation has been lost. This is
    /// generally non-recoverable.
    ContextLost,
}

/// The error type for when the OS cannot perform the requested operation.
#[derive(Debug)]
pub struct OsErrorWrapper {
    error: platform::OsError,
}

impl OsErrorWrapper {
    pub fn new(error: platform::OsError) -> Self {
        OsErrorWrapper { error }
    }
}

#[macro_export]
macro_rules! make_error {
    ($ty:expr) => {{
        crate::error::ExternalError::new(line!(), file!(), $ty)
    }};
}

impl fmt::Display for OsErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!(
            "Os Error: {}",
            self.error
        ))
    }
}

impl fmt::Display for ExternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!(
            "Error at{}:{}: {}",
            self.file, self.line, self.ty
        ))
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ErrorType::OsError(oew) => oew.fmt(f),
            ErrorType::NotSupported => f.pad("Operation not supported"),
            ErrorType::NoAvailableConfig => f.pad("No available config with the requested properties"),
            ErrorType::BadApiUsage => f.pad("This crate's API has been used incorrectly."),
            ErrorType::ContextLost => f.pad("Context lost."),
        }
    }
}

impl error::Error for OsError {}
impl error::Error for ExternalError {}
