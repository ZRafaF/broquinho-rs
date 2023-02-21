// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
mod broquinho;
use broquinho::Broquinho;

mod paddle;
use paddle::Paddle;

mod ball;
use ball::Ball;

use helper::*;

pub struct Game {
    pub broquinho_vec: Vec<Broquinho>,
    pub paddle: Paddle,
    CANVAS_SIZE: CanvasSize,
}

impl Game {
    pub fn new(canvas_size: CanvasSize) -> Self {
        Game {
            paddle: Paddle::new(canvas_size.clone()),
            broquinho_vec: vec![
                Broquinho::new(helper::Position { x: (1), y: (1) }, 2),
                Broquinho::new(helper::Position { x: (2), y: (1) }, 2),
            ],
            CANVAS_SIZE: canvas_size.clone(),
        }
    }

    pub fn hit(&self, broquinho_ref: &mut Broquinho, damage: u8) {
        broquinho_ref.damage(damage)
    }

    pub fn move_right(&mut self) {
        self.paddle.move_paddle(MovementDirection::Right);
    }

    pub fn move_left(&mut self) {
        self.paddle.move_paddle(MovementDirection::Left);
    }

    fn kill_broquinho(&self, broquinho_ref: &mut Broquinho) {
        todo!()
    }
}
