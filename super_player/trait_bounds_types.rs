use std::fmt::Display;

struct Foo<T: Display> {
    bar: T,
}

struct Bar<F>
where
    F: Display,
{
    bar: F,
}

fn main() {
    let foo = Foo {
        bar: "hello".to_string(),
    };
    let bar = Bar {
        bar: "hello".to_string(),
    };
}
