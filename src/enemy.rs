use crate::{
    frame::{Drawable, Frame},
    NUM_COLS,
};

const BUFFER_SIZE: usize = 50;

pub struct Enemy {
    x: usize,
    y: usize,
    direction: Direction,
    explode: bool,
    buffer_frames: usize,
}

impl Enemy {
    pub fn new(x: usize, y: usize) -> Self {
        Enemy {
            x,
            y,
            direction: Direction::Left,
            explode: false,
            buffer_frames: BUFFER_SIZE,
        }
    }

    pub fn update(&mut self) {
        // consume buffer frames before moving the enemy
        if self.buffer_frames > 0 {
            self.buffer_frames -= 1;
            return;
        }

        if self.buffer_frames == 0 {
            self.buffer_frames = BUFFER_SIZE;
        }

        // triggers when you hit a wall
        if self.x == 0 || self.x == NUM_COLS - 1 {
            self.direction = match self.direction {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            };
            self.y += 1;
        }

        // move left or right depending on the direction of the enemy
        match self.direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn exploded(&self) -> bool {
        self.explode
    }
}

pub enum Direction {
    Left,
    Right,
}

impl Drawable for Enemy {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.explode { "*" } else { "@" };
    }
}
