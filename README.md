# lru

Four LRU (least-recently-used) caches, each implemented a different way in Rust —
the companion code for a hands-on series on **the borrow checker**.

The series is one long argument: the borrow checker isn't bureaucracy, it's a
rule you already follow, and when your data doesn't fit the rule there's a design
space of ways out — each with a real cost. A doubly-linked LRU cache is the
smallest honest example of data that *doesn't* fit, so it gets built four times,
once per escape hatch.

> **Read the series alongside the code — it's written to be run, not just read.**
> Start here: https://thomasrones.com/technical/languages/rust/the-rule-you-already-follow

## Layout

Each implementation exposes the same tiny API (`new` / `get` / `put`) and is
meant to pass the same test, so you can diff them against each other. The
implementations are left as `todo!()` on purpose — this repo is a workbench, and
doing them yourself is the entire point.

| dir | approach | the series post it belongs to |
|---|---|---|
| `post-01/` | the aliasing-XOR-mutation demo (Go + Rust), not an LRU yet | Part 1 — the rule you already follow |
| `attempt-1/` | naive `Box<Node>` + `&mut` links — the one that fights you | Part 4 — ownership has to be a tree |
| `attempt-2/` | `Rc<RefCell<Node>>` — the check moved to runtime | Part 5 — move the check to runtime |
| `attempt-3/` | arena: nodes in a `Vec`, links are `usize` indices | Part 6 — make it a tree by fiat |
| `attempt-4/` | `unsafe` with `*mut Node`, checked by Miri | Part 7 — raw pointers, and the tool that checks them |

## Running it

```bash
git clone https://github.com/Trones21/lru
cd lru

# the whole workspace builds today (the attempts are stubs)
cargo build

# implement an attempt, then run its test (remove the #[ignore] first)
cargo test -p attempt-2

# part 1 — the two demo programs (standalone, not in the workspace)
cd post-01 && cat README.md

# the Miri pass from part 7 (needs nightly)
cargo +nightly miri test -p attempt-4
```

## How to actually use this

Reading a compiler error in a browser is not the same as producing one. The point
of the repo is to **break things on purpose**: attempt the naive version and read
the errors, trigger a `BorrowMutError`, run the benchmark, run Miri. Each post
ends with a short list of specific things to try — do those. If you get output
that differs from the posts (different toolchain, different error text), the site
has comments; that kind of "here's what mine did" is genuinely useful.

## Status

Early — the scaffold is here; implementations land as the series posts are
written and the exercises are actually run. Watch or star to catch them as they
go up.
