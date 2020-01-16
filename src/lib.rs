/*!
A small library and custom derive to create a map-like struct that uses match expressions to `get` and `insert` values.

If you know your keys at compile-time, this library will likely be faster than `HashMap` for supported map operations.

Provides the following operations on the wrapping struct (via `derive` macros):

* `MyMap::get`, returns `Result<Option<&V>, Error>`
* `MyMap::get_mut`, returns `Result<Option<&mut V>, Error>`
* `MyMap::insert`, returns `Result<Option<V>, Error>`, where `V` is the old value if one exists
* `MyMap::remove`, returns `Result<Option<V>, Error>`
* `MyMap::values`, returns an iterator over `&V`s

# Usage

```rust,no_run

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

# Changelog

## 0.2.0

* Removed `easy` and `strict` `MapLike` traits. It's better to handle unknown keys explicitly, even for `get`s.
* Added `get_mut` operation to the wrapping struct

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

        #[derive(Default, FastMap)]
        #[fast_map(crate_name = "crate", keys("x", "y", "z"))]
        struct Foo<'a>(crate::Map3<str, &'a str>);

        let mut foo = Foo::default();

        let insert_x = String::from("x");

        foo.insert(insert_x.as_str(), &insert_x).unwrap();

        assert_eq!(foo.values().collect::<Vec<_>>().len(), 1);

        let x = foo.remove("x").ok().flatten().unwrap();

        assert_eq!(x, "x");

        assert!(foo.values().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn it_works_3() {

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
