#[derive(Copy, Debug)]
struct Dummy;

fn main() {
    let a = Dummy;
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);
}
