// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::Position;

pub struct Ball {
    screen_pos: Position<f32>,
    velocity: Position<f32>,
    radius: f32,
}

impl Ball {
    pub fn new(screen_pos: Position<f32>, velocity: Position<f32>, radius: f32) -> Self {
        Ball {
            screen_pos: (screen_pos),
            velocity: (velocity),
            radius: (radius),
        }
    }

    pub fn get_screen_pos(&self) -> &Position<f32> {
        &self.screen_pos
    }
    pub fn get_velocity(&self) -> &Position<f32> {
        &self.velocity
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}
