# fast-map

[![Docs](https://docs.rs/fast-map/badge.svg)](https://docs.rs/crate/fast-map/)
[![Crates.io](https://img.shields.io/crates/v/fast-map.svg)](https://crates.io/crates/fast-map)

A small library and custom derive to create a map-like struct that uses match expressions to `get` and `insert` values.

If you know your keys at compile-time, this library will likely be faster than `HashMap` for supported map operations.

Provides map operations through `strict::MapLike`, which returns an error when attempting to use unknown keys, and
`easy::MapLike`, which ignores missing keys and more closely matches the `HashMap` API.

## Usage

```rust
use fast_map::easy::MapLike;

fn main() {
    pub enum A { A, B, C, D };

    #[derive(Default, fast_map::FastMap)]
    #[fast_map(keys(A::A, A::B, A::C, A::D))]
    struct Foo(fast_map::Map4<A, String>);

    let mut foo = Foo::default();

    foo.insert(A::B, "B".into());

    assert_eq!(foo.get(A::B), Some(&"B".to_string()));

    assert_eq!(foo.get(A::C), None);

    foo.insert(A::C, "C".into());

    assert_eq!(foo.values().collect::<Vec<_>>().len(), 2);
}
```

Current version: 0.1.1

License: MIT
