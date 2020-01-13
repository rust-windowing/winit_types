#![cfg(target_os = "android")]

use std::sync::Arc;
pub type OsError = Arc<std::io::Error>;
