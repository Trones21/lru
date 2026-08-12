// The same program as post-01/go/main.go — but Rust refuses to compile it.
//
// `p` is a shared borrow of `s`. `s.push(4)` needs an exclusive borrow (it might
// reallocate). The compiler won't let the exclusive borrow exist while the shared
// borrow is still going to be used on the println line.
//
// Run it:  rustc main.rs && ./main
// Expect:  it does NOT build. You get something like:
//
//   error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
//    --> main.rs:N:5
//     |
//     |     let p = &s[0];
//     |              - immutable borrow occurs here
//     |     s.push(4);
//     |     ^^^^^^^^^ mutable borrow occurs here
//     |     println!("{}", *p);
//     |                    -- immutable borrow later used here
//
// Paste YOUR exact error into the article's comments — it may differ by toolchain.
//
// Then try to make it compile: move `push` before you take `p`, or finish using
// `p` before the `push`. That "you can't have both AT THE SAME TIME" is part 2.
fn main() {
    let mut s = vec![1, 2, 3];
    let p = &s[0];
    s.push(4);
    println!("{}", *p);
}
