#![cfg(target_os = "android")]

#[derive(Clone, Debug)]
pub enum OsError {
    Misc(String),
}

impl std::fmt::Display for OsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Misc(e) => f.pad(e),
        }
    }
}
