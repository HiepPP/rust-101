fn give_me<T>(value: T) {
    let _ = value;
}

fn main() {
    give_me(1);
    give_me(1.0);
    give_me(true);
       give_me("hello");
}