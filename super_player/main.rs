struct Audio(String);
struct Video(String);

// two kind of method within trait
// 1. associated method: not need instance, knowns as static method
// 2. instance type: first parameter is self, need instance, knowns as instance method

trait Playable {
    fn play(&self);
    fn haha(&self) {
        println!("haha");
    }
    // do not take self, akin to static method that does not require an instance of the implementation to invoke it.
    fn pause() {
        println!("Pause");
    }
}

impl Playable for Audio {
    fn play(&self) {
        println!("Playing {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Playing {}", self.0);
    }
}

fn main() {
    println!("Audio: ");
}
