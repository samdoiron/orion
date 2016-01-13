// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use std::io;
use std::error;
use std::fmt;
use std::result;

use std::error::Error;

pub type Result<T> = result::Result<T, TransportError>;

pub trait ReadTransport {
    fn receive(&mut self) -> Result<String>;
    fn receive_no_wait(&mut self) -> Result<Option<String>>;
}

pub trait WriteTransport {
    fn send(&mut self, &str) -> Result<()>;
}

#[derive(Debug)]
pub enum TransportError {
    Closed,
    IoError(String, io::Error),
    Error(String)
}

impl error::Error for TransportError {
    fn description(&self) -> &str {
        match *self {
            TransportError::Closed => "Transport was closed",
            TransportError::IoError(ref msg, _) => msg,
            TransportError::Error(ref msg) => msg
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            TransportError::Closed => None,
            TransportError::IoError(_, ref err) => Some(err),
            TransportError::Error(_) => None
        }
    }
}

impl fmt::Display for TransportError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl TransportError {
    pub fn new(message: String, cause: Option<io::Error>) -> TransportError {
        if cause.is_some() {
            return TransportError::IoError(message, cause.unwrap());
        } else {
            return TransportError::Error(message);
        }
    }
}
