use std::error::Error;
use std::fmt;

/// Исправимые ошибки.
#[derive(Debug, Clone)]
pub struct FatalError {
    description: String,
}

impl FatalError {
    pub(crate) fn new(message: String) -> FatalError {
        FatalError {
            description: message,
        }
    }
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for FatalError {}