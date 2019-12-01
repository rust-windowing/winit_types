use std::{error, fmt};

use crate::platform;

pub use crate::platform::OsError;

/// An error whose cause is outside of the crate's control.
#[derive(Clone, Debug)]
pub struct Error {
    line: u32,
    file: &'static str,
    ty: ErrorType,
}

impl Error {
    pub fn new(line: u32, file: &'static str, ty: ErrorType) -> Self {
        Error { line, file, ty }
    }
}

/// The type of error.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
    ($errty:expr) => {{
        winit_types::error::Error::new(line!(), file!(), $errty)
    }};
}

#[macro_export]
macro_rules! make_oserror {
    ($err:expr) => {{
        make_error!(winit_types::error::ErrorType::OsError(
            winit_types::error::OsErrorWrapper::new($err)
        ))
    }};
}

impl fmt::Display for OsErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!("Os Error: {}", self.error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!("Error at{}:{}: {}", self.file, self.line, self.ty))
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ErrorType::OsError(oew) => oew.fmt(f),
            ErrorType::NotSupported => f.pad("Operation not supported"),
            ErrorType::NoAvailableConfig => {
                f.pad("No available config with the requested properties")
            }
            ErrorType::BadApiUsage => f.pad("This crate's API has been used incorrectly."),
            ErrorType::ContextLost => f.pad("Context lost."),
        }
    }
}

impl error::Error for OsErrorWrapper {}
impl error::Error for Error {}
