#![allow(unused)]

use exhaustive_macros::{exhaustive_test, Exhaustive};

fn main() {}

#[derive(Debug, Exhaustive)]
enum Test1 {
    A(bool),
    B { x: bool },
}

#[derive(Debug, Exhaustive)]
struct Test2 {
    a: bool,
}

#[derive(Debug, Exhaustive)]
enum Generic<T> {
    A(T),
    B(T),
}

#[exhaustive_test]
fn test(v: Test1, w: Test2) {
    println!("{v:?} {w:?}");
}
