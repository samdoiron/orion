// Copyright (C) 2015  Samuel Doiron
use std::io;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, TransportError>;

pub trait ReadTransport {
    fn receive(&mut self) -> Result<String>;
    fn receive_no_wait(&mut self) -> Result<Option<String>>;
}

pub trait WriteTransport {
    fn send(&mut self, &str) -> Result<()>;
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
