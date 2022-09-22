use crate::{
    enemy::Enemies,
    frame::{Drawable, Frame},
    shot::Shot,
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

    pub fn handle_shot_collisions(&mut self, enemies: &mut Enemies) {
        for shot in self.shots.iter_mut() {
            let mut hit_enemies = enemies
                .army
                .iter_mut()
                .filter(|enemy| enemy.x == shot.x && enemy.y == shot.y)
                .collect::<Vec<_>>();

            if hit_enemies.len() > 0 {
                shot.boom();
                hit_enemies.iter_mut().for_each(|enemy| enemy.boom());
            }
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
