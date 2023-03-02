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
    pos: Position<u16>,
    screen_pos: Position<f32>,
    life: i16,
}

impl Broquinho {
    pub fn new(
        position: Position<u16>,
        screen_position: Position<f32>,
        starting_life: i16,
    ) -> Self {
        Broquinho {
            pos: (position),
            screen_pos: (screen_position.clone()),
            life: (starting_life),
        }
    }

    pub fn get_pos(&self) -> &Position<u16> {
        &self.pos
    }

    pub fn get_screen_pos(&self) -> &Position<f32> {
        &self.screen_pos
    }

    pub fn get_life(&self) -> i16 {
        self.life
    }
    pub fn set_life(&mut self, life: i16) {
        self.life = life;
    }

    /*
    pub fn damage(&mut self) {
        if damage < self.get_life() {
            self.set_life(self.get_life() - damage);
        } else {
        }
    }
    */
}
