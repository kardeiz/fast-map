# fast-map

[![Docs](https://docs.rs/fast-map/badge.svg)](https://docs.rs/crate/fast-map/)
[![Crates.io](https://img.shields.io/crates/v/fast-map.svg)](https://crates.io/crates/fast-map)

A small library and custom derive to create a map-like struct that uses match expressions to `get` and `insert` values.

If you know your keys at compile-time, this library will likely be faster than `HashMap` for supported map operations.

Provides the following operations on the wrapping struct (via `derive` macros):

* `MyMap::get`, returns `Result<Option<&V>, Error>`
* `MyMap::get_mut`, returns `Result<Option<&mut V>, Error>`
* `MyMap::insert`, returns `Result<Option<V>, Error>`, where `V` is the old value if one exists
* `MyMap::remove`, returns `Result<Option<V>, Error>`
* `MyMap::values`, returns an iterator over `&V`s

## Usage

```rust

fn main() {
    pub enum A { A, B, C, D };

    #[derive(Default, fast_map::FastMap)]
    #[fast_map(keys(A::A, A::B, A::C, A::D))]
    struct Foo(fast_map::Map4<A, String>);

    let mut foo = Foo::default();

    foo.insert(A::B, "B".into()).unwrap();

    assert_eq!(foo.get(A::B).unwrap(), Some(&"B".to_string()));

    assert_eq!(foo.get(A::C).unwrap(), None);

    foo.insert(A::C, "C".into()).unwrap();

    assert_eq!(foo.values().collect::<Vec<_>>().len(), 2);
}
```

## Changelog

### 0.2.0

* Removed `easy` and `strict` `MapLike` traits. It's better to handle unknown keys explicitly, even for `get`s.
* Added `get_mut` operation to the wrapping struct


Current version: 0.2.0

License: MIT
