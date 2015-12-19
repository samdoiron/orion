// Copyright (C) 2015  Samuel Doiron
use use_cases::NamedDataPoint;

pub trait NamedDataPointGateway {
    fn receive_datapoint(&mut self) -> NamedDataPoint;
}
