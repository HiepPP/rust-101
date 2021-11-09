struct Container<T> {
    item: T,
}
//When writing an impl block for any generic type, need to declare the generic type parameters before using it within the impl block.
impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

fn main() {
    let c = Container::new(42);
    println!("{}", c.item);
}
