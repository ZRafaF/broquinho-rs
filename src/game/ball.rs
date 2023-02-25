// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::Position;
use macroquad::prelude::scene::Handle;

pub struct Ball {
    screen_pos: Position<f32>,
    velocity: Position<f32>,
    radius: f32,
    broquinho_size: f32,
    pos: Position<u16>,
}

impl Ball {
    pub fn new(
        screen_pos: Position<f32>,
        velocity: Position<f32>,
        radius: f32,
        broquinho_size: f32,
    ) -> Self {
        Ball {
            screen_pos: (screen_pos),
            velocity: (velocity),
            radius: (radius),
            broquinho_size: (broquinho_size),
            pos: (helper::screen_pos_to_pos(screen_pos, broquinho_size)),
        }
    }

    pub fn get_screen_pos(&self) -> &Position<f32> {
        &self.screen_pos
    }
    pub fn set_screen_pos(&mut self, new_pos: Position<f32>) {
        self.screen_pos = new_pos;
    }
    pub fn get_pos(&self) -> &Position<u16> {
        &self.pos
    }
    pub fn get_velocity(&self) -> &Position<f32> {
        &self.velocity
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn process(&mut self, delta_time: &f32) {
        let delta_pos = Position {
            x: { self.velocity.x * delta_time },
            y: { self.velocity.y * delta_time },
        };
        let new_pos = delta_pos + self.screen_pos;
        self.screen_pos = new_pos;
        self.pos = helper::screen_pos_to_pos(self.screen_pos, self.broquinho_size);
    }
}
