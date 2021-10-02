
use std::string::String;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl ParseError {
    pub fn new(message: String) -> Self {
        return ParseError{message: message};
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl Error for ParseError {}
