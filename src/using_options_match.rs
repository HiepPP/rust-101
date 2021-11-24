use sdt::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let incremented_value = match map.get("one") {
        Some(value) => value + 1,
        None => 0,
    };

    println!("{}", incremented_value);
}
