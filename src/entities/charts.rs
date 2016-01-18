// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use entities::series::{SeriesId};
use entities::identified::Identified;

pub type ChartId = String;

#[derive(Debug, Clone)]
pub struct ScatterPlot {
    pub name: String,
    pub x_series: SeriesId,
    pub y_series: SeriesId
}

#[derive(Debug, Clone)]
pub struct Histogram {
    pub name: String,
    pub x_series: SeriesId
}

impl Histogram { 
    pub fn new(name: String, x_series: SeriesId) -> Histogram {
        Histogram {
            name: name,
            x_series: x_series
        }
    }
}

impl Identified<ChartId> for ScatterPlot {
    fn id(&self) -> ChartId {
        self.name.clone()
    }
}

impl<'a> Identified<ChartId> for Histogram {
    fn id(&self) -> ChartId {
        self.name.clone()
    }
}
