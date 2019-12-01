#![cfg(target_arch = "wasm32")]

use std::fmt;

#[derive(Clone, Debug)]
pub struct OsError(pub String);

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
