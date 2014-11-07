extern crate gtb;

use gtb::game::Game;
use gtb::game::GameState;
use gtb::game::Entity;

fn main() {
    let mut state1 = GameState::new(Entity::from_json("data/menu2.json"));
    println!("menu name: {}", state1["play".to_string()].name);
    state1["play".to_string()].click(|pos: gtb::core::Position<int>| {
        println!("clicky {}, {}", pos.x, pos.y);
    });
    let mut game = Game::new(state1);
    //game.set_state(state1);
    game.run();
}
