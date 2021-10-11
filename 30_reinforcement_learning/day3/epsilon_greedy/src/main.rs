mod cointoss;
use cointoss::{Application, CoinToss};
mod agent;

fn main() {
    println!("Hello, world!");
    
    let mut game: CoinToss = Application::init(vec![0.1, 0.8, 0.3], 30, 27);

    game.action(0);
    game.action(1);
    game.action(2);
    println!("Get toss count : {}", game.get_count());
    game.reset();
    game.show_parameter();
}
