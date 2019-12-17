use std::{error, fmt};

use crate::platform;

pub use crate::platform::OsError;

/// An error whose cause is outside of the crate's control.
#[derive(Clone, Debug)]
pub struct Error {
    pub line: u32,
    pub file: &'static str,
    pub ty: ErrorType,
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
    NotSupported(String),
    /// The robustness is not supported by the backend.
    RobustnessNotSupported,
    /// The opengl version is not supported by the backend.
    OpenGlVersionNotSupported,
    /// The OS cannot perform the operation.
    OsError(OsErrorWrapper),
    /// The requested config was not available.
    NoAvailableConfig,
    /// This crate's API was used in an invalid manner.
    BadApiUsage(String),
    /// The context you were using for this operation has been lost. This is
    /// generally non-recoverable.
    ContextLost,

    /// Multiple errors happened.
    Multiple(Vec<Box<Error>>),
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
macro_rules! append_errors {
    ($err1:expr, $err2:expr) => {{
        use winit_types::error::ErrorType;
        match ($err1.ty, $err2.ty) {
            (ErrorType::Multiple(errs1), ErrorType::Multiple(errs2)) => make_error!(ErrorType::Multiple(errs1.drain(..).chain(errs2.drain(..)).collect())),
            (ErrorType::Multiple(errs), _) => { errs.push(Box::new($err2)); $err1 },
            (_, ErrorType::Multiple(errs)) => { errs.push(Box::new($err1)); $err2 },
            (_, _) => make_error!(ErrorType::Multiple(vec![Box::new($err1), Box::new($err2)])),
        }
    }};
    ($err:expr) => ($err);
    ($err1:expr, $err2:expr, $($errrem:expr),+) => {{
        append_errors!(append_errors!($err1, $err2), append_errors!($($errrem),+))
    }};
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
            ErrorType::NotSupported(t) => f.pad(&format!("Operation not supported: {}", t)),
            ErrorType::RobustnessNotSupported => f.pad("Robustness not supported"),
            ErrorType::OpenGlVersionNotSupported => f.pad("OpenGL version not supported"),
            ErrorType::NoAvailableConfig => {
                f.pad("No available config with the requested properties")
            }
            ErrorType::BadApiUsage(t) => f.pad(&format!(
                "This crate's API has been used incorrectly: {}",
                t
            )),
            ErrorType::ContextLost => f.pad("Context lost."),
            ErrorType::Multiple(errs) => f.pad(&format!("Multiple errors: {:?}", errs)),
        }
    }
}

impl error::Error for OsErrorWrapper {}
impl error::Error for Error {}
