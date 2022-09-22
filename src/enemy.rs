use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_EXPLODE_FRAMES, NUM_FRAMES_BETWEEN_ENEMY_MOVING, NUM_ROWS,
};

pub struct Enemies {
    pub army: Vec<Enemy>,
    direction: Direction,
    just_bounced: bool,
}

impl Enemies {
    pub fn new() -> Self {
        let mut army = Vec::new();
        let direction = Direction::Right;
        let just_bounced = false;

        for x in 1..NUM_COLS - 1 {
            for y in 0..((NUM_ROWS - 1) / 2) {
                if x % 2 == 0 && y % 2 == 0 {
                    army.push(Enemy::new(x, y));
                }
            }
        }

        Enemies {
            army,
            direction,
            just_bounced,
        }
    }

    pub fn update(&mut self) {
        let mut wall_bounce: bool = self
            .army
            .iter_mut()
            .any(|enemy| enemy.x == 0 || enemy.x == NUM_COLS - 1);

        // We want to move downwards first when we bounce, so we need to check if we just bounced
        // to make sure we get an extra update to move down before we move into the real direction
        if wall_bounce && self.just_bounced {
            wall_bounce = false;
            self.just_bounced = false;
        }

        if wall_bounce {
            self.direction.switch();
            self.just_bounced = true;
        }

        for enemy in self.army.iter_mut() {
            enemy.update(&self.direction, wall_bounce);
        }

        self.army.retain(|enemy| enemy.explode_frames > 0);
    }
}

pub struct Enemy {
    pub x: usize,
    pub y: usize,
    explode: bool,
    buffer_frames: usize,
    explode_frames: usize,
}

impl Enemy {
    pub fn new(x: usize, y: usize) -> Self {
        Enemy {
            x,
            y,
            explode: false,
            buffer_frames: NUM_FRAMES_BETWEEN_ENEMY_MOVING,
            explode_frames: NUM_EXPLODE_FRAMES,
        }
    }

    pub fn update(&mut self, direction: &Direction, wall_bounce: bool) {
        if self.explode {
            if self.explode_frames > 0 {
                self.explode_frames -= 1;
                return;
            }
        }
        // consume buffer frames before moving the enemy
        if self.buffer_frames > 0 {
            self.buffer_frames -= 1;
            return;
        }

        if self.buffer_frames == 0 {
            self.buffer_frames = NUM_FRAMES_BETWEEN_ENEMY_MOVING;
        }

        if wall_bounce {
            self.y += 1;
            return;
        }

        // move left or right depending on the direction of the enemy
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn boom(&mut self) {
        self.explode = true;
    }

    pub fn exploded(&self) -> bool {
        self.explode
    }
}

impl Drawable for Enemies {
    fn draw(&self, frame: &mut Frame) {
        for enemy in self.army.iter() {
            frame[enemy.x][enemy.y] = if enemy.explode { "*" } else { "@" };
        }
    }
}

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn switch(&mut self) {
        *self = match *self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }
}
