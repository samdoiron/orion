// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use super::{Series, Histogram};
use io::serialize::Serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Main {
    pub series: Vec<Series>,
    pub charts: Vec<Histogram>
}


impl Serialize for Main {
    fn serialize(&self) -> String {
        json::encode(&self).unwrap()
    }
}
