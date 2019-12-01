#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]

use glutin_x11_sym::x11_dl::error::OpenError;
use smithay_client_toolkit::reexports::client::ConnectError;

use std::fmt;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum OsError {
    XError(XError),
    XMisc(&'static str),
    XNotSupported(XNotSupported),
    // For some reason is not clone, so just Arc it.
    WaylandConnectError(Arc<ConnectError>)
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OsError::XError(e) => f.pad(&e.description),
            OsError::XMisc(e) => f.pad(e),
            OsError::XNotSupported(e) => e.fmt(f),
            OsError::WaylandConnectError(e) => e.fmt(f),
        }
    }
}

/// Error triggered by xlib.
#[derive(Debug, Clone)]
pub struct XError {
    pub description: String,
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
}

impl fmt::Display for XError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "X error: {} (code: {}, request code: {}, minor code: {})",
            self.description, self.error_code, self.request_code, self.minor_code
        )
    }
}

/// Error returned if this system doesn't have XLib or can't create an X connection.
#[derive(Clone, Debug)]
pub enum XNotSupported {
    /// Failed to load one or several shared libraries.
    LibraryOpenError(OpenError),
    /// Connecting to the X server with `XOpenDisplay` failed.
    XOpenDisplayFailed,
}

impl From<OpenError> for OsError {
    #[inline]
    fn from(err: OpenError) -> OsError {
        OsError::XNotSupported(XNotSupported::LibraryOpenError(err))
    }
}

impl fmt::Display for XNotSupported {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        formatter.write_str(

        match *self {
            XNotSupported::LibraryOpenError(_) => "Failed to load one of xlib's shared libraries",
            XNotSupported::XOpenDisplayFailed => "Failed to open connection to X server",
        }
            )
    }
}
