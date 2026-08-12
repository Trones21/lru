//! Attempt 1 — naive `Box<Node>` + `&mut` links (Part 4).
//!
//! The honest failure. A doubly-linked list wants every node to point at its
//! neighbour AND be pointed at by it — ownership as a graph, not a tree. Try to
//! build it with `Box<Node>` for forward links and `&mut Node` (or `&Node`) for
//! back-pointers and document the exact sequence of errors as you fight it.
//!
//! Don't reach for `Rc`/`RefCell`/`unsafe` here — the point of this attempt is to
//! get stuck, and to understand *why* the naive shape can't work before you're
//! shown the ways around it. Capture the errors; they're the post.
//!
//! The API below is the target every attempt shares.

pub struct LruCache {
    // TODO: Box<Node> links. Expect to get stuck on the back-pointers.
}

impl LruCache {
    pub fn new(_capacity: usize) -> Self {
        todo!("attempt 1: build the naive Box/&mut version and record the errors")
    }

    /// Returns the value for `key` and marks it most-recently-used.
    pub fn get(&mut self, _key: i32) -> Option<i32> {
        todo!()
    }

    /// Inserts/updates `key`, marks it most-recently-used, evicting the LRU
    /// entry if over capacity.
    pub fn put(&mut self, _key: i32, _value: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "TODO: implement attempt 1"]
    fn evicts_least_recently_used() {
        let mut c = LruCache::new(2);
        c.put(1, 1);
        c.put(2, 2);
        assert_eq!(c.get(1), Some(1)); // touch 1 -> 2 is now the LRU entry
        c.put(3, 3); // over capacity -> evict key 2
        assert_eq!(c.get(2), None);
        assert_eq!(c.get(3), Some(3));
    }
}
