use std::collections::btree_map::BTreeMap;
use super::series::{Series};

pub struct Session {
    series: BTreeMap<String, Series>
}

impl Session {
    pub fn new() -> Session {
        return Session{series: BTreeMap::new()}
    }

    pub fn get_series_by_name(&mut self, name: &str) -> Option<&mut Series> {
        return self.series.get_mut(name);
    }

    pub fn does_series_exist(&self, name: &str) -> bool {
        return self.series.get(name).is_some();
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.insert(series.name.clone(), series);
    }
}

#[allow(non_snake_case, dead_code)]
mod tests {
    use super::Session;
    use test_util;
    use super::super::test_util::{random_series};

    fn random_session() -> Session {
        let some_num_series = test_util::random::in_range(10, 20) as usize;
        let some_num_datapoints = test_util::random::in_range(1, 100) as usize;
        let mut randoms = Vec::with_capacity(some_num_series);

        for _ in 0..some_num_series {
            randoms.push(random_series(some_num_datapoints));
        }

        let random_index = test_util::random::in_range(0, some_num_series as i32 - 1) as usize;
        let target_series_name = randoms[random_index].name.clone();

        let mut session = Session::new();
        for each in randoms {
            session.add_series(each);
        }

        session
    }

    fn get_series_by_name__with_matching_series__returns_series() {
        let some_num_series = test_util::random::in_range(10, 20) as usize;
        let some_num_datapoints = test_util::random::in_range(1, 100) as usize;
        let mut randoms = Vec::with_capacity(some_num_series);

        for _ in 0..some_num_series {
            randoms.push(random_series(some_num_datapoints));
        }

        let random_index = test_util::random::in_range(0, some_num_series as i32 - 1) as usize;
        let target_series_name = randoms[random_index].name.clone();

        let mut session = Session::new();
        for each in randoms {
            session.add_series(each);
        }

        let maybe_target = session.get_series_by_name(&target_series_name);
        let unwrapped = maybe_target.expect("no series found, but one existed");
        assert_eq!(unwrapped.name, target_series_name);
    }

    fn get_series_by_name__with_no_matching_series__returns_none() {
        let mut session = random_session();
        let got = session.get_series_by_name("some non-existant name");
        match got {
            Some(ser) => panic!("returned series named {:?}", ser),
            None => ()
        }
    }
}
