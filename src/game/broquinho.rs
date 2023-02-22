// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::*;

#[derive(Hash, Clone)]
pub struct Broquinho {
    // Struct for the destroyable blocks
    pos: Position,
    life: u8,
}

impl Broquinho {
    pub fn new(position: Position, starting_life: u8) -> Self {
        Broquinho {
            pos: (position.clone()),
            life: (starting_life),
        }
    }

    pub fn get_pos_x(&self) -> f32 {
        self.pos.x as f32
    }
    pub fn get_pos_y(&self) -> f32 {
        self.pos.y as f32
    }
    pub fn get_life(&self) -> u8 {
        self.life
    }
    pub fn set_life(&mut self, life: u8) {
        self.life = life;
    }

    pub fn damage(&mut self, damage: u8) {
        if damage > self.get_life() {
            self.set_life(self.get_life() - damage);
        } else {
        }
    }
}
