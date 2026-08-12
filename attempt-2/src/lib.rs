//! Attempt 2 — `Rc<RefCell<Node>>` (Part 5).
//!
//! The first one that works. `Rc` gives shared ownership (so a node can be
//! pointed at by both its neighbour and the map), and `RefCell` moves the
//! aliasing-XOR-mutation check from compile time to *run* time. Same rule, later
//! enforcement: instead of a compiler error you get a `BorrowMutError` panic if
//! you ever hold two live borrows of the same cell.
//!
//! Back-links that would form an `Rc` cycle should be `Weak` so the cache can
//! actually drop.
//!
//! Once it works, do the part-5 exercise: deliberately provoke a `BorrowMutError`
//! (borrow a cell while a borrow of it is still live) and read the panic. That
//! panic is exactly the bug the Go program in post-01 had — silently.

pub struct LruCache {
    // TODO: HashMap<i32, Rc<RefCell<Node>>> + Rc/Weak doubly-linked list.
}

impl LruCache {
    pub fn new(_capacity: usize) -> Self {
        todo!("attempt 2: Rc<RefCell<Node>>")
    }

    pub fn get(&mut self, _key: i32) -> Option<i32> {
        todo!()
    }

    pub fn put(&mut self, _key: i32, _value: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "TODO: implement attempt 2"]
    fn evicts_least_recently_used() {
        let mut c = LruCache::new(2);
        c.put(1, 1);
        c.put(2, 2);
        assert_eq!(c.get(1), Some(1));
        c.put(3, 3);
        assert_eq!(c.get(2), None);
        assert_eq!(c.get(3), Some(3));
    }
}
