#![cfg(target_os = "ios")]

use std::fmt;

#[derive(Debug)]
pub enum OsError {}

impl fmt::Display for OsError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => unreachable!(),
        }
    }
}
