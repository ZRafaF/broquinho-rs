// Copyright (c) 2023 rafae
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
pub struct Position {
    x: u16,
    y: u16,
}

pub struct Broquinho {
    // Struct for the destroyable blocks
    pos: Position,
    life: u8,
}

pub enum MovementDirection {
    Right,
    Left,
}

pub struct Paddle {
    pub x: u16,
    pub length: u16,
    speed: u16,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            x: (0),
            length: (50),
            speed: (10),
        }
    }

    pub fn move_paddle(&mut self, direction: MovementDirection) {
        match direction {
            MovementDirection::Left => {
                if self.x > self.speed {
                    self.x -= self.speed;
                } else {
                    self.x = 0;
                }
            }
            MovementDirection::Right => {
                if self.x < 1000 {
                    self.x += self.speed;
                }
            }
        }
    }
}

pub struct Game {
    pub broquinho_vec: Vec<Broquinho>,
    pub paddle: Paddle,
}

impl Game {
    pub fn new() -> Self {
        Game {
            paddle: Paddle::new(),
            broquinho_vec: vec![],
        }
    }

    pub fn hit(&self, broquinho_ref: &mut Broquinho, damage: u8) {
        if damage > broquinho_ref.life {
            broquinho_ref.life -= damage;
        } else {
        }
    }

    pub fn move_right(&mut self) {
        self.paddle.move_paddle(MovementDirection::Right);
    }

    pub fn move_left(&mut self) {
        self.paddle.move_paddle(MovementDirection::Left);
    }

    fn kill_broquinho(&self, broquinho_ref: &mut Broquinho) {
        todo!()
    }
}
