use serialize::{json,Decodable,Decoder,Encodable,Encoder};
use std::io::File;

pub struct Game {
    state: GameState
}

impl Game {
    pub fn new(initial_state: GameState) -> Game {
        Game { state: initial_state } 
    }
    pub fn run(& mut self) {
        loop {
            self.state.root.draw();
            break;
        }
    }
    pub fn set_state(& mut self, new_state: GameState) {
        println!("setting new game state");
        self.state = new_state;
    }
}

pub struct GameState {
    root: Entity
}

impl GameState {
    pub fn new(root: Entity) -> GameState {
        GameState { root: root }
    }
}

pub struct EntityEvents {
    pub click: Option<|position: ::core::Position|:'static>
}

impl<S: Decoder<E>, E> Decodable<S, E> for EntityEvents {
    fn decode(decoder: &mut S) -> Result<EntityEvents, E> {
        Ok(EntityEvents{click: None})
    }
}

impl<S: Encoder<E>, E> Encodable<S, E> for EntityEvents {
    fn encode(&self, decoder: &mut S) -> Result<(), E> {
        Ok(())
    }
}

#[deriving(Decodable, Encodable)]
pub struct Entity {
    pub position: ::core::Position,
    pub name: String,
    entities: Vec<Entity>,
    events: Option<EntityEvents>
}



impl Entity {
    pub fn from_json(file_name: &'static str) -> Entity {
        let path = &Path::new(file_name);
        let display = path.display();
        let mut file = match File::open(path) {
            Err(why) => fail!("couldn't open {}: {}", display, why.desc),
            Ok(file) => file,
        };
        let contents = match file.read_to_string() {
            Err(why) => fail!("failed to read file"),
            Ok(contents) => contents,
        };
        
        let decoded: Entity = json::decode(contents.as_slice()).unwrap();
        println!("data: {}", decoded.entities[0].name);
        decoded
    }
    pub fn new(name: String, x: f64, y: f64) -> Entity {
        Entity {
            position: ::core::Position{x: x, y: y},
            name: name, //"generic",
            entities: Vec::new(),
            events: Some(EntityEvents{click:None})
        }
    }
    pub fn push(& mut self, other: Entity) {
        self.entities.push(other);
    }

    pub fn draw(& mut self) {
        // draw stuff
        println!("drawing entity");
        let p = ::core::Position::new(1.1, 1.4);
        self.clickEvent(p);
        for ent in self.entities.iter_mut() {
            ent.draw();
        }
    }

    fn clickEvent(& mut self, pos: ::core::Position) {
        match(self.events) {
            Some(ref mut events) => {
                match(events.click) {
                    Some(ref mut click) => (*click)(pos),
                    None => {
                        println!("no click event listener");
                        ()
                    }
                }
            },
            None => {
                println!("no event listener object");   
                ()
            }
        };
    }
    pub fn click(& mut self, cb: |pos: ::core::Position|:'static) {
        let p = ::core::Position::new(1.1, 1.2);
        match(self.events) {
            Some(ref mut events) => {
                println!("assign click listener");
                events.click = Some(cb)
            },
            None => {
                println!("no place to assign a listener");
                self.events = Some(EntityEvents{click: Some(cb)});
                ()
            }
        };
    }
}

impl Index<String, Entity> for GameState {
    fn index<'a>(&'a self, rhs: &String) -> &'a Entity {
        for ent in self.root.entities.iter() {               
            if(&ent.name == rhs) {
                return ent;
            }
        }
        unreachable!()
    }
}

impl IndexMut<String, Entity> for GameState {
    fn index_mut<'a>(&'a mut self, rhs: &String) -> &'a mut Entity {
        for ent in self.root.entities.iter_mut() {               
            if(&ent.name == rhs) {
                return ent;
            }
        }
        unreachable!()
    }
}

impl IndexMut<String, Entity> for Entity {
    fn index_mut<'a>(&'a mut self, rhs: &String) -> &'a mut Entity {
        for ent in self.entities.iter_mut() {               
            if(&ent.name == rhs) {
                return ent;
            }
        }
        unreachable!()
    }
}

impl Index<String, Entity> for Entity {
    fn index<'a>(&'a self, rhs: &String) -> &'a Entity {
        for ent in self.entities.iter() {               
            if(&ent.name == rhs) {
                return ent;
            }
        }
        unreachable!()
    }
}
