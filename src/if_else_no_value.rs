fn main() {
    let result = if 1 == 2 {
        "Nothing makes sense";
    } else {
        "Sanity reigns";
    };

    eprintln!("Result of computation: {:?}", result);
}
