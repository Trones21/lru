# Part 1 — the rule you already follow

Two versions of the same four-line program. One compiles and is silently wrong;
the other refuses to compile and tells you why. That contrast is the whole post.

## Go — compiles, silently wrong

```bash
cd go
go run main.go      # prints [1 2 3 4] — the write to *p is lost
```

Then give the slice spare capacity (see the comment in `main.go`) so `append`
doesn't reallocate, and watch the bug disappear. Same code, one variable changed.

## Rust — refuses to compile

```bash
cd rust
rustc main.rs       # does NOT build — error[E0502]
```

Read the error in full. Then try to make it compile by moving things around —
that's the door part 2 walks through.

Neither of these is an LRU cache yet. The cache starts in `attempt-1/` (part 4),
once the rule from this post has had three more posts to sink in.
