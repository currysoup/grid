use std::mem::size_of;
use std::ptr;

pub struct RawTable<K, V> {
    size: usize,
    cap: usize,
    next_table: *mut RawTable,
}

struct Cell<K, V> {
    hash: u64,
    key: K,
    value: V,
}

const CELLS_PER_NEIGHBOURHOOD: usize = 4;

struct Neighbourhood {
    cells: [Cell; CELLS_PER_NEIGHBOURHOOD],
}

const FLAG_TOMBSTONE: u8 = 0;
const FLAG_FORWARD: u8 = 1;
const FLAG_EMPTY: u8 = 2;

// Must be a power of 2
const DEFAULT_SIZE: usize = 64;

impl<K, V> RawTable<K, V> {
    pub fn new(cap: usize) -> RawTable {
        RawTable {
            size: 0,
            cap: cap,
            next_table: ptr::null_mut,
        }
    }

    // Use hopscotch hashing to find a cell to store the data in
    fn find_cell(&self, hash: u64) -> *mut Cell {
        // Put `self.size - 1` as a struct member
        let idx = hash & (self.size - 1)
    }
}

