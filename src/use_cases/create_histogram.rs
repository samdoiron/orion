// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

use use_cases::repos::Repo;
use entities::series::{Series, SeriesId};
use entities::charts::{Histogram, ChartId};
use entities::identified::Identified;

pub struct CreateHistogram<'a> {
    pub histogram_repo: &'a mut Repo<ChartId,Histogram>,
    pub series_repo: &'a mut Repo<SeriesId,Series>,

    pub histogram_name: &'a str,
    pub series_id: SeriesId,

    pub output: &'a mut OnHistogramCreated
}

/// Sent to `OnHistogramCreated` output port on successful creation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistogramCreated {
    pub histogram_id: ChartId,
    pub histogram_name: String,
    pub series_id: SeriesId,
    pub series_name: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    HistogramRepoUnavailable,
    SeriesRepoUnavailable,
    InvalidSeriesId
}

pub type CreateHistogramResult = Result<HistogramCreated, self::Error>;

pub trait OnHistogramCreated {
    fn on_histogram_created(&mut self, CreateHistogramResult);
}

impl<T> OnHistogramCreated for T
    where T: FnMut(CreateHistogramResult) {
    fn on_histogram_created(&mut self, result: CreateHistogramResult) {
        &self(result);
    }
}

pub fn create_histogram(request: &mut CreateHistogram) {
    // Ensure that the given series id is valid
    let series_result = request.series_repo.get(request.series_id.clone());
    if series_result.is_err() {
        request.output.on_histogram_created(Err(Error::SeriesRepoUnavailable));
        return
    }

    let maybe_series = series_result.unwrap();
    if maybe_series.is_none() {
        request.output.on_histogram_created(Err(Error::InvalidSeriesId));
        return
    }
    let series = maybe_series.unwrap();

    // Create the histogram
    let histogram = Histogram::new(request.histogram_name.to_string(), 
                                   request.series_id.clone());
    let add_histogram_result = request.histogram_repo.add(histogram.clone());
    if add_histogram_result.is_err() {
        request.output.on_histogram_created(Err(Error::HistogramRepoUnavailable));
        return;
    }

    request.output.on_histogram_created(Ok(HistogramCreated{
        histogram_id: histogram.id(),
        histogram_name: histogram.name,
        series_id: series.id(),
        series_name: series.name
    }));
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::sync::mpsc;

    use entities::series::Series;
    use entities::identified::Identified;
    use entities::test_util::random_series;

    use test_util::random;
    use use_cases::repos::{StubRepo};
    use util;
    
    use super::{create_histogram, CreateHistogram};

    fn random_populated_series() -> Series {
        let some_num_datapoints = random::in_range(0, 100) as usize;
        random_series(some_num_datapoints)
    }

    fn random_name() -> String {
        random::length_ascii_string(0, 100)
    }

    #[test]
    fn create_histogram__happy_path__returns_success() {
        let some_series = random_populated_series();
        let mut series_repo = StubRepo::containing(&some_series);

        let (tx, rx) = mpsc::channel();
        let mut callback = |result| {  tx.send(result).unwrap() };

        let mut request = CreateHistogram {
            histogram_repo: &mut StubRepo::empty(),
            histogram_name: &random_name(),
            series_id: some_series.id(),
            series_repo: &mut series_repo,
            output: &mut callback
        };
        create_histogram(&mut request);

        let timer = util::ms_timer(200);
        while timer.try_recv().is_err() {
            let result = rx.try_recv();
            match result {
                // No mpsc error nor creation error.
                Ok(Ok(created)) => {
                    assert_eq!(created.histogram_name, request.histogram_name);
                    assert_eq!(created.series_id, some_series.id());
                    assert_eq!(created.series_name, some_series.name);
                    return
                },

                // No mpsc error, creation error
                Ok(Err(err)) => panic!(err),

                // mpsc error, non-recoverable
                Err(mpsc::TryRecvError::Disconnected) => panic!("mpsc disconnected"),

                // No response yet...
                Err(mpsc::TryRecvError::Empty) => ()
            }
        }
    }
}

