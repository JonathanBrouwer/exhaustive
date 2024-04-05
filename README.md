<div align="center">
  <h1><code>Exhaustive</code></h1>
  <p><strong>The trait for generating all values of a type, and a property-based test macro.</strong></p>
</div>

## About

This crate provides a trait to generate all values (up to a certain depth) of a type. 
It also provides a derive macro that will derive this trait for you.
Finally, it provides a test macro that run a property-based test for all values of a type. 

## Example

```rust
#[derive(Debug, Exhaustive)]
enum Test1 { A, B, }
#[derive(Debug, Exhaustive)]
struct Test2 { a: bool }

#[exhaustive_test]
fn test(v: Test1, w: Test2) {
    println!("{v:?} {w:?}");
}
```

```text
A Test2 { a: false }
A Test2 { a: true }
B Test2 { a: false }
B Test2 { a: true }
```

## Arbitrary-length types (Recursive types, container types)

In order to implement `Exhaustive` for recursive types or arbitrary-length types such as `Vec`s, the crate supports limiting the size of types. 
This is implemented by limiting the amount of choices that can be made during the generation of the type. 

When using the `exhaustive_test` macro, you can limit the amount of choices using:
```rust
#[exhaustive_test(20)]
```

The amount of choices that is made during generation of a type is not guaranteed to be stable between releases, though it is always guaranteed to be **linear** with the size of type in memory. 
This means that for most types, the amount of choices is **exponential** with the runtime of the test.
The recommended way of choosing this value is to pick it experimentally based on the desired runtime of the test.

## Details

### Standard library types

The trait is implemented for most standard library types, such as `Vec`, `Result`, `Option`, `Box`, `HashMap`, `HashSet`, tuples, arrays, etc. 
The primary exception is the number types, since these grow too quickly to exhaustively test. 

### Derive macro

The derive macro can derive `Exhaustive` for any `struct` or `enum`, whose fields all implement `Exhaustive`.
For this to work, the `macros` feature of this crate needs to be enabled. This feature is enabled by default.

### Implementing by hand

The trait can be implemented by hand. The signature of the trait is:
```rust
trait Exhaustive: Sized {
    fn generate(u: &mut DataSourceTaker) -> Result<Self>;
}
```
The `DataSourceTaker` has two important functions that can be used to generate your type:
```rust
fn choice(&mut self, range: usize) -> Result<usize>;
fn iter_of<T: Exhaustive>(&mut self) -> Result<DataSourceTakerIter<T>>;
```
- `choice` will generate a number in `0..range`. 
  This can be used to choose a variant of an enum, or to make any other choice you need to make while generating your datatype.
- `iter_of` will generate an iterator of `T`. 
  This iterator will always be finitely-sized, and should be fully consumed.
  `iter_of` makes 1 choice to determine the size of the iterator, after which the generation of the elements may make more choices.

### Using this crate without the exhaustive_test macro

Any type that implements the `Exhaustive` trait will have an auto-implementation of the following function:

```rust
fn iter_exhaustive(max_choices: Option<usize>) -> impl Iterator<Item=Self>;
```

This function provides an iterator of all generated values of the type with optionally a maximum number of choices.

### Comparison with arbitrary

The [arbitrary](https://crates.io/crates/arbitrary) crate was a big inspiration for this crate. 
It provides similar functionality to this crate, but it made a few different design decisions that make it unusable for efficient exhaustive testing.

Types are always generated from a byte slice, which means that it is not possible to make exact choices, only to read a byte and make a choice depending on that. 
This is fine for the intended use case of `arbitrary`, which is generating arbitrary values. 
But to generate all values exhaustively this means we'd have to generate all byte sequences up to possibly a big size. This ends up generating a LOT of duplicate values, and therefore scales poorly.