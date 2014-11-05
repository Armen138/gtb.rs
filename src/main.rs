extern crate gtb;
extern crate serialize;

use gtb::game::Game;
use gtb::game::GameState;
use gtb::game::Entity;
use std::io::File;
use serialize::{json,Decodable,Encodable};

#[deriving(Decodable, Encodable)]
struct EntityData {
    id: String,
    X: f64,
    Y: f64,
    image: String
}

#[deriving(Decodable, Encodable)]
struct StateData {
    id: String,
    entities: Vec<EntityData> 
}

fn callback() {
    println!("callback called");
}

//fn load_data(data:&str) -> Entity {
    //let path = &Path::new(data);
    //let display = path.display();
    //let mut file = match File::open(path) {
        //Err(why) => fail!("couldn't open {}: {}", display, why.desc),
        //Ok(file) => file,
    //};
    //let contents = match file.read_to_string() {
        //Err(why) => fail!("failed to read file"),
        //Ok(contents) => contents,
    //};
    
    //let decoded: StateData = json::decode(contents.as_slice()).unwrap();
    //println!("data: {}", decoded.entities[0].id);
    //let mut root = Entity::new("rootnode".to_string(), 0.0, 0.0);
    //for entity in decoded.entities.iter() {
        ////let child = Entity::new(entity.id.as_slice(), entity.X, entity.Y);
        //let child = Entity::from_json(entity);
        //root.push(child);
    //}
    //root
//}

fn main() {
    //let menu1 = load_data("data/menu.json");
    //let state1 = GameState::new(menu1);
    //let position = gtb::core::Position::new(0.0, 0.0);
    //let main_menu = Entity::new("mainmenu", 0.0, 0.0);
    //let options_menu = Entity::new("optionsmenu", 0.0, 0.0);
    //let state2 = GameState::new(main_menu);
    //options_menu.on(Click, callback);
    //state2.find("play").click(|x: int, y: initial_state| {

    //});as_string
    //let name = state2["main".to_string()].name;
    //println!("menu name: {}", name);
    let mut state1 = GameState::new(Entity::from_json("data/menu2.json"));
    println!("menu name: {}", state1["play".to_string()].name);
    state1["play".to_string()].click(|pos: gtb::core::Position| {
        println!("clicky {}, {}", pos.x, pos.y);
    });
    let mut game = Game::new(state1);
    //game.set_state(state1);
    game.run();
}
