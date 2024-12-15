#[derive(Clone, Debug)]
pub struct Location {
    x: u16,
    y: u16,
}

impl Location {
    pub fn new(x: u16, y: u16) -> Self {
        Location { x, y }
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
