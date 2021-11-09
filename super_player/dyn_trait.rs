use std::fmt::Display;

fn show_me(item: &dyn Display) {
    println!("{}", item);
}

fn main() {
    let a = "hello";
    let b = "world";
    show_me(&a);
    show_me(&b);
}
