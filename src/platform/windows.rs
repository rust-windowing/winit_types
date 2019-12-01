#![cfg(target_os = "windows")]

use std::sync::Arc;

pub type OsError = Arc<std::io::Error>;
