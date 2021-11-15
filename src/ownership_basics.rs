// write!(f, "{:?}", x) uses Debug
// write!(f, "{}", x) uses Display

#[derive(Debug)]
struct Foo(u32);

fn main() {
    let foo = Foo(2048);
    let bar = foo;
    println!("Foo is {:?}", foo);
    println!("Bar is {:?}", bar);
}
