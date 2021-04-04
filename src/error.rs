use std::{error::Error, fmt, io};

#[derive(PartialEq)]
pub struct AppError {
    kind: String,
    message: String,
}

impl AppError {
    pub fn new(kind: String, message: String) -> AppError {
        AppError { kind, message }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.kind, self.message)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

impl From<subparse::errors::Error> for AppError {
    fn from(error: subparse::errors::Error) -> Self {
        AppError {
            kind: "parse".to_string(),
            message: error.to_string(),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: "io".to_string(),
            message: error.to_string(),
        }
    }
}

impl From<regex::Error> for AppError {
    fn from(error: regex::Error) -> Self {
        AppError {
            kind: "regex".to_string(),
            message: error.to_string(),
        }
    }
}
