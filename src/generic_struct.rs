struct GenericStruct<T>(T);

struct Container<T> {
    item: T,
}

fn main() {
    let x = GenericStruct(5);
    let y = GenericStruct(String::from("hello"));
    let z = GenericStruct(String::from("world"));
    let a = Container { item: 5 };
}
