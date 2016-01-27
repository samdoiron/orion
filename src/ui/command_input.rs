// An input mechanism for recieving UI commands / events, implementing the
// command pattern.
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

use std::convert;

use io::transport::{ReadTransport, TransportError};
use io::transport;

use ui::command::Command;
use ui::command_parser::{parse_command, ParseError};

use log;

pub trait CommandInput {
    fn receive_command(&mut self) -> transport::Result<Command>;
    fn receive_command_no_wait(&mut self) -> transport::Result<Option<Command>>;
}


/// Any ReadTransport can be used as a CommandInput. It is assumed
/// that the transport will send a properly-formed UTF-8 encoded string, which
/// conforms to the basic CommandInput protocol (see ui::command_parser).
impl<T> CommandInput for T 
    where T: ReadTransport {
    fn receive_command(&mut self) -> transport::Result<Command> {
        let received = try!(self.receive());
        Ok(try!(parse_command(&received)))
    }

    fn receive_command_no_wait(&mut self) -> transport::Result<Option<Command>> {
        let received = try!(self.receive_no_wait());
        if received.is_none() {
            return Ok(None);
        }
        let unwrapped = received.unwrap();
        log::debug(&format!("COMMAND: {}", &unwrapped));
        let parsed = try!(parse_command(&unwrapped));
        Ok(Some(parsed))
    }
}

impl convert::From<ParseError> for TransportError {
    fn from(_err: ParseError) -> TransportError {
        TransportError::new("Protocol error: failed to parse command".to_string(), None)
    }
}
