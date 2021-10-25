use std::collections::HashMap;

fn main() {
    let mut fruids = HashMap::new();
    fruids.insert("apple", 3);
    fruids.insert("mango", 6);
    fruids.insert("orange", 2);
    fruids.insert("avocado", 7);
    for (k, v) in &fruids {
        println!("I got {} {}", v, k);
    }

    fruids.remove("orange");
    let old_avocado = fruids["avocado"];
    fruids.insert("avocado", old_avocado);
    println!("\nI now have {} avocados", fruids["avocado"]);
}
