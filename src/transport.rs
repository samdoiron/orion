use std::sync::mpsc;
use net2::TcpListenerExt;
use std::net::{TcpStream, TcpListener};
use std::thread;
use entities::series::{DataPoint};
use std::result;
use std::fmt;
use std::io;
use std::io::{Read};
use std::num;
use std::convert;
use std::error::{Error};
use std::str;
use log;

#[derive(Debug)]
pub struct NamedDataPoint {
    name: String,
    datapoint: DataPoint
}

pub trait Transport {
    fn receive(&mut self) -> NamedDataPoint;
}


#[derive(Debug)]
enum TransportErrorKind {
    IoError(io::Error),
    ProtocolError,
}
use self::TransportErrorKind::*;

#[derive(Debug)]
struct TransportError {
    message: String,
    kind: TransportErrorKind
}

impl TransportError {
    fn io_error(err: io::Error, message: Option<String>) -> TransportError {
        TransportError {
            kind: IoError(err),
            message: format!("ProtocolError: {}", &message.unwrap_or_else(|| String::new()))
        }
    }

    fn protocol_error(message: Option<String>) -> TransportError {
        TransportError {
            kind: ProtocolError,
            message: format!("ProtocolError: {}", &message.unwrap_or_else(|| String::new()))
        }
    }
}

impl convert::From<io::Error> for TransportError {
    fn from(err: io::Error) -> TransportError {
        TransportError::io_error(err, None)
    }
}

impl convert::From<num::ParseFloatError> for TransportError {
    fn from(_: num::ParseFloatError) -> TransportError {
        TransportError::protocol_error(Some("failed to parse floating point value".to_string()))
    }
}

impl convert::From<num::ParseIntError> for TransportError {
    fn from(_: num::ParseIntError) -> TransportError {
        TransportError::protocol_error(Some("failed to parse int value".to_string()))
    }
}

impl convert::From<str::Utf8Error> for TransportError {
    fn from(_: str::Utf8Error) -> TransportError {
        TransportError::protocol_error(Some("received invalid UTF-8 text transfer".to_string()))
    }
}



impl Error for TransportError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            IoError(ref err) => Some(err as &Error),
            _ => None
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

pub type Result<T> = result::Result<T, TransportError>;

fn spawn_accept_thread(port: u16, datapoint_tx: mpsc::Sender<NamedDataPoint>) -> Result<mpsc::Sender<()>> {
    let listener = try!(TcpListener::bind(("127.0.0.1", port)));
    let datapoint_tx = datapoint_tx.clone();
    let (quit_tx, quit_rx) = mpsc::channel::<()>();
    thread::spawn(move || {
        let mut acceptor = TcpAcceptor::new(datapoint_tx, quit_rx);
        acceptor.listen(listener);
    });
    Ok(quit_tx)
}

pub struct TcpServer {
    accept_thread_quit_tx: mpsc::Sender<()>,
    datapoint_rx: mpsc::Receiver<NamedDataPoint>
}

impl TcpServer {
    pub fn new(port: u16) -> Result<TcpServer> {
        let (tx, rx) = mpsc::channel();
        Ok(TcpServer {
            accept_thread_quit_tx: try!(spawn_accept_thread(port, tx)),
            datapoint_rx: rx
        })
    }

    pub fn stop(&mut self) {
        let is_err = self.accept_thread_quit_tx.send(()).is_err();
        if is_err {
            log::error("accept_thread already dead on attmept to stop");
        }
    }
}

impl Transport for TcpServer {
    fn receive(&mut self) -> NamedDataPoint {
        return self.datapoint_rx.recv()
            .ok().expect("failed to read on datapoint channel");
    }
}

struct TcpAcceptor {
    quit_txs: Vec<mpsc::Sender<()>>,
    quit_rx: mpsc::Receiver<()>,
    datapoint_tx: mpsc::Sender<NamedDataPoint>
}

impl TcpAcceptor {
    fn new(datapoint_tx: mpsc::Sender<NamedDataPoint>, quit_rx: mpsc::Receiver<()>) -> TcpAcceptor {
        return TcpAcceptor {
            quit_txs: vec![],
            datapoint_tx: datapoint_tx,
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
                    // TODO Only unexpected if err != EWOULDBLOCK or EAGAIN
                    // because we use nonblocking accept.
                    let message = format!("dropped incoming tcp connection: {}", err);
                    log::warn(&message)
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
        let datapoint_tx = self.datapoint_tx.clone();
        let (quit_tx, quit_rx) = mpsc::channel::<()>();
        self.quit_txs.push(quit_tx);
        thread::spawn(move || {
            let mut manager = TcpClientManager::new(stream, datapoint_tx, quit_rx);
            manager.listen();
        });
    }
}

struct TcpClientManager {
    stream: TcpStream,
    tx: mpsc::Sender<NamedDataPoint>,
    quit_rx: mpsc::Receiver<()>
}

// Large enough to contain any valid u32 ascii encoded.
const PACKAGE_LENGTH_LENGTH: u32 = 10;
const PACKAGE_LENGTH_WARN_THRESHOLD: u32 = 100_000_000;

impl TcpClientManager {
    fn new(stream: TcpStream, tx: mpsc::Sender<NamedDataPoint>, quit_rx: mpsc::Receiver<()>) -> TcpClientManager {
        return TcpClientManager{ stream: stream, tx: tx, quit_rx: quit_rx};
    }

    fn listen(&mut self) {
        let mut running = true;
        while running {
            let package_length = self.read_package_length()
                .ok().expect("failed to read package length");

            if package_length > PACKAGE_LENGTH_WARN_THRESHOLD {
                log::warn(&format!("very large package size of {} bytes", package_length));
            }

            let package_str = self.read_package(package_length)
                .ok().expect("failed to read package");

            let package = self.decode(&package_str)
                .ok().expect("failed to decode package");

            self.tx.send(package)
                .ok().expect("failed to enqueue datapoint");

            running = match self.quit_rx.try_recv() {
                Ok(_) => false,
                Err(mpsc::TryRecvError::Empty) => true,
                Err(mpsc::TryRecvError::Disconnected) => panic!("acceptor disconnected before client manager")
            };
        }
    }

    fn read_package(&mut self, package_length: u32) -> Result<String> {
        let mut s = String::with_capacity(package_length as usize);
        let took = self.stream.read_to_string(&mut s);

        match took {
            Ok(_) => Ok(s),
            Err(err) => Err(TransportError::io_error(err, None))
        }
    }

    fn read_package_length(&mut self) -> Result<u32> {
        // Read n bytes of ASCII-encoded number representing
        // total transfer size..
        let mut buff = [0; PACKAGE_LENGTH_LENGTH as usize];
        let bytes_read = self.stream.read(&mut buff);
        match bytes_read {
            // TcpStream read is blocking, so this should always be true, unless
            // it's an error.
            Ok(len) => assert!(len == PACKAGE_LENGTH_LENGTH as usize),
            Err(err) => return Err(TransportError::io_error(err, None))
        }

        let as_str = try!(str::from_utf8(&buff));
        Ok(try!(as_str.parse::<u32>()))
    }

    // NamedDataPoint packages (currently only ones supported) have the format:
    // `<name> <timestamp> <float value>`
    fn decode(&self, package: &str) -> Result<NamedDataPoint> {
        let mut split = package.split(" ");

        let name = split.next();
        let timestamp = split.next();
        let series_value = split.next();

        let too_few_error_message = Some("too few package values provided".to_string());
        name.and(timestamp)
            .and(series_value)
            .map_or(Err(TransportError::protocol_error(too_few_error_message)), |_| {
                let name = name.unwrap();
                let timestamp = try!(timestamp.unwrap().parse::<u64>());
                let series_value = try!(series_value.unwrap().parse::<f64>());

                Ok(NamedDataPoint {
                    name: name.to_string(),
                    datapoint: DataPoint(timestamp, series_value)
                })
            })
    }
}


