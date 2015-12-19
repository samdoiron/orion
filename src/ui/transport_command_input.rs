// Copyright (C) 2015  Samuel Doiron
use std::convert;

use ui::command_input::CommandInput;
use ui::command::Command;
use ui::command_parser::{parse_command, ParseError};
use transport::transport;
use transport::transport::{ReadTransport, TransportError};

/// Any ReadTransport can be used as a CommandInput. It is assumed
/// that the transport will send a properly-formed UTF-8 encoded string, which
/// conforms to the basic CommandInput protocol (see ui::command_parser).
struct TransportCommandInput<'a> {
    transport: &'a mut ReadTransport
}

impl<'a> CommandInput for TransportCommandInput<'a> {
    fn receive_command(&mut self) -> transport::Result<Command> {
        let received = try!(self.transport.receive());
        Ok(try!(parse_command(&received)))
    }

    fn receive_command_no_wait(&mut self) -> transport::Result<Option<Command>> {
        let received = try!(self.transport.receive_no_wait());
        if received.is_none() {
            return Ok(None);
        }
        Ok(Some(try!(parse_command(&received.unwrap()))))
    }
}

impl convert::From<ParseError> for TransportError {
    fn from(err: ParseError) -> TransportError {
        TransportError::new("Protocol error: failed to parse command".to_string(), None)
    }
}
