// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::*;

pub struct Paddle {
    pub x: u16,
    pub length: u16,
    speed: u16,
    CANVAS_SIZE: CanvasSize,
}

impl Paddle {
    pub fn new(canvas_size: CanvasSize) -> Self {
        Paddle {
            x: (0),
            length: (50),
            speed: (10),
            CANVAS_SIZE: canvas_size,
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
                if self.x < self.CANVAS_SIZE.WIDTH as u16 - self.length {
                    self.x += self.speed;
                }
                if self.x > self.CANVAS_SIZE.WIDTH as u16 - self.length {
                    self.x = self.CANVAS_SIZE.WIDTH as u16 - self.length;
                }
            }
        }
    }
}
