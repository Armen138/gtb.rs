extern crate gtb;

use gtb::game::Game;
use gtb::game::GameState;
use gtb::game::Entity;

fn main() {
    let mut state1 = GameState::new(Entity::from_json("data/menu2.json"));
    let mut state2 = GameState::new(Entity::from_json("data/menu2.json"));
    let mut game = Game::new(state2);
    println!("menu name: {}", state1["play".to_string()].name);
    state1["play".to_string()].click(|pos: gtb::core::Position<int>| {
        println!("clicky {}, {}", pos.x, pos.y);
    });
    state1.root.keyup(|code: u8| {
        if code == 53 {
            println!("looks like escape");
            game.close = true;
        }
        println!("keyup: {}", code);
    });
    //let mut game = Game::new(state1);
    game.set_state(state1);
    game.run();
}
