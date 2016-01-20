// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

#[derive(RustcDecodable, RustcEncodable)]
pub struct DataPoint {
    pub time: u64,
    pub value: f64
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Series {
    pub name: String,
    pub current_value: DataPoint
}

