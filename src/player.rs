use crate::{
    frame::{Drawable, Frame},
    shot::{self, Shot},
    NUM_COLS, NUM_ROWS,
};

pub const MAGAZINE_SIZE: usize = 2;

pub struct Player {
    x: usize,
    y: usize,
    pub shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::with_capacity(MAGAZINE_SIZE),
        }
    }

    pub fn shoot(&mut self) {
        if self.shots.len() < MAGAZINE_SIZE {
            self.shots.push(Shot::new(self.x, self.y - 1));
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}
