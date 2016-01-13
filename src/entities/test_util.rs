// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use test_util;
use super::series::{Series, DataPoint, MicroTime};

pub fn random_sorted_datapoints(num: usize) -> Vec<DataPoint> {
    assert!(num < 500_000_000, "Extremely large number of datapoints requested");
    let mut datapoints: Vec<DataPoint> = Vec::with_capacity(num);
    for i in 0..num {
        datapoints.push(DataPoint(i as MicroTime, test_util::random::in_range(1, 100) as f64));
    }
    return datapoints;
}

pub fn random_datapoints(num: usize) -> Vec<DataPoint> {
    assert!(num < 500_000_000, "Extremely large number of datapoints requested");
    let mut datapoints = random_sorted_datapoints(num);
    let shift = test_util::random::in_range(1, (num as i32) - 1) as usize;
    // LAME: Just shift all indexes by a random amount and wrap around.
    for i in 0..num {
        datapoints[i] = datapoints[(i + shift) % num].clone();
    }
    return datapoints;
}


pub fn random_series(num: usize) -> Series {
    assert!(num < 500_000_000, "Extremely large number of datapoints requested");
    let datapoints = random_datapoints(num);
    let some_name_length = test_util::random::in_range(1, 30) as usize;
    let series_name = test_util::random::ascii_string(some_name_length);
    let mut series = Series::new(series_name);

    for point in datapoints {
        &series.add_value(point);
    }

    return series;
}
