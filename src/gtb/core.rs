use serialize::{Encodable};
use std::num::Num;

pub enum MouseButton {
    Left,
    Right,
    Middle
}

#[deriving(Decodable, Encodable)]
pub struct Rectangle<T:Num> {    
    pub position: Position<T>,
    pub size: Size<T>
}

#[deriving(Decodable, Encodable)]
pub struct Size<T:Num> {
    pub width: T,
    pub height: T
}

#[deriving(Decodable, Encodable)]
pub struct Position<T:Num> {
    pub x: T,
    pub y: T
}

impl<T:Num> Position<T> {
    pub fn new(x: T, y: T) -> Position<T> {
        Position { x: x, y: y }
    }

    pub fn distance(&self, other: Position<T>) -> (T) {
        let distance = (self.x - other.x) * (self.x - other.x) +
                        (self.y - other.y) * (self.y - other.y); //.sqrt();
        distance
    }

    pub fn set(& mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    //pub fn get(&self) -> (T, T) {
        //let x = self.x;
        //let y = self.y;
        //(x, y)
    //}
}

impl<T:Num> Add<Position<T>, Position<T> > for Position<T> {
    fn add(&self, _rhs: &Position<T>) -> Position<T> {
        Position { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl<T:Num> Sub<Position<T>, Position<T> > for Position<T> {
    fn sub(&self, _rhs: &Position<T>) -> Position<T> {
        Position { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}
