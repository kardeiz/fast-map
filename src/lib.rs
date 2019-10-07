pub use fast_map_derive::FastMap;

#[allow(unused)]
mod maps;

pub use maps::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {      

        pub enum A { A, B, C, D };

        #[derive(Default, FastMap)]
        #[fast_map(keys(A::A, A::B, A::C, A::D))]
        struct Foo(crate::Map4<A, String>);

        let mut foo = Foo::new();

        foo.insert(A::B, "B".into());

        assert_eq!(foo.get(A::B), Some(&"B".to_string()));

        assert_eq!(foo.get(A::C), None);

        foo.insert(A::C, "C".into());

        assert_eq!(foo.values().collect::<Vec<_>>().len(), 2);
    }

    #[test]
    fn it_works_2() {      

        #[derive(Default, FastMap)]
        #[fast_map(keys("x", "y", "z"))]
        struct Foo<'a>(crate::Map3<str, &'a str>);

        let mut foo = Foo::new();

        foo.insert("x", "X");

        let x = foo.remove("x").unwrap();

        assert_eq!(x, "X");

        assert!(foo.values().collect::<Vec<_>>().is_empty());

    }

    #[test]
    fn it_works_3() {      

        #[derive(Default, FastMap)]
        #[fast_map(keys(1, 2, 3))]
        struct Foo<'a>(crate::Map3<usize, &'a str>);

        let mut foo = Foo::default();

        foo.insert(1, "1");
        foo.insert(2, "2");
        foo.insert(3, "3");
        foo.insert(4, "4");

        let x = foo.remove(1);

        assert_eq!(foo.values().map(|x| *x).collect::<Vec<_>>(), vec!["2", "3"]);

    }
}
