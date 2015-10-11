use std::cmp;
use std::collections::BTreeSet;
use std::collections::Bound::{Included};

pub type MicroTime = u64; 

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DataPoint<T: Ord + Clone>(pub MicroTime, pub T);

pub struct Series<T: Ord + Clone> {
    pub name: String,
    points: Vec<DataPoint<T>>,
}

impl<T: Ord + Clone> Series<T> {
    pub fn new(name: String) -> Series<T> {
        return Series{name: name, points: vec![]}
    }

    pub fn len(&self) -> usize {
        return self.points.len();
    }

    pub fn random_point(&self) -> Option<DataPoint<T>> {
        return self.points.last().map(|x| (*x).clone());
    }

    pub fn add_value(&mut self, point: DataPoint<T>) {
        self.points.push(point);
    }

    // Return the most recent N items, sorted by datapoint's timestamp.
    pub fn most_recent(&self, requested_amount: usize) -> Vec<DataPoint<T>> {
        let num_to_take = cmp::min(requested_amount, self.points.len());
        let mut recent_points = Vec::with_capacity(num_to_take);

        for i in 0..num_to_take {
            &recent_points.push(self.points[self.points.len() - 1 - i].clone());
        }

        return recent_points;
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    extern crate rand;
    use super::{Series, DataPoint, MicroTime};
    use tests::{rand_in_range, random_ascii_string, strange_unicode_strings};

    fn random_sorted_datapoints(num: usize) -> Vec<DataPoint<i32>> {
        assert!(num < 500_000_000, "Extremely large number of datapoints requested");
        let mut datapoints: Vec<DataPoint<i32>> = Vec::with_capacity(num);
        for i in 0..num {
            datapoints.push(DataPoint(i as MicroTime, rand_in_range(1, 100)));
        }
        return datapoints;
    }

    fn random_datapoints(num: usize) -> Vec<DataPoint<i32>> {
        assert!(num < 500_000_000, "Extremely large number of datapoints requested");
        let mut datapoints = random_sorted_datapoints(num);
        let shift = rand_in_range(1, (num as i32) - 1) as usize;
        // LAME: Just shift all indexes by a random amount and wrap around.
        for i in 0..num {
            datapoints[i] = datapoints[(i + shift) % num].clone();
        }
        return datapoints;
    }

    fn random_series(num: usize) -> Series<i32> {
        assert!(num < 500_000_000, "Extremely large number of datapoints requested");
        let datapoints = random_datapoints(num);
        let some_name_length = rand_in_range(1, 30) as usize;
        let series_name = random_ascii_string(some_name_length);
        let mut series = Series::new(series_name);

        for point in datapoints {
            &series.add_value(point);
        }

        return series;
    }

    #[test]
    fn create_series__unicode_stress_test__doesnt_panic() {
        let strs = strange_unicode_strings();
        for name in strs {
            let var: Series<i32> = Series::new(name);
        }
    }

    #[test]
    fn add_value__increases_length() {
        let mut series = Series::new("fps".to_string());
        let num_items = rand_in_range(1, 10);

        for _ in 0..num_items {
            &series.add_value(DataPoint(123123, 60));
        }

        assert_eq!(num_items, series.points.len() as i32);
    }

    #[test]
    fn len__with_content__returns_num_points() {
        let num_datapoints = rand_in_range(10, 100) as usize;
        let series = random_series(num_datapoints);
        assert_eq!(num_datapoints, series.len());
    }

    #[test]
    fn most_recent__enough_elements__returns_requested_amount() {
        let num_datapoints = rand_in_range(10, 100) as usize;
        let series = random_series(num_datapoints);

        let tail_size = rand_in_range(1, (num_datapoints / 2) as i32) as usize;
        let tail = series.most_recent(tail_size);

        assert_eq!(tail_size, tail.len());
    }

    #[test]
    fn most_recent__too_few_elements__returns_all() {
        let num_datapoints = 100;
        let series = random_series(num_datapoints);

        let tail_size = num_datapoints + rand_in_range(1, num_datapoints as i32 / 10) as usize;
        let tail = series.most_recent(tail_size);

        assert_eq!(num_datapoints, tail.len());
    }

    #[test]
    fn most_recent__items_added_unsorted__returns_sorted_by_time() {
        let datapoints = vec![
            DataPoint(3, 10),
            DataPoint(2, 10),
            DataPoint(4, 10),
            DataPoint(1, 10),
        ];
        let mut series = Series::new(random_ascii_string(10));
        for point in &datapoints {
            &series.add_value(point.clone());
        }
        let most_recent = series.most_recent(3);
        let correct_order = vec![
            datapoints[3].clone(),
            datapoints[0].clone(),
            datapoints[1].clone()
        ];
        assert_eq!(correct_order, most_recent);
    }
}
