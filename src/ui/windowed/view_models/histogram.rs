// Copyright (C) 2015  Samuel Doiron
use super::series::Series;

pub struct Histogram {
    title: String,
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
    series: Vec<Series>
}
