pub use fast_map_derive::FastMap;

#[allow(unused)]
mod tups;

pub use tups::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {      

        pub enum A { B, C, D };

        #[derive(Default, FastMap)]
        #[fast_map(keys(A::B, A::C, A::D))]
        struct Foo(crate::Tup3<A, String>);

        let mut foo = Foo::default();

        foo.insert(&A::B, "STRING".into());

        let y = foo.get(&A::B);

        println!("{:?}", &y);

        let z = foo.get(&A::C);

        println!("{:?}", &z);

    }
}
