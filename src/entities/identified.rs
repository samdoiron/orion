// Copyright (C) 2015  Samuel Doiron, see LICENSE for details

pub trait Identified<T>
    where T: Ord + Clone + Eq {
    fn id(&self) -> T;
}

