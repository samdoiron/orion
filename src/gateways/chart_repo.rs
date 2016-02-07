// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use use_cases::repos::{Repo, RepoResult};

use entities::identified::Identified;
use entities::chart;
use entities::chart::{Histogram};

use std::collections::BTreeMap;
use std::mem;

macro_rules! impl_in_memory_for {
    ( $( $chart:ident, $map:ident ),* ) => {
        impl InMemory {
            fn new() -> InMemory {
                InMemory {
                $(
                    $map: BTreeMap::new()
                 )*
                }
            }
        }
        $(
            impl Repo<chart::Id, $chart> for InMemory {
                fn add(&mut self, item: $chart) -> RepoResult<()> {
                    let id = item.id();
                    self.$map.insert(id, item);
                    Ok(())
                }

                fn get(&self, id: chart::Id) -> RepoResult<Option<$chart>> {
                    Ok(self.$map.get(&id).map(|c| c.clone()))
                }

                fn update(&mut self, new_chart: $chart) 
                    -> RepoResult<Option<()>> {

                    let updated = self.$map.get_mut(&new_chart.id())
                        .map(|old| mem::replace(old, new_chart));

                    Ok(updated.map(|_| ()))
                }

                fn remove(&mut self, id: chart::Id)
                    -> RepoResult<Option<$chart>> {
                    Ok(self.$map.remove(&id))
                }
            }
         )*
    }
}

struct InMemory {
    histograms: BTreeMap<chart::Id, Histogram>
}

// Can't generate the struct above, because concat_ident! doesn't work for
// keys in struct definitions.
impl_in_memory_for! {
    Histogram, histograms
}
