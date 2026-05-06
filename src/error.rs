use std::io;
use std::fmt::{ Display, Formatter, Result };
use std::error;
#[derive(Debug)]

pub enum ServerError {
    Parse(String),
    Io(String),
    NotFound,
    MethodNotAllowed,
    BadRequest(String),
    Internal(String),
}

// wraps io errors into ServerError
impl From<io::Error> for ServerError {
    fn from(e: io::Error) -> Self{
        ServerError::Io(e.to_string())
    }
}
impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ServerError::Parse(e) => write!(f, "parsing error: {}", e),
            ServerError::Io(e) => write!(f, "io error: {}", e),
            ServerError::NotFound => write!(f, "404 Error Not Found"),
            ServerError::MethodNotAllowed => write!(f, "405 Error Method Not Found"),
            ServerError::BadRequest(e) => write!(f, "bad request error: {}", e),
            ServerError::Internal(e) => write!(f, "internal error: {}", e),
        }
    }
}
impl error::Error for ServerError {}