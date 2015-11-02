use std::sync::mpsc;
use net2::{TcpListenerExt, TcpStreamExt};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::result;
use std::io;
use std::io::{Read};
use std::num;
use std::convert;
use std::error::{Error};
use std::str;
use std::string;
use log;

use transport::{Transport, TransportError};

impl Transport for TcpServer {
    fn receive(&mut self) -> Result<String> {
        match self.package_rx.recv() {
            Ok(package) => Ok(package),
            Err(err) => Err(TransportError::new(
                    format!("failed to read from package receiver: {}", err).to_string(),
                    None))
        }
    }

    fn receive_no_wait(&mut self) -> Result<Option<String>> {
        match self.package_rx.try_recv() {
            Ok(package) => Ok(Some(package)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(err) => Err(TransportError::new(
                    format!("failed to read from package receiver: {}", err).to_string(),
                    None))
        }
    }
}

pub type Result<T> = result::Result<T, TransportError>;

fn spawn_accept_thread(port: u16, package_tx: mpsc::Sender<String>) -> Result<mpsc::Sender<()>> {
    let listener = try!(TcpListener::bind(("127.0.0.1", port)));
    let package_tx = package_tx.clone();
    let (quit_tx, quit_rx) = mpsc::channel::<()>();
    thread::spawn(move || {
        let mut acceptor = TcpAcceptor::new(package_tx, quit_rx);
        acceptor.listen(listener);
    });
    Ok(quit_tx)
}

pub struct TcpServer {
    accept_thread_quit_tx: mpsc::Sender<()>,
    package_rx: mpsc::Receiver<String>
}

impl TcpServer {
    pub fn new(port: u16) -> Result<TcpServer> {
        let (tx, rx) = mpsc::channel();
        Ok(TcpServer {
            accept_thread_quit_tx: try!(spawn_accept_thread(port, tx)),
            package_rx: rx
        })
    }

    pub fn stop(&mut self) {
        let is_err = self.accept_thread_quit_tx.send(()).is_err();
        if is_err {
            log::error("accept_thread already dead on attmept to stop");
        }
    }
}

struct TcpAcceptor {
    quit_txs: Vec<mpsc::Sender<()>>,
    quit_rx: mpsc::Receiver<()>,
    package_tx: mpsc::Sender<String>
}

impl TcpAcceptor {
    fn new(package_tx: mpsc::Sender<String>, quit_rx: mpsc::Receiver<()>) -> TcpAcceptor {
        return TcpAcceptor {
            quit_txs: vec![],
            package_tx: package_tx,
            quit_rx: quit_rx
        }
    }

    fn listen(&mut self, listen: TcpListener) {
        listen.set_nonblocking(true)
            .ok().expect("fail to set nonblocking on TcpSocket");

        let mut running = true; 
        while running {
            match listen.accept() {
                Ok((stream, _)) => self.handle_connection(stream),
                Err(err) => {
                    // WouldBlock is normal (just means there was no TCP client
                    // waiting to connect, and we won't wait for one because we're
                    // in non-blocking mode.
                    if err.kind() != io::ErrorKind::WouldBlock {
                        let message = format!("dropped incoming tcp connection: {}", err);
                        log::warn(&message)
                    }
                }
            }

            running = match self.quit_rx.try_recv() {
                Ok(_) => false,
                Err(mpsc::TryRecvError::Empty) => true,
                Err(mpsc::TryRecvError::Disconnected) => panic!("TcpServer disconnected without killing TcpAcceptor")
            };
        }
        self.stop_client_managers();
    }

    fn stop_client_managers(&mut self) {
        let mut all_ok = true;
        for quit_tx in &self.quit_txs {
            all_ok = all_ok || quit_tx.send(()).is_ok();
        }
        if !all_ok {
            log::error("one or more client managers already dead");
        }
    }

    // Pass own clients in as mutable so that we don't need a mutable self reference,
    // because above listen.incoming() requires an imutable borrow, and a mutable borrow
    // here would lead to having a mutable borrow for listener.
    fn handle_connection(&mut self, stream: TcpStream) {
        let package_tx = self.package_tx.clone();
        let (quit_tx, quit_rx) = mpsc::channel::<()>();
        self.quit_txs.push(quit_tx);
        thread::spawn(move || {
            let mut manager = TcpClientManager::new(stream, package_tx, quit_rx);
            manager.listen();
        });
    }
}

struct TcpClientManager {
    stream: TcpStream,
    tx: mpsc::Sender<String>,
    quit_rx: mpsc::Receiver<()>
}

// Large enough to contain any valid u32 ascii encoded.
const PACKAGE_LENGTH_LENGTH: u32 = 10;
const PACKAGE_LENGTH_WARN_THRESHOLD: u32 = 100_000_000;

impl TcpClientManager {
    fn new(stream: TcpStream, tx: mpsc::Sender<String>, quit_rx: mpsc::Receiver<()>) -> TcpClientManager {
        log::info("client connected to TCP server");
        return TcpClientManager{ stream: stream, tx: tx, quit_rx: quit_rx};
    }

    fn listen(&mut self) {
        let mut running = true;
        while running {
            let package_length = match self.read_package_length() {
                Ok(val) => val,
                // Probably means no more packages
                Err(_) => {
                    log::info("client disconnected from TCP server");
                    break
                }
            };

            if package_length > PACKAGE_LENGTH_WARN_THRESHOLD {
                log::warn(&format!("very large package size of {} bytes", package_length));
            }

            let package_str = match self.read_package(package_length) {
                Ok(package_str) => package_str,
                Err(err) => panic!("failed to read package with length = {}", err)
            };

            self.tx.send(package_str)
                .ok().expect("failed to send package");

            running = match self.quit_rx.try_recv() {
                Ok(_) => false,
                Err(mpsc::TryRecvError::Empty) => true,
                Err(mpsc::TryRecvError::Disconnected) => panic!("acceptor disconnected before client manager")
            };
        }
    }

    fn read_package(&mut self, package_length: u32) -> Result<String> {
        // Impossible for unsafety to leak. 
        // Basically, allocate memory for the package ONLY once, without
        // initializing it, read from the reader, and reinterperate as String.
        // If this is ever a problem, just remove it and use `vec!`. The time
        // savings aren't huge (~10% of thread run time)
        unsafe {
            let mut buff = Vec::with_capacity(package_length as usize);

            // Unsafe, because it reveals uninitialized memory, but
            // .read() on a TCP stream blocks until it is filled, so no
            // harm done.
            buff.set_len(package_length as usize);

            let took = try!(self.stream.read(&mut buff));
            assert_eq!(took, package_length as usize);

            return Ok(try!(String::from_utf8(buff)));
        }
    }

    fn read_package_length(&mut self) -> Result<u32> {
        // Read n bytes of ASCII-encoded number representing
        // total transfer size..
        let mut buff = [0; PACKAGE_LENGTH_LENGTH as usize];
        let bytes_read = self.stream.read(&mut buff);
        match bytes_read {
            // TODO Return result optional and not combine normal disconnect with read error?
            Ok(0) => return Err(TransportError::new("client disconnected".to_string(), None)),
            Ok(len) => assert_eq!(PACKAGE_LENGTH_LENGTH as usize, len),
            Err(err) => return Err(TransportError::new("failed to read package length".to_string(), Some(err)))
        }

        let as_str = try!(str::from_utf8(&buff));
        Ok(try!(as_str.parse::<u32>()))
    }
}

// === Error extensions

impl convert::From<io::Error> for TransportError {
    fn from(err: io::Error) -> TransportError {
        TransportError::new(err.description().to_string(), Some(err))
    }
}

impl convert::From<num::ParseIntError> for TransportError {
    fn from(_: num::ParseIntError) -> TransportError {
        TransportError::new("failed to parse int value".to_string(), None)
    }
}

impl convert::From<string::FromUtf8Error> for TransportError {
    fn from(_: string::FromUtf8Error) -> TransportError {
        TransportError::new("received invalid UTF-8 text transfer".to_string(), None)
    }
}

impl convert::From<str::Utf8Error> for TransportError {
    fn from(_: str::Utf8Error) -> TransportError {
        TransportError::new("received invalid UTF-8 text transfer".to_string(), None)
    }
}
