pub use fast_map_derive::FastMap;

#[allow(unused)]
mod maps;

pub use maps::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {      

        pub enum A { B, C, D };

        #[derive(Default, FastMap)]
        #[fast_map(keys(A::B, A::C, A::D))]
        struct Foo(crate::Map3<A, String>);

        let mut foo = Foo::default();

        foo.insert(A::B, "STRING".into());

        let y = foo.get(A::B);

        println!("{:?}", &y);

        let z = foo.get(&A::C);

        println!("{:?}", &z);

        foo.insert(A::C, "2".into());

        for f in foo.values() {
            println!("{:?}", &f);
        }
    }

    #[test]
    fn it_works_2() {      

        #[derive(Default, FastMap)]
        #[fast_map(keys("x", "y", "z"))]
        struct Foo<'a>(crate::Map3<str, &'a str>);

        let mut foo = Foo::default();

        foo.insert("x", "X");

        let x = foo.remove("x");

        println!("{:?}", &x);

        for f in foo.values() {
            println!("{:?}", &f);
        }
    }

    #[test]
    fn it_works_3() {      

        #[derive(Default, FastMap)]
        #[fast_map(keys(1, 2, 3))]
        struct Foo<'a>(crate::Map3<usize, &'a str>);

        let mut foo = Foo::default();

        foo.insert(1, "1");

        let x = foo.remove(1);

        println!("{:?}", &x);

        for f in foo.values() {
            println!("{:?}", &f);
        }

    }
}
