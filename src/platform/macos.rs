#![cfg(target_os = "macos")]

use std::fmt;

#[derive(Clone, Debug)]
pub enum OsError {
    CGError(core_graphics::base::CGError),
    CreationError(&'static str),
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsError::CGError(e) => f.pad(&format!("CGError {}", e)),
            OsError::CreationError(e) => f.pad(e),
        }
    }
}
