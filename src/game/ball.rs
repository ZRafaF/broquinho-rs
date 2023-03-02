// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use helper::{CollisionDirection, Position};
use macroquad::rand::RandomRange;

#[derive(Debug, Clone)]
pub struct Ball {
    screen_pos: Position<f32>,
    velocity: Position<f32>,
    radius: f32,
    broquinho_size: f32,
    pos: Position<u16>,
    //rng: ThreadRng,
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
            //rng: rand::thread_rng(),
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
    #[allow(dead_code)]
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

    pub fn ricochet(&mut self, collision_direction: &CollisionDirection) {
        let random_noise: f32 = RandomRange::gen_range(-5.0, 5.0);
        match collision_direction {
            CollisionDirection::Left => {
                self.velocity.x = self.velocity.x.abs();
                self.velocity.y += random_noise;
            }
            CollisionDirection::Right => {
                self.velocity.x = -self.velocity.x.abs();
                self.velocity.y += random_noise;
            }
            CollisionDirection::Down => {
                self.velocity.x += random_noise;
                self.velocity.y = -self.velocity.y.abs();
            }
            CollisionDirection::Top => {
                self.velocity.x += random_noise;
                self.velocity.y = self.velocity.y.abs();
            }
        }
    }
}
