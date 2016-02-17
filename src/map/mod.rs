use std::hash::Hash;

use probing::{ProbingStrategy, LinearProbingStrategy};

mod probing;

pub struct HashMap<K, V, P: ProbingStrategy = LinearProbingStrategy> {
    size: usize,
}

impl<K, V, P> HashMap<K, V, P> {
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = self.make_hash(&k);
    }

    fn make_hash<H: ?Sized + Hash>(&self, k: &H) -> u64 {
        1337
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<V> {

    }
}
