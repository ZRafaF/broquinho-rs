// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::*;

const MAX_PADDLE_SPEED: f32 = 400.0;
const PADDLE_ACCELERATION: f32 = 1300.0;

#[derive(Debug, Clone)]
pub struct Paddle {
    screen_pos: Position<f32>,
    pub length: f32,
    speed: f32,
    canvas_size: CanvasSize,
    paddle_height: f32,
}

impl Paddle {
    pub fn new(canvas_size: CanvasSize, paddle_length: f32) -> Self {
        Paddle {
            screen_pos: (Position {
                x: (canvas_size.width / 2.0),
                y: (canvas_size.height - 5.0),
            }),

            speed: (0.0),
            canvas_size: canvas_size,
            paddle_height: 10.0,
            length: paddle_length,
        }
    }

    pub fn get_screen_pos(&self) -> &Position<f32> {
        &self.screen_pos
    }

    pub fn get_paddle_height(&self) -> f32 {
        self.paddle_height
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
        let new_position = self.screen_pos.x + delta_position;
        if new_position < 0.0 {
            self.screen_pos.x = 0.0;
            self.speed *= -0.3;
            return;
        }
        if new_position > self.canvas_size.width - self.length {
            self.screen_pos.x = self.canvas_size.width - self.length;
            self.speed *= -0.3;
            return;
        }

        self.screen_pos.x = new_position
    }
}
