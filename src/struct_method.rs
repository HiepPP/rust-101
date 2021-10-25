struct Player{
    name: String,
    iq: u8,
    friends: u8
}

impl Player{
    fn with_name(name: &str) -> Player{
        Player{
            name: name.to_string(),
            iq: 200,
            friends: 100
        }
    }

    fn get_friends(&self) -> u8{
        self.friends
    }

    fn set_friends(&mut self, count: u8){
        self.friends = count
    }
}

fn main(){
    let mut player = Player::with_name("HiepPP");
    player.set_friends(100);
    println!("{}'s friends count: {}", player.name, player.get_friends());
    let _ = Player::get_friends(&player);
}
