use std::{error, fmt};

use crate::platform;

pub use crate::platform::OsError;

#[macro_export]
macro_rules! make_error {
    ($errty:expr) => {{
        winit_types::error::Error::new(line!(), file!(), $errty)
    }};
}

macro_rules! lmake_error {
    ($errty:expr) => {{
        Error::new(line!(), file!(), $errty)
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

    pub fn append(&mut self, o: Error) {
        match (self, o) {
            (
                Error {
                    ty: ErrorType::Multiple(ref mut errs1),
                    ..
                },
                Error {
                    ty: ErrorType::Multiple(ref mut errs2),
                    ..
                },
            ) => {
                errs1.append(errs2);
            }
            (
                err1 @ Error {
                    ty: ErrorType::Multiple(_),
                    ..
                },
                err2,
            ) => {
                err1.ty.append(err2);
            }
            (
                err1,
                mut err2 @ Error {
                    ty: ErrorType::Multiple(_),
                    ..
                },
            ) => {
                std::mem::swap(err1, &mut err2);
                err1.ty.append(err2);
            }
            (err1, err2) => {
                let mut new_err1 = lmake_error!(ErrorType::Multiple(vec![]));
                std::mem::swap(err1, &mut new_err1);
                err1.ty.append(new_err1);
                err1.ty.append(err2);
            }
        }
    }
}

/// The type of bits is not supported
#[derive(Clone, Debug)]
pub enum BitType {
    /// The requested number of stencil bits is not supported
    Stencil,
    /// The requested number of depth bits is not supported
    Depth,
    /// The requested number of color bits is not supported
    Color,
    /// The requested number of alpha bits is not supported
    Alpha,
}

/// The type of error.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ErrorType {
    /// The operation is not supported by the backend.
    NotSupported(String),
    /// The robustness is not supported by the backend.
    RobustnessNotSupported,
    /// The opengl version is not supported by the backend.
    OpenGlVersionNotSupported,
    /// The swap control range is not supported.
    SwapControlRangeNotSupported,
    /// Adaptive swap control is not supported.
    AdaptiveSwapControlNotSupported,
    /// The requested flush control is not supported.
    FlushControlNotSupported,
    /// The requested floating point surface mode is not supported.
    FloatingPointSurfaceNotSupported,
    /// The requested sRGB surface mode is not supported.
    SrgbSurfaceNotSupported,
    /// The requested hardware acceleration mode is not supported.
    HardwareAccelerationNotSupported,
    /// The requested surface types were not supported.
    SurfaceTypesNotSupported {
        change_pbuffer: bool,
        change_pixmap: bool,
        change_surfaceless: bool,
        change_window: bool,
    },
    /// Stereoscopy is not supported.
    StereoscopyNotSupported,
    /// The requested double buffering mode is not supported.
    DoubleBufferNotSupported,
    /// The requested multisampling mode is not supported.
    MultisamplingNotSupported,
    /// The requested number of bits is not supported, with a suggestion of what
    /// to change it to.
    NumberOfBitsNotSupported(BitType, u8),
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

impl ErrorType {
    #[inline]
    /// You can't put the match statment in the function else the borrow checker dies.
    fn append(&mut self, err: Error) {
        match self {
            ErrorType::Multiple(ref mut errs) => errs.push(Box::new(err)),
            _ => unreachable!(),
        };
    }
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

impl fmt::Display for OsErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!("Os Error: {}", self.error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!(
            "Error at {}:{}: {}",
            self.file, self.line, self.ty
        ))
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.pad(&format!("{:?}", self))
    }
}

impl error::Error for OsErrorWrapper {}
impl error::Error for Error {}
