use entities::series::{Series, DataPoint};
use entities::session::{Session};

pub fn add_value_to_series(session: &mut Session, series_name: &str, datapoint: DataPoint) {
    // Nessesary because if we didn't do this, then we would need to
    // borrow another mutable reference to add a new series in the case where
    // one didn't exist. Rust only allows one mutable borrow at a time.
    //
    // Basically, we need a mutable borrow to determine if we need another mutable
    // borrow, which will be in a lower block, which we cant do.
    //
    // Unfortunately, it means we require two lookups :(
    // NOTE: Plan seems to be for rust to change this (rust-lang issue #811).
    if session.does_series_exist(series_name) {
        // Unwrap is safe unless does_series_exist lied to us (in which case
        // we should crash anyway)
        let series = session.get_series_by_name(series_name).unwrap();
        series.add_value(datapoint);
    } else {
        let mut series = Series::new(series_name.to_string());
        series.add_value(datapoint);
        session.add_series(series);
    }
}
