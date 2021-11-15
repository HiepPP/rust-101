// With in a match expression, a move type is also moved by default.
#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food,
}

fn main() {
    let bag = Bag { food: Food::Salad };
    match bag.food {
        Food::Salad => println!("Salad"),
        a => println!("{:?}", a),
    }
    println!("{:?}", bag); //value borrowed here after partial move
}
