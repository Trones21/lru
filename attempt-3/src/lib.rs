//! Attempt 3 — arena allocation, links are indices (Part 6).
//!
//! Make it a tree by fiat. All nodes live in a single `Vec<Node>`; "pointers"
//! become `usize` indices into that Vec. Ownership is now trivially a tree (the
//! Vec owns every node), and the links are just numbers the borrow checker never
//! has to reason about — so no `Rc`, no `RefCell`, no `unsafe`.
//!
//! Use a sentinel or `Option<usize>` for null links, and keep a free-list of
//! slots so removed nodes' indices can be reused instead of growing forever.
//!
//! Part 6 is where this stops being about syntax and starts being about layout:
//! benchmark it against attempt-2 with criterion, and explain the delta — you're
//! trading pointer chasing and refcount traffic for contiguous memory.

pub struct LruCache {
    // TODO: Vec<Node> arena, usize indices for prev/next, head/tail, free-list.
}

impl LruCache {
    pub fn new(_capacity: usize) -> Self {
        todo!("attempt 3: Vec-backed arena with usize links")
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
    #[ignore = "TODO: implement attempt 3"]
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
