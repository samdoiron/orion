// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use use_cases::NamedDataPoint;

pub trait NamedDataPointGateway {
    fn receive_datapoint(&mut self) -> NamedDataPoint;
}
