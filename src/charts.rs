use series::Series;

pub struct ScatterPlot<T: Ord + Clone> {
    pub x_series: Series<T>,
    pub y_series: Vec<Series<T>>,
}

pub struct Histogram<T: Ord + Clone> {
    pub x_series: Series<T>,
}
