#![feature(test)]
// #![feature(proc_macro_hygiene)]

extern crate fast_map;
extern crate test;

// #[macro_use]
// extern crate phf_macros;

#[bench]
fn bench_fast_map(b: &mut test::Bencher) {
    use fast_map::easy::MapLike;

    pub enum A {
        A,
        B,
        C,
        D,
    };

    #[derive(Default, fast_map::FastMap)]
    #[fast_map(keys(A::A, A::B, A::C, A::D))]
    struct Foo(fast_map::Map4<A, String>);

    let mut foo = Foo::default();

    b.iter(|| {
        foo.insert(A::A, "A".to_string());
        foo.insert(A::B, "B".to_string());

        (foo.get(&A::A).cloned(), foo.get(&A::B).cloned(), foo.get(&A::C).cloned())
    })
}

#[bench]
fn bench_hash_map(b: &mut test::Bencher) {
    #[derive(PartialEq, Eq, Hash)]
    pub enum A {
        A,
        B,
        C,
        D,
    };

    let mut foo = std::collections::HashMap::new();

    b.iter(|| {
        foo.insert(A::A, "A".to_string());
        foo.insert(A::B, "B".to_string());

        (foo.get(&A::A).cloned(), foo.get(&A::B).cloned(), foo.get(&A::C).cloned())
    })
}

// #[bench]
// fn bench_phf(b: &mut test::Bencher) {
//     // use phf::phf_map;

//     #[derive(Clone)]
//     pub enum Keyword {
//         Loop,
//         Continue,
//         Break,
//         Fn,
//         Extern,
//     }

//     static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
//         "loop" => Keyword::Loop,
//         "continue" => Keyword::Continue,
//         "break" => Keyword::Break,
//         "fn" => Keyword::Fn,
//         "extern" => Keyword::Extern,
//     };

//     b.iter(|| {
//         (
//             KEYWORDS.get("loop").cloned(),
//             KEYWORDS.get("break").cloned(),
//             KEYWORDS.get("extern").cloned(),
//         )
//     })
// }

#[bench]
fn bench_fast_map_2(b: &mut test::Bencher) {
    use fast_map::easy::MapLike;

    #[derive(Clone)]
    pub enum Keyword {
        Loop,
        Continue,
        Break,
        Fn,
        Extern,
    }

    #[derive(Default, fast_map::FastMap)]
    #[fast_map(keys("loop", "continue", "break", "fn", "extern"))]
    struct Keywords(fast_map::Map5<str, Keyword>);

    let mut keywords = Keywords::default();

    keywords.insert("loop", Keyword::Loop);
    keywords.insert("continue", Keyword::Loop);
    keywords.insert("break", Keyword::Break);
    keywords.insert("fn", Keyword::Fn);
    keywords.insert("extern", Keyword::Extern);

    b.iter(|| {
        (
            keywords.get("loop").cloned(),
            keywords.get("break").cloned(),
            keywords.get("extern").cloned(),
        )
    })
}
