//! Attempt 4 — raw pointers in `unsafe` (Part 7).
//!
//! What `std::collections::LinkedList` actually does. Nodes are heap-allocated
//! and linked with `*mut Node`; you take on the aliasing rules yourself instead
//! of proving them to the compiler. This is the fast, sharp version — and the
//! one where you can be wrong in ways that still compile and still pass tests.
//!
//! That's the point of the post. After it works, run it under Miri:
//!
//! ```text
//! cargo +nightly miri test -p attempt-4
//! ```
//!
//! Miri checks your `unsafe` against Stacked Borrows, the formal aliasing model —
//! the layer underneath the compiler. There's a good chance it flags something
//! your tests didn't. If it does, that finding IS the post; if it doesn't, write
//! about what it was checking for.
//!
//! Keep the `unsafe` surface small and document the invariant each block relies
//! on. `NonNull<Node>` is the idiomatic pointer type here.

pub struct LruCache {
    // TODO: raw *mut Node / NonNull<Node> links, manual alloc/dealloc.
}

impl LruCache {
    pub fn new(_capacity: usize) -> Self {
        todo!("attempt 4: unsafe raw-pointer list, then run Miri")
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
    #[ignore = "TODO: implement attempt 4"]
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
