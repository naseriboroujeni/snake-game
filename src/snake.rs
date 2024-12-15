use rand::Rng;

use crate::location::{Direction, Location};

#[derive(Clone)]
pub struct Snake {
    body: Vec<Location>,
}

impl Snake {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen();
        let random_y = rng.gen();
        let initial_location = Location::new(random_x, random_y);
        Snake {
            body: vec![initial_location],
        }
    }

    pub fn get_head(&self) -> Location {
        self.body.get(0).cloned().unwrap()
    }

    pub fn move_snake(&mut self, max_x: u16, max_y: u16, direction: Direction) {
        let old_head = self.body.get(0).unwrap();
        let new_head = match direction {
            Direction::Up => Location::new(old_head.get_x(), (old_head.get_y() + 1) % max_y),
            Direction::Right => Location::new((old_head.get_x() + 1) % max_x, old_head.get_y()),
            Direction::Down => Location::new(
                old_head.get_x(),
                if old_head.get_y() == 0 {
                    max_y - 1
                } else {
                    old_head.get_y() - 1
                },
            ),
            Direction::Left => Location::new(
                if old_head.get_x() == 0 {
                    max_x - 1
                } else {
                    old_head.get_x() - 1
                },
                old_head.get_y(),
            ),
        };

        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1].clone();
        }

        self.body[0] = new_head;
    }
}
