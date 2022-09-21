use crate::frame::{Drawable, Frame};

pub struct Shot {
    x: usize,
    y: usize,
    explode: bool,
    buffer_frames: usize,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Shot {
            x,
            y,
            explode: false,
            buffer_frames: 5,
        }
    }

    pub fn update(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        } else {
            if self.buffer_frames > 0 {
                self.buffer_frames -= 1;
            } else {
                self.explode = true;
            }
        }
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
