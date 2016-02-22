use std::hash::Hash;

use table::RawTable;

mod table;

const REPROBE_LIMIT: u32 = 10;

pub struct ClickMap<K, V> {
    table: [TableRow; 64],
}

pub struct TableRow<K, V> {
    key: K,
    value: V,
}

impl<K, V> ClickMap<K, V>
    where K: Eq + Hash, P: Prober {

    pub fn new() -> ClickMap<K, V, LinearProber> {
        ClickMap {
            table: [TableRow<K, V> ; 64],
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = self.make_hash(&k);
        None
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<V> {
        None
    }

    fn make_hash<H: ?Sized + Hash>(&self, k: &H) -> u64 {
        1337
    }
}
