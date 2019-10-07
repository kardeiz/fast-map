/*!
A small library and custom derive to create a map-like struct that uses match expressions to `get` and `insert` values.

If you know your keys at compile-time, this library will likely be faster than `HashMap` for supported map operations.

Provides map operations through `strict::MapLike`, which returns an error when attempting to use unknown keys, and
`easy::MapLike`, which ignores missing keys and more closely matches the `HashMap` API.

# Usage

```rust,no_run
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
*/

pub use fast_map_derive::FastMap;

/// Currently just `KeyNotFound`
#[derive(Debug)]
pub enum Error {
    KeyNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::KeyNotFound => "Key not found".fmt(f),
        }
    }
}

impl std::error::Error for Error {}

mod maps;

pub use maps::*;

/// Iterator over existing values
pub struct Values<'fast_map, T>(std::vec::IntoIter<Option<&'fast_map T>>);

impl<'fast_map, T> Values<'fast_map, T> {
    #[doc(hidden)]
    pub fn new(t: std::vec::IntoIter<Option<&'fast_map T>>) -> Self {
        Values(t)
    }
}

impl<'fast_map, T> Iterator for Values<'fast_map, T> {
    type Item = &'fast_map T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Some(ref inner)) => {
                    return Some(inner);
                }
                Some(None) => {
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

/// Provides a `MapLike` trait that ignores errors on missing keys (more like `HashMap` API)
pub mod easy {

    use super::{strict, Values};

    pub trait MapLike<K: ?Sized, T>: strict::MapLike<K, T> {
        fn get<B: std::borrow::Borrow<K>>(&self, key: B) -> Option<&T> {
            strict::MapLike::get(self, key).ok().and_then(|x| x)
        }

        fn insert<B: std::borrow::Borrow<K>>(&mut self, key: B, val: T) -> Option<T> {
            strict::MapLike::insert(self, key, val).ok().and_then(|x| x)
        }

        fn remove<B: std::borrow::Borrow<K>>(&mut self, key: B) -> Option<T> {
            strict::MapLike::remove(self, key).ok().and_then(|x| x)
        }

        fn values<'fast_map>(&'fast_map self) -> Values<'fast_map, T> {
            strict::MapLike::values(self)
        }
    }

    impl<K: ?Sized, T, U> MapLike<K, T> for U where U: strict::MapLike<K, T> {}
}

/// Provides a `MapLike` trait that errors when trying to `get` or `insert` missing keys
pub mod strict {

    use super::{Error, Values};

    pub trait MapLike<K: ?Sized, T> {
        fn get<B: std::borrow::Borrow<K>>(&self, key: B) -> std::result::Result<Option<&T>, Error>;

        fn insert<B: std::borrow::Borrow<K>>(
            &mut self,
            key: B,
            val: T,
        ) -> std::result::Result<Option<T>, Error>;

        fn remove<B: std::borrow::Borrow<K>>(
            &mut self,
            key: B,
        ) -> std::result::Result<Option<T>, Error>;

        fn values<'fast_map>(&'fast_map self) -> Values<'fast_map, T>;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        pub enum A {
            A,
            B,
            C,
            D,
        };

        use strict::MapLike;

        #[derive(Default, FastMap)]
        #[fast_map(crate_name = "crate", keys(A::A, A::B, A::C, A::D))]
        struct Foo(crate::Map64<A, String>);

        let mut foo = Foo::default();

        foo.insert(A::B, "B".into()).unwrap();

        assert_eq!(foo.get(A::B).unwrap(), Some(&"B".to_string()));

        assert_eq!(foo.get(A::C).unwrap(), None);

        foo.insert(A::C, "C".into()).unwrap();

        assert_eq!(foo.values().collect::<Vec<_>>().len(), 2);
    }

    #[test]
    fn it_works_2() {
        use easy::MapLike;

        #[derive(Default, FastMap)]
        #[fast_map(crate_name = "crate", keys("x", "y", "z"))]
        struct Foo<'a>(crate::Map3<str, &'a str>);

        let mut foo = Foo::default();

        let insert_x = String::from("x");

        foo.insert(insert_x.as_str(), &insert_x);

        assert_eq!(foo.values().collect::<Vec<_>>().len(), 1);

        let x = foo.remove("x").unwrap();

        assert_eq!(x, "x");

        assert!(foo.values().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn it_works_3() {
        use crate::easy::MapLike;

        #[derive(Default, FastMap)]
        #[fast_map(crate_name = "crate", keys(1, 2, 3))]
        struct Foo<'a>(crate::Map3<usize, &'a str>);

        let mut foo = Foo::default();

        foo.insert(1, "1");
        foo.insert(2, "2");
        foo.insert(3, "3");
        foo.insert(4, "4");

        let _ = foo.remove(1);

        assert_eq!(foo.values().map(|x| *x).collect::<Vec<_>>(), vec!["2", "3"]);
    }
}
