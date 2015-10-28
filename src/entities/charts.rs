use super::series::Series;

pub struct ScatterPlot {
    pub x_series: Series,
    pub y_series: Vec<Series>,
}

pub struct Histogram {
    pub x_series: Series,
}
