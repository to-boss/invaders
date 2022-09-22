use crate::{
    frame::{Drawable, Frame},
    NUM_EXPLODE_FRAMES,
};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    explode: bool,
    explode_frames: usize,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Shot {
            x,
            y,
            explode: false,
            explode_frames: NUM_EXPLODE_FRAMES,
        }
    }

    pub fn update(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        } else {
            if self.explode_frames > 0 {
                self.explode_frames -= 1;
            } else {
                self.explode = true;
            }
        }
    }

    pub fn boom(&mut self) {
        self.explode = true;
    }

    pub fn exploded(&self) -> bool {
        self.explode
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.y == 0 { "*" } else { "|" };
    }
}
