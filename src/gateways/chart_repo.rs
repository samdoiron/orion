// Copyright (C) 2015  Samuel Doiron
use use_cases::repos::{ChartRepo, Id, make_id};
use entities::charts::Histogram;
use std::collections::BTreeMap;
use std::marker::PhantomData;

struct IdGenerator<T> {
    next_id: u32,
    _phantom: PhantomData<T>
}

impl<T> IdGenerator<T> {
    fn new() -> IdGenerator<T> {
        IdGenerator { next_id: 1, _phantom: PhantomData }
    }

    fn next(&mut self) -> Id<T> {
        let current = self.next_id;
        self.next_id += 1;
        make_id::<T>(current)
    }
}

macro_rules! impl_in_memory_for {
    ( $( $chart:ident, $map:ident, $ids:ident ),* ) => {
        impl InMemoryRepo {
            fn new() -> InMemoryRepo {
                InMemoryRepo {
                $(
                    $map: BTreeMap::new(),
                    $ids: IdGenerator::new(),
                 )*
                }
            }
        }
        $(
            impl ChartRepo<$chart> for InMemoryRepo {
                fn store(&mut self, item: $chart) -> Id<$chart> {
                    let id = self.$ids.next();
                    self.$map.insert(id.clone(), item);
                    id
                }

                fn get(&self, id: Id<$chart>) -> Option<&$chart> {
                    self.$map.get(&id)
                }
            }
         )*
    }
}

struct InMemoryRepo {
    histogram_ids: IdGenerator<Histogram>,
    histograms: BTreeMap<Id<Histogram>, Histogram>
}

// Can't generate the struct above, because concat_ident! doesn't work for
// keys in struct definitions.
impl_in_memory_for! {
    Histogram, histograms, histogram_ids
}
