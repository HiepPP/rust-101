fn take_the_n(n: u8) {}

fn take_the_s(s: String) {}

fn main() {
    let n = 5; // primitive type implement Copy by default
    let s = String::from("hello"); // s move because String does not implement Copy

    take_the_n(n);
    // take_the_s(s);
    take_the_s(s.clone()); // Fix: String implement Clone

    println!("n is {}", n);
    println!("s is {}", s);
}
