// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use super::series::Series;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Histogram {
    title: String,
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
    series: Vec<Series>
}
