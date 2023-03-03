// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::*;

// Struct for the destroyable blocks

/*
    (0,0)
    *----------------
    |               |
    |               |
    |               |
    |               |
    |               |
    -----------------
*/
#[derive(Debug, Clone)]
pub struct Broquinho {
    screen_pos: Position<f32>,
    life: f32,
}

impl Broquinho {
    pub fn new(screen_position: Position<f32>, starting_life: f32) -> Self {
        Broquinho {
            screen_pos: (screen_position.clone()),
            life: (starting_life),
        }
    }

    pub fn get_screen_pos(&self) -> &Position<f32> {
        &self.screen_pos
    }

    pub fn get_life(&self) -> f32 {
        self.life
    }
    pub fn set_life(&mut self, life: f32) {
        self.life = life;
    }
}
