
use std::string::String;
use std::error::Error;

#[derive(Debug)]
pub struct GenericError {
    message: String
}

impl GenericError {
    pub fn new(message: String) -> Self {
        return GenericError{message: message};
    }
}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl Error for GenericError {}
