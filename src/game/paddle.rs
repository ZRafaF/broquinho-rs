// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::*;

const MAX_PADDLE_SPEED: f32 = 500.0;
const PADDLE_ACCELERATION: f32 = 900.0;

pub struct Paddle {
    pub x: f32,
    pub length: f32,
    speed: f32,
    canvas_size: CanvasSize,
}

impl Paddle {
    pub fn new(canvas_size: CanvasSize) -> Self {
        Paddle {
            x: (0.0),
            length: (50.0),
            speed: (0.0),
            canvas_size: canvas_size,
        }
    }

    pub fn move_paddle(&mut self, direction: MovementDirection, delta_time: &f32) {
        let new_speed = match direction {
            MovementDirection::Left => self.speed - (PADDLE_ACCELERATION * delta_time),
            MovementDirection::Right => self.speed + (PADDLE_ACCELERATION * delta_time),
        };
        if new_speed > MAX_PADDLE_SPEED {
            self.speed = MAX_PADDLE_SPEED;
        } else if new_speed < -MAX_PADDLE_SPEED {
            self.speed = -MAX_PADDLE_SPEED;
        }
        self.speed = new_speed;
    }

    pub fn process(&mut self, delta_time: &f32) {
        let delta_position = self.speed * delta_time;
        let new_position = self.x + delta_position;
        if new_position < 0.0 {
            self.x = 0.0;
            self.speed *= -0.8;
            return;
        }
        if new_position > self.canvas_size.WIDTH - self.length {
            self.x = self.canvas_size.WIDTH - self.length;
            self.speed *= -0.8;

            return;
        }

        self.x = new_position
    }
}
