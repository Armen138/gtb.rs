use serialize::{json,Decodable,Encodable};

#[deriving(Decodable, Encodable)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x: x, y: y }
    }

    pub fn distance(&self, other: Position) -> (f64) {
        let distance = ((self.x - other.x) * (self.x - other.x) +
                        (self.y - other.y) * (self.y - other.y)).sqrt();
        distance
    }

    pub fn set(& mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Add<Position, Position> for Position {
    fn add(&self, _rhs: &Position) -> Position {
        Position { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl Sub<Position, Position> for Position {
    fn sub(&self, _rhs: &Position) -> Position {
        Position { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}
