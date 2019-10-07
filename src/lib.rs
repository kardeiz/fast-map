/*!
A small library and custom derive to create map-like structs that use match expressions to `get` and `insert` fields.



A small library for creating string templates, similar to [eRuby](https://ruby-doc.org/stdlib/libdoc/erb/rdoc/ERB.html)
and [JSP](https://en.wikipedia.org/wiki/JavaServer_Pages) (uses angle-bracket-percent tags: `<%= expr %>`).

Templates are precompiled for speed and safety, though partial dynamic rendering is provided when the `dynamic` flag is
enabled (some setup is required).

# Usage

```rust,no_run

```

*/

pub use fast_map_derive::FastMap;

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

#[allow(unused)]
mod maps;

pub use maps::*;

pub struct Values<'fast_map, T>(pub std::vec::IntoIter<Option<&'fast_map T>>);

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

pub trait MapLike<K: ?Sized, T> {
    fn get<B: std::borrow::Borrow<K>>(&self, key: B) -> Option<&T>;

    fn insert<B: std::borrow::Borrow<K>>(&mut self, key: B, val: T) -> Option<T>;

    fn remove<B: std::borrow::Borrow<K>>(&mut self, key: B) -> Option<T>;

    fn values<'fast_map>(&'fast_map self) -> Values<'fast_map, T>;
}

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

        #[derive(Default, FastMap)]
        #[fast_map(strict, crate_name = "crate", keys(A::A, A::B, A::C, A::D))]
        struct Foo(crate::Map64<A, String>);

        let mut foo = Foo::new();

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

        let mut foo = Foo::new();

        let insert_x = String::from("x");

        foo.insert(insert_x.as_str(), &insert_x);

        assert_eq!(foo.values().collect::<Vec<_>>().len(), 1);

        let x = foo.remove("x").unwrap();

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
