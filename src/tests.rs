use rand;
use std::char;
use rand::distributions::{IndependentSample, Range};
use rustc_serialize::json;

pub fn rand_in_range(start: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let valid_range = Range::new(start, end);
    return valid_range.ind_sample(&mut rng);
}

pub fn random_ascii_string(length: usize) -> String {
    let mut string = String::with_capacity(length);
    let mut i = 0;
    while i < length {
        let c = char::from_u32(rand_in_range('!' as i32, 'z' as i32) as u32);
        if c.is_some() {
            string.push(c.unwrap());
            i += 1;
        }
    }
    return string;
}

pub fn strange_unicode_strings() -> Vec<String> {
    let naughty_strings = include_str!("blns.json");

    // This parse should *never* fail, so it's Ok to just unwrap.
    let parsed = json::decode(naughty_strings).unwrap();
    return parsed;
}

