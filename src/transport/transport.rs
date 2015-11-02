use std::io;
use std::error;
use std::fmt;

/// Represents anything that can read Strings from somewhere, for example
/// a TCP socket.
pub trait Transport {
    fn receive(&mut self) -> TransportResult<String>;
    fn receive_no_wait(&mut self) -> TransportResult<Option<String>>;
}

#[derive(Debug)]
pub struct TransportError {
    message: String,
    io_error: Option<io::Error>
}

impl error::Error for TransportError {
    fn description(&self) -> &str {
        match self.io_error {
            Some(ref err) => err.description(),
            None => &self.message,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.io_error {
            Some(ref some) => Some(some),
            None => None
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl TransportError {
    pub fn new(message: String, cause: Option<io::Error>) -> TransportError {
        return TransportError {
            message: message,
            io_error: cause
        }
    }
}

pub type TransportResult<T> = Result<T, TransportError>;

