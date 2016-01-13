// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use std::marker::PhantomData;
use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Id<T>(u32, PhantomData<T>);

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

pub fn make_id<T>(i: u32) -> Id<T> {
    Id(i, PhantomData)
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<T> Eq for Id<T> {}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        return make_id::<T>(self.0)
    }
}

impl<T> Copy for Id<T> {}

pub trait Repo<T> {
    fn store(&mut self, T) -> Id<T>;
    fn get(&self, Id<T>) -> Option<&T>;
}
