// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

#[derive(RustcDecodable, RustcEncodable)]
pub struct DataPoint {
    time: u64,
    value: f64
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Series {
    name: String,
    current_value: DataPoint
}
