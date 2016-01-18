// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use std::collections::btree_map::BTreeMap;
use super::series::{Series};

#[derive(Debug)]
struct SessionId(i32);

pub struct Session {
    series: BTreeMap<String, Series>
}

impl Session {
    pub fn new() -> Session {
        Session{series: BTreeMap::new()}
    }

    pub fn get_series_by_name(&mut self, name: &str) -> Option<&mut Series> {
        self.series.get_mut(name)
    }

    pub fn does_series_exist(&self, name: &str) -> bool {
        self.series.get(name).is_some()
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.insert(series.name.clone(), series);
    }


}

#[allow(non_snake_case, dead_code)]
mod tests {
    use super::Session;
    use entities::series::Series;
    use test_util;
    use super::super::test_util::{random_series};

    fn random_session() -> Session {
        let some_num_series = test_util::random::in_range(10, 20) as usize;
        let some_num_datapoints = test_util::random::in_range(10, 100) as usize;
        let mut randoms = Vec::with_capacity(some_num_series);

        for _ in 0..some_num_series {
            randoms.push(random_series(some_num_datapoints));
        }

        let mut session = Session::new();
        for each in randoms {
            session.add_series(each);
        }

        session
    }

    fn random_series_from_session(session: &Session) -> &Series {
        let index = test_util::random::in_range(0, (session.series.len() - 1) as i32) as usize;
        let (_, series) = session.series.iter().nth(index).unwrap();
        series
    }

    #[test]
    fn get_series_by_name__with_matching_series__returns_series() {
        let mut session = random_session();
        let target_series_name = random_series_from_session(&session).name.clone();

        let maybe_target = session.get_series_by_name(&target_series_name);
        let unwrapped = maybe_target.expect("no series found, but one existed");
        assert_eq!(unwrapped.name, target_series_name);
    }

    #[test]
    fn get_series_by_name__with_no_matching_series__returns_none() {
        let mut session = random_session();
        let got = session.get_series_by_name("some non-existant name");
        match got {
            Some(ser) => panic!("returned series named {:?}", ser),
            None => ()
        }
    }

    #[test]
    fn does_series_exist__when_it_does__says_it_does() {
        let session = random_session();
        let series = random_series_from_session(&session);
        assert!(session.does_series_exist(&series.name))
    }

    #[test]
    fn does_series_exist__when_it_does_not__says_it_does_not() {
    }
}
