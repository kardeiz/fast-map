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

If you know that your operations cannot fail (e.g. if your key type is an `enum`, and you list all variants as keys),
you can add `infallible = true` to your derive attributes, which will `unwrap` the result of your map operations.

## Usage

```rust

fn main() {
    pub enum A { A, B, C, D };

    #[derive(Default, fast_map::FastMap)]
    // We know this cannot fail, since we list all the `enum` variants, so we add `infallible = true`
    #[fast_map(infallible = true, keys(A::A, A::B, A::C, A::D))]
    struct Foo(fast_map::Map4<A, String>);

    let mut foo = Foo::default();

    foo.insert(A::B, "B".into());

    assert_eq!(foo.get(A::B), Some(&"B".to_string()));

    assert_eq!(foo.get(A::C), None);

    foo.insert(A::C, "C".into());

    assert_eq!(foo.values().collect::<Vec<_>>().len(), 2);
}
```

## Changelog

### 0.2.1

* Add the non-erroring operations back as depending on macro attribute (`infallible = true`). Default is `false`.

### 0.2.0

* Removed `easy` and `strict` `MapLike` traits. It's better to handle unknown keys explicitly, even for `get`s.
* Added `get_mut` operation to the wrapping struct


<hr/>

Current version: 0.2.1

License: MIT
