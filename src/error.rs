use std::fmt;
use std::default::Default;

/// Custom Error for FieldElement type
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
        write!(f, "FieldElement Error: {}", self.message)
    }
}

impl std::error::Error for FieldError {}

/// Custom Error for Point type
#[derive(Debug, Clone)]
pub struct PointError {
    pub message: String,
}

impl Default for PointError {
    fn default() -> Self {
        PointError {
            message: "Point Error".to_string(),
        }
    }
}

impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Point Error: {}", self.message)
    }
}

impl std::error::Error for PointError {}

