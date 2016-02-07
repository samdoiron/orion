// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use entities::identified::Identified;

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    InvalidEntry,
    DuplicateEntry,
    IntegrityError
}

pub type RepoResult<T> = Result<T, RepoError>;

pub trait Repo<K, V> 
    where K: Ord + Eq + Clone,
          V: Identified<K> + Clone {

    // Does not check if already present, basically "put"
    fn add(&mut self, V) -> RepoResult<()>;

    fn get(&self, K) -> RepoResult<Option<V>>;

    // NOT upsert.
    fn update(&mut self, V) -> RepoResult<Option<()>>;

    fn remove(&mut self, K) -> RepoResult<Option<V>>;
}

pub struct StubRepo<V> 
    where V: Clone {
    add_response: RepoResult<()>,
    get_response: RepoResult<Option<V>>,
    update_response: RepoResult<Option<()>>,
    remove_response: RepoResult<Option<V>>
}

impl<V> StubRepo<V>
    where V: Clone {
    pub fn empty() -> StubRepo<V> {
        StubRepo {
            add_response: Ok(()),
            get_response: Ok(None),
            update_response: Ok(None),
            remove_response: Ok(None)
        }
    }

    pub fn containing(value: &V) -> StubRepo<V> {
        StubRepo {
            add_response: Ok(()),
            get_response: Ok(Some(value.clone())),
            update_response: Ok(None),
            remove_response: Ok(None)
        }
    }
}

impl<K, V> Repo<K, V> for StubRepo<V> 
    where K: Ord + Eq + Clone,
          V: Identified<K> + Clone {
    fn add(&mut self, _: V) -> RepoResult<()> {
        self.add_response.clone()
    }

    fn get(&self, _: K) -> RepoResult<Option<V>> {
        self.get_response.clone()
    }

    fn update(&mut self, _: V) -> RepoResult<Option<()>> {
        self.update_response.clone()
    }

    fn remove(&mut self, _: K) -> RepoResult<Option<V>> {
        self.remove_response.clone()
    }
}

impl<K, V> Repo<K, V> for BTreeMap<K, V>
    where K: Ord + Eq + Clone,
          V: Identified<K> + Clone {
    fn add(&mut self, value: V) -> RepoResult<()> {
        self.insert(value.id(), value);
        Ok(())
    }

    fn get(&self, id: K) -> RepoResult<Option<V>> {
        Ok(self.get(&id).map(|v| v.clone()))
    }

    fn update(&mut self, value: V) -> RepoResult<Option<()>> {
        let id = value.id();
        if !self.contains_key(&id) {
            return Ok(None)
        }
        self.insert(id, value);
        Ok(Some(()))
    }

    fn remove(&mut self, id: K) -> RepoResult<Option<V>> {
        Ok(self.remove(&id))
    }
}
