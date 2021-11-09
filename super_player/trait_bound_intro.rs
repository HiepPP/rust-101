struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero is loading");
    }
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy is loading");
    }
}

impl Game {
    // This is a trait bound method, allow us to constrain the range of parameters that a generic API can accept.
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
