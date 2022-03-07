use std::fmt;
use std::default::Default;

/// Custom Error for Field type
#[derive(Debug, Clone)]
pub struct FieldError {
    pub message: String,
}

impl Default for FieldError {
    fn default() -> Self {
        FieldError {
            message: "FieldElement Error".to_string(),
        }
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FieldError {}

