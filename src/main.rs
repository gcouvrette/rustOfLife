mod game;
use game::Game as Game;

fn main() {
    let game = Game::new();
    println!("{:?}", game.get_state());
}
