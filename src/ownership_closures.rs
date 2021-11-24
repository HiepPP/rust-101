#[derive(Debug)]
struct Foo;

fn main() {
    let a = Foo;
    // ownership of Foo is moved to b inside the closure, can't access a again
    let closure = || {
        let b = a;
    };
    println!("{:?}", a); //value borrowed here after move
}
