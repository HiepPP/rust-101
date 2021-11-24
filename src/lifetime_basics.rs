struct SomeRef<T> {
    part: &T, //expected named lifetime parameter
}

fn main() {
    let a = SomeRef { part: &43 };
}
