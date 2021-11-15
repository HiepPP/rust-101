// within an impl block, any method with self as the first parameter takes ownership of the value on which the method is called.
// This means that after call the method on the value, cannot use that value again.
struct Item(u32);

impl Item {
    fn new() -> Item {
        Item(1993)
    }

    fn take_item(self) { //after call this method, ownership moved
                         //does nothing
    }
}

fn main() {
    let it = Item::new();
    it.take_item();
    println!("{}", it.0); // value borrowed here after move
}
