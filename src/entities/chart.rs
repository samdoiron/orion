// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use entities::series;
use entities::identified::Identified;

pub type Id = String;

#[derive(Debug, Clone)]
pub struct ScatterPlot {
    pub name: String,
    pub x_series: series::Id,
    pub y_series: series::Id
}

#[derive(Debug, Clone)]
pub struct Histogram {
    pub name: String,
    pub x_series: series::Id
}

impl Histogram { 
    pub fn new(name: String, x_series: series::Id) -> Histogram {
        Histogram {
            name: name,
            x_series: x_series
        }
    }
}

impl Identified<Id> for ScatterPlot {
    fn id(&self) -> Id {
        self.name.clone()
    }
}

impl<'a> Identified<Id> for Histogram {
    fn id(&self) -> Id {
        self.name.clone()
    }
}
