// Copyright (C) 2015  Samuel Doiron
pub fn create_histogram(output: &mut OnHistogramCreated, name: &str) {
    output.histogram_created(HistogramCreated {
    });
}

pub struct HistogramCreated {
    pub id: i32,
}

trait OnHistogramCreated {
    fn on_histogram_created(HistogramCreated) {
    }
}
