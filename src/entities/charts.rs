// Copyright (C) 2015  Samuel Doiron
use super::series::Series;

#[derive(Debug)]
pub struct ScatterPlot {
    pub x_series: Series,
    pub y_series: Vec<Series>,
}

#[derive(Debug)]
pub struct Histogram {
    pub x_series: Series,
}
