use std::{fmt, io};

pub struct Error {
    kind: String,
    message: String,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.kind, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<subparse::errors::Error> for Error {
    fn from(error: subparse::errors::Error) -> Self {
        Error {
            kind: "parse".to_string(),
            message: error.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: "io".to_string(),
            message: error.to_string(),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error {
            kind: "regex".to_string(),
            message: error.to_string(),
        }
    }
}
