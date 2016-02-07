// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use std::cmp;
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

use entities::identified::Identified;

pub type MicroTime = u64; 

// NOTE: Not generic to T: Ord + Clone instead of f64 because in Session we need
// to record a Vec of every series in a Series. This means that if DataPoint
// was generic, then Series would need to be generic (to know how to accept
// the correct input datapoint to add_value), making it impossible to store
// generic types of Series in a collection in Session.
//
// Might be possible in the future / worth refactoring, but this is fine for
// now; especially because JSON doesn't tell us the difference either.
#[derive(Debug, Clone)]
pub struct DataPoint(pub MicroTime, pub f64);

// Datapoints are ordered by time recorded, not value.
impl PartialEq for DataPoint {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.0 != other.0;
    }
}

impl Eq for DataPoint {
}

impl PartialOrd for DataPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.0.partial_cmp(&other.0);
    }
}

impl Ord for DataPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_time = self.0;
        let their_time = other.0;
        return my_time.cmp(&their_time);
    }
}

pub type Id = String;

impl Identified<Id> for Series {
    fn id(&self) -> Id {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Series {
    pub name: String,
    points: BinaryHeap<DataPoint>,
}

impl Series {
    pub fn new(name: String) -> Series {
        Series{name: name, points: BinaryHeap::new()}
    }

    pub fn len(&self) -> usize {
        return self.points.len();
    }

    pub fn add_value(&mut self, point: DataPoint) {
        self.points.push(point);
    }

    // Return the most recent N items, sorted by datapoint's timestamp.
    // TODO: Optimize selecting the top-N items. Probably possible in 
    // O(log(n)) time using a modified heap which keeps track of
    // its number of children or something.
    // Currently this fn has O(nlog(n)) time, where n = min(size, requested),
    // and unnessesarily requires a mutable reference.
    // NOTE Rank-select tree?
    pub fn most_recent(&mut self, requested_amount: usize) -> Vec<DataPoint> {
        let num_to_take = cmp::min(requested_amount, self.points.len());
        let mut points = Vec::with_capacity(num_to_take);

        // Take max off heap for every datapoint we need
        let mut taken = 0;
        while taken < num_to_take {
            let point = self.points.pop();
            match point {
                Some(t) => {
                    points.push(t);
                    taken += 1;
                }
                None => panic!("Got None from heap before exhausting expected size")
            }
        }

        // Reinsert back into heap
        for point in &points {
            self.points.push(point.clone());
        }

        return points;
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{Series, DataPoint};
    use test_util;
    use super::super::test_util::{random_series};


    #[test]
    fn create_series__unicode_stress_test__doesnt_panic() {
        let strs = test_util::fuzz::strange_unicode_strings();
        for name in strs {
            Series::new(name);
        }
    }

    #[test]
    fn add_value__increases_length() {
        let mut series = Series::new("fps".to_string());
        let num_items = test_util::random::in_range(1, 10);

        for _ in 0..num_items {
            &series.add_value(DataPoint(123123, 60f64));
        }

        assert_eq!(num_items, series.points.len() as i32);
    }

    #[test]
    fn len__with_content__returns_num_points() {
        let num_datapoints = test_util::random::in_range(10, 100) as usize;
        let series = random_series(num_datapoints);
        assert_eq!(num_datapoints, series.len());
    }

    #[test]
    fn most_recent__enough_elements__returns_requested_amount() {
        let num_datapoints = test_util::random::in_range(10, 100) as usize;
        let mut series = random_series(num_datapoints);

        let tail_size = test_util::random::in_range(1, (num_datapoints / 2) as i32) as usize;
        let tail = series.most_recent(tail_size);

        assert_eq!(tail_size, tail.len());
    }

    #[test]
    fn most_recent__too_few_elements__returns_all() {
        let num_datapoints = 100;
        let mut series = random_series(num_datapoints);

        let tail_size = num_datapoints + test_util::random::in_range(1, num_datapoints as i32 / 10) as usize;
        let tail = series.most_recent(tail_size);

        assert_eq!(num_datapoints, tail.len());
    }

    #[test]
    fn most_recent__items_added_unsorted__returns_sorted_by_time() {
        let datapoints = vec![
            DataPoint(3, 10f64),
            DataPoint(2, 10f64),
            DataPoint(4, 10f64),
            DataPoint(1, 10f64),
        ];
        let mut series = Series::new(test_util::random::ascii_string(10));
        for point in &datapoints {
            &series.add_value(point.clone());
        }
        let most_recent = series.most_recent(3);
        let correct_order = vec![
            datapoints[2].clone(),
            datapoints[0].clone(),
            datapoints[1].clone()
        ];
        assert_eq!(correct_order, most_recent);
    }
}
