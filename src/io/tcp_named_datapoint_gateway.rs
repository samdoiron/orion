// Copyright (C) 2015  Samuel Doiron
use entities::series::DataPoint;
use gateways::NamedDataPointGateway;
use use_cases::NamedDataPoint;
use transport::{ReadTransport, TcpServer};

use std::error;
use std::fmt;
use std::convert;
use std::num::{ParseIntError, ParseFloatError};

pub struct TcpNamedDataPointGateway {
    tcp_server: TcpServer
}

const PORT: u16 = 3141;

impl TcpNamedDataPointGateway {
    pub fn new() -> TcpNamedDataPointGateway {
        return TcpNamedDataPointGateway {
            // FIXME Don't unwrap, handle port collision
            tcp_server: TcpServer::new(PORT).unwrap(),
        }
    }

    fn decode(&self, received: String) -> Result<NamedDataPoint, DecodeError> {
        let mut split = received.split(" ");

        let series_name = split.next();
        let timestamp = split.next();
        let series_value = split.next();

        if series_name.and(timestamp).and(series_value).is_some() {
            let series_name = series_name.unwrap();
            let timestamp = try!(timestamp.unwrap().parse::<u64>());
            let series_value = try!(series_value.unwrap().parse::<f64>());

            Ok(NamedDataPoint {
                series_name: series_name.to_string(),
                datapoint: DataPoint(timestamp, series_value)
            })
        } else {
            Err(DecodeError::new("too few package values provided".to_string()))
        }
    }
}

impl NamedDataPointGateway for TcpNamedDataPointGateway {
    fn receive_datapoint(&mut self) -> NamedDataPoint {
        loop {
            let datapoint = self.tcp_server.receive();
            if datapoint.is_err() {
                continue
            }

            let decoded = self.decode(datapoint.unwrap());
            if decoded.is_ok() {
                return decoded.unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct DecodeError {
    message: String
}

impl DecodeError {
    fn new(message: String) -> DecodeError {
        DecodeError { message: message }
    }
}

impl error::Error for DecodeError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl convert::From<ParseFloatError> for DecodeError {
    fn from(_: ParseFloatError) -> DecodeError {
        DecodeError::new("failed to parse DataPoint float".to_string())
    }
}

impl convert::From<ParseIntError> for DecodeError {
    fn from(_: ParseIntError) -> DecodeError {
        DecodeError::new("failed to parse DataPoint int".to_string())
    }
}
