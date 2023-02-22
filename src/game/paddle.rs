// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::*;

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
            speed: (500.0),
            canvas_size: canvas_size,
        }
    }

    pub fn move_paddle(&mut self, direction: MovementDirection, delta_time: &f32) {
        let delta_position = match direction {
            MovementDirection::Left => -self.speed * delta_time,
            MovementDirection::Right => self.speed * delta_time,
        };
        let new_position = self.x + delta_position;
        if new_position < 0.0 {
            self.x = 0.0;
            return;
        }
        if new_position > self.canvas_size.WIDTH - self.length {
            self.x = self.canvas_size.WIDTH - self.length;
            return;
        }

        self.x = new_position;
    }
}
