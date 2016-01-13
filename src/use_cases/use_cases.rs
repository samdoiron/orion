// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use entities::series::{Series, DataPoint};
use entities::charts::{Histogram};
use entities::session::{Session};
use use_cases::repos::Repo;

use std::fmt;

pub struct UseCases {
    histogram_repo: Box<Repo<Histogram>>
}

#[derive(Debug)]
pub struct NamedDataPoint {
    pub series_name: String,
    pub datapoint: DataPoint
}

impl fmt::Display for NamedDataPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NamedDataPoint({}, {}, {})", self.series_name, self.datapoint.0, self.datapoint.1)
    }
}

pub fn add_value_to_series(session: &mut Session, datapoint: NamedDataPoint) {
    let series_name = datapoint.series_name;
    let datapoint = datapoint.datapoint;
    // Nessesary because if we didn't do this, then we would need to
    // borrow another mutable reference to add a new series in the case where
    // one didn't exist. Rust only allows one mutable borrow at a time.
    //
    // Basically, we need a mutable borrow to determine if we need another mutable
    // borrow, which will be in a lower block, which we cant do.
    //
    // Unfortunately, it means we require two lookups :(
    // NOTE: Plan seems to be for rust to change this (rust-lang issue #811).
    if session.does_series_exist(&series_name) {
        // Unwrap is safe unless does_series_exist lied to us (in which case
        // we should crash anyway)
        let mut series = session.get_series_by_name(&series_name).unwrap();
        series.add_value(datapoint);
    } else {
        let mut series = Series::new(series_name.to_string());
        series.add_value(datapoint);
        session.add_series(series);
    }
}
