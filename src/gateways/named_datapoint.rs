use use_cases::NamedDataPoint;

pub trait NamedDataPointGateway {
    fn receive(&mut self) -> NamedDataPoint;
}
