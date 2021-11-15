#[derive(Clone, Debug)]
struct Dummy {
    items: u32,
}

fn main() {
    let a = Dummy { items: 1 };
    let b = a.clone();
    println!("a: {:?}, a : {:?}", a, b);
}
