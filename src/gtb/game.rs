use serialize::{json,Decodable,Decoder,Encodable,Encoder};
use std::io::File;

pub struct Game {
    state: GameState,
    window: ::glutin::Window,
    mouse_position: ::core::Position<int>,
    close: bool
}

impl Game {
    fn handle_event(& mut self, event: ::glutin::Event) {
        match event {
            ::glutin::Event::MouseMoved(pos) => {
                //(self.mouse_position.x, self.mouse_position.y) = pos;
                self.mouse_position.set(pos.val0(), pos.val1());
                //self.mouse_position.x = pos.val0();
                //self.mouse_position.y = pos.val1();
                //println!("mouse move: {}, {}", x, y);
                self.state.root.dispatch_mousemove(self.mouse_position);
            }
            ::glutin::Event::MouseInput(state, button) => {
                
                match state {
                    ::glutin::ElementState::Pressed => {                        
                        self.state.root.dispatch_click(::core::MouseButton::Left, self.mouse_position);
                        self.state.root.dispatch_mousedown(::core::MouseButton::Left, self.mouse_position);
                    }
                    ::glutin::ElementState::Released => {
                        self.state.root.dispatch_mouseup(::core::MouseButton::Left, self.mouse_position);
                    }
                }
            }
            ::glutin::Event::KeyboardInput(state, keycode, virtualkey) => {
                match state {
                    ::glutin::ElementState::Pressed => {
                        self.state.root.dispatch_keydown(keycode);
                    }
                    ::glutin::ElementState::Released => {
                        self.state.root.dispatch_keyup(keycode);
                    }
                }
            }
            _ => {
                println!("unimplemented glutin event.");
            }
        }
    }
    pub fn new(initial_state: GameState) -> Game {
        Game { 
            state: initial_state,
            window: ::glutin::Window::new().unwrap(),
            mouse_position: ::core::Position::new(0, 0),
            close: false
        } 
    }
    pub fn run(& mut self) {
        //let window = ::glutin::Window::new().unwrap();
        unsafe { self.window.make_current() };
        ::gl::load_with(|symbol| self.window.get_proc_address(symbol));
        unsafe { ::gl::ClearColor(0.0, 1.0, 0.0, 1.0) };

        //while !self.window.is_closed() {
        while !self.close {
            let mut events = self.window.poll_events();
            for ev in events {
                self.handle_event(ev);
                //match(ev) {
                    //::glutin::Event::MouseMoved(pos) => {
                        //let (x, y) = pos;
                        //println!("mouse move: {}, {}", x, y);
                    //}
                    //_ => {
                        //println!("unimplemented glutin event.");
                    //}
                //}
            }
            unsafe { ::gl::Clear(::gl::COLOR_BUFFER_BIT) };
            self.state.root.draw();
            self.window.swap_buffers();
        }
        //drop(self.window);
        //
        //loop {
            //self.state.root.draw();
            //break;
        //}
    }
    pub fn set_state(& mut self, new_state: GameState) {
        println!("setting new game state");
        self.state = new_state;
    }
}

pub struct GameState {
    pub root: Entity
}

impl GameState {
    pub fn new(root: Entity) -> GameState {
        GameState { root: root }
    }
}

pub struct EntityEvents {
    pub click: Option<|position: ::core::Position<int>|:'static>,
    pub keyup: Option<|code: u8|:'static>
}

impl<S: Decoder<E>, E> Decodable<S, E> for EntityEvents {
    fn decode(decoder: &mut S) -> Result<EntityEvents, E> {
        Ok(EntityEvents{
            click: None,
            keyup: None
        })
    }
}

impl<S: Encoder<E>, E> Encodable<S, E> for EntityEvents {
    fn encode(&self, decoder: &mut S) -> Result<(), E> {
        Ok(())
    }
}

#[deriving(Decodable, Encodable)]
pub struct Entity {
    pub position: ::core::Position<int>,
    pub name: String,
    size: Option<::core::Size<int>>,
    children: Vec<Entity>,
    events: Option<EntityEvents>
}



impl Entity {
    pub fn from_json(file_name: &'static str) -> Entity {
        let path = &Path::new(file_name);
        let display = path.display();
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.desc),
            Ok(file) => file,
        };
        let contents = match file.read_to_string() {
            Err(why) => panic!("failed to read file, {}", why.desc),
            Ok(contents) => contents,
        };
        
        let decoded: Entity = match json::decode(contents.as_slice()) {
            Err(why) => panic!("panic! error decoding json, {}", why),
            Ok(decoded) => decoded,
        };
        decoded
    }
    pub fn new(name: String, x: int, y: int) -> Entity {
        Entity {
            position: ::core::Position{x: x, y: y},
            size: None,
            name: name, //"generic",
            children: Vec::new(),
            events: Some(EntityEvents{
                click:None,
                keyup:None
            })
        }
    }
    pub fn push(& mut self, other: Entity) {
        //let mut children = self.children;
        self.children.push(other);
    }

    pub fn draw(& mut self) {
        // draw stuff
        //println!("drawing entity");
        //let p = ::core::Position::new(1, 4);
        //self.dispatch_click(::core::MouseButton::Left, p);
        //let mut children = self.children;
        for ent in self.children.iter_mut() {
            ent.draw();
        }
    }

    pub fn dispatch_keyup(& mut self, code: u8) {
        match self.events {
            Some(ref mut events) => {
                match events.keyup {
                    Some(ref mut keyup) => (*keyup)(code),
                    None => {
                        //println!("no click event listener");
                        ()
                    }
                }
            },
            None => {
                //println!("no event listener object");   
                ()
            }
        };

    }

    pub fn dispatch_keydown(& mut self, code: u8) {

    }

    pub fn dispatch_mousemove(& mut self, pos: ::core::Position<int>) {

    }

    pub fn dispatch_mouseup(& mut self, 
                            button: ::core::MouseButton, 
                            pos: ::core::Position<int>) {

    }

    pub fn dispatch_mousedown(& mut self, 
                              button: ::core::MouseButton, 
                              pos: ::core::Position<int>) {

    }

    pub fn dispatch_click(& mut self, 
                      button: ::core::MouseButton, 
                      pos: ::core::Position<int>) {
        match self.events {
            Some(ref mut events) => {
                match events.click {
                    Some(ref mut click) => (*click)(pos),
                    None => {
                        //println!("no click event listener");
                        ()
                    }
                }
            },
            None => {
                //println!("no event listener object");   
                ()
            }
        };
    }
    pub fn keyup(& mut self, cb: |code: u8|:'static) {
        match self.events {
            Some(ref mut events) => {
                println!("assign keyup listener");
                events.keyup = Some(cb)
            },
            None => {
                println!("no place to assign a listener");
                self.events = Some(EntityEvents{
                    keyup : Some(cb),
                    click: None
                });
                ()
            }
        };

    }

    pub fn click(& mut self, cb: |pos: ::core::Position<int>|:'static) {
        let p = ::core::Position::new(1i, 2i);
        match self.events {
            Some(ref mut events) => {
                println!("assign click listener");
                events.click = Some(cb)
            },
            None => {
                println!("no place to assign a listener");
                self.events = Some(EntityEvents{
                    click: Some(cb),
                    keyup: None
                });
                ()
            }
        };
    }
}

impl Index<String, Entity> for GameState {
    fn index<'a>(&'a self, rhs: &String) -> &'a Entity {
        //let children = self.root.children;
        for ent in self.root.children.iter() {               
            if &ent.name == rhs {
                return ent;
            }
        }
        unreachable!()
    }
}

impl IndexMut<String, Entity> for GameState {
    fn index_mut<'a>(&'a mut self, rhs: &String) -> &'a mut Entity {
        //let mut children = self.root.children;
        for ent in self.root.children.iter_mut() {               
            if &ent.name == rhs {
                return ent;
            }
        }
        unreachable!()
    }
}

impl IndexMut<String, Entity> for Entity {
    fn index_mut<'a>(&'a mut self, rhs: &String) -> &'a mut Entity {
        //let mut children = self.children;
        for ent in self.children.iter_mut() {               
            if &ent.name == rhs {
                return ent;
            }
        }
        unreachable!()
    }
}

impl Index<String, Entity> for Entity {
    fn index<'a>(&'a self, rhs: &String) -> &'a Entity {
        //let mut children = self.children;
        for ent in self.children.iter() {               
            if &ent.name == rhs {
                return ent;
            }
        }
        unreachable!()
    }
}
