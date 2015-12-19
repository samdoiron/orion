// High-level websocket server
// Copyright (C) 2015  Samuel Doiron
use websocket::{Server, Client, Message, Sender, Receiver};
use websocket::server;
use websocket::result::WebSocketError;
use websocket::stream::WebSocketStream;
use websocket::message::Type;

use transport;
use transport::{ReadTransport, WriteTransport, TransportError};
use log;

use std::io;


// A Warning to future code editors (AKA me)
// The person who wrote the WebSocket library for rust drank *way* too much
// generic kool-aid, and basically all of the types are nested three deep with
// traits that accept 2+ type parameters. Also, the name of the trait is usually
// the same as the name of the concrete implementation you get back.
//
// This means that all your time editing this will be trying to help the compiler
// deduce the types of variables.
//
// Have fun!

pub struct WebSocketServer<'a> {
    server: Server<'a>
}

pub type Port = u16;

pub struct WebSocketReceiver {
    receiver: server::Receiver<WebSocketStream>,
}

pub struct WebSocketSender {
    sender: server::Sender<WebSocketStream>,
}

impl<'a> WebSocketServer<'a> {
    pub fn new(port: Port) -> io::Result<WebSocketServer<'a>>  {
        let mut server = try!(Server::bind(("127.0.0.1", port)));
        Ok(WebSocketServer { server: server })
    }

    pub fn accept(&mut self) -> (WebSocketSender, WebSocketReceiver) {
        // Not sure what would cause this to fail.
        let request = self.server.accept()
            .ok().expect("failed to accept websocket connection")
            .read_request()
            .ok().expect("failed to read request");

        // Handshake to accept the websocket
        let response = request.accept();
        let mut conn = response.send().unwrap();
        let (mut snd, mut rcv) = conn.split();

        (WebSocketSender{sender: snd}, WebSocketReceiver{receiver: rcv})
    }
}


fn decode_or_handle_message(message: Message)
    -> Option<String> {
        log::warn("Ping handling disabled, connections might disconnect.");
    match message.opcode {
        Type::Text => {
            let decoded = String::from_utf8(message.payload.into_owned());
            decoded.ok()
        } 

        // XXX Got rid of PING handling because that would break the 
        // seperation of sender / receiver (reading might require) sending.
        // Those must be seperate, because ex. the presenter requires
        // a write end and the command input requires a read end, and they can't
        // share a single reference.
        //
        // Possible fixes: Arc<>, where internally read has a reference to
        // write, and aquires it when nessesary.

        _ => None
    }
}

impl ReadTransport for WebSocketReceiver {
    fn receive(&mut self) -> transport::Result<String> {
        for maybe_message in self.receiver.incoming_messages() {
            return match maybe_message {
                Ok(m) => {
                    let message: Message = m;
                    let decoded = decode_or_handle_message(message);
                    if decoded.is_some() {
                        Ok(decoded.unwrap())
                    } else {
                        let message = "Failed to decode message as utf-8".to_string();
                        Err(TransportError::new(message, None))
                    }
                },
                Err(err) => {
                    let message = "Failed to receive message".to_string();
                    Err(TransportError::new(message, None))
                }
            }
        }
        return Err(TransportError::new("Websocket closed?".to_string(), None))
    }

    // What is the difference between the expected running time
    // of an randomized algorithm and the expected running time of
    // a deterministic algorithm.

    fn receive_no_wait(&mut self) -> transport::Result<Option<String>> {
        match self.receiver.recv_message() {
            Ok(m) => Ok(decode_or_handle_message(m)),
            Err(WebSocketError::NoDataAvailable) => Ok(None),
            Err(err) => {
                let message = "Could not receive on websocket".to_string();
                Err(TransportError::new(message, None))
            }
        }
    }
}

impl WriteTransport for WebSocketSender {
    fn send(&mut self, message: &str) -> transport::Result<()> {
        let m: Message = Message::text(message);
        let result = self.sender.send_message(&m);
        if result.is_ok() {
            Ok(())
        } else {
            let message = "Could not send on websocket".to_string();
            Err(TransportError::new(message, None))
        }
    }
}
