use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct AstroError {
    message: String,
}

impl AstroError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for AstroError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "AstrometryError: {}", self.message)
    }
}

impl Error for AstroError {}
