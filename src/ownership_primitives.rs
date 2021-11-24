// Primitive get copied instead of moved
fn main() {
    let foo = 1234;
    let bar = foo; // does not move but copy
    println!("{:?} {:?}", foo, bar);
}
