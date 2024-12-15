use crate::snake::Snake;

pub struct Board {
    max_x: u16,
    max_y: u16,
    snakes: Vec<Snake>,
}

impl Board {
    pub fn new(max_x: u16, max_y: u16, snakes_count: u8) -> Self {
        Board {
            max_x,
            max_y,
            snakes: (0..snakes_count).map(|_| Snake::new()).collect(),
        }
    }

    pub fn get_snakes(&self) -> Vec<Snake> {
        self.snakes.clone()
    }
}
