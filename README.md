<div align="center">
  <h1><code>Exhaustive</code></h1>
  <p><strong>The trait for generating all values of a type, and a property-based test macro.</strong></p>
</div>

## About

This crate provides a trait to generate all values (up to a certain depth) of a type. 
It also provides a derive macro that will derive this trait for you.
Finally, it provides a test macro that run a property-based test for all values of a type.

## Example

```rs
#[derive(Debug, Exhaustive)]
enum Test1 {
    A,
    B,
}

#[derive(Debug, Exhaustive)]
struct Test2 {
    a: bool,
}

#[exhaustive_test]
fn test(v: Test1, w: Test2) {
    println!("{v:?} {w:?}");
}
```

```
A Test2 { a: false }
A Test2 { a: true }
B Test2 { a: false }
B Test2 { a: true }
```