// Copyright (c) 2023 Rafael F. Meneses
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
pub mod broquinho;
use broquinho::Broquinho;

mod paddle;
use paddle::Paddle;

pub mod ball;
use ball::Ball;

// Import everything from the helper module
use helper::*;

const SAFE_PADDLE_ZONE: f32 = 150.0; // Size of the zone without blocks
const BALL_RADIUS: f32 = 4.0; // Size of the zone without blocks

#[derive(Debug, Clone)]
pub struct Game {
    pub broquinho_vec: Vec<Broquinho>,
    pub ball: Ball,
    pub paddle: Paddle,
    CANVAS_SIZE: CanvasSize,
    broquinhos_per_row: u16, // Defines the size of the broquinhos
    broquinho_size: f32,
    num_of_cols: u16,
}

impl Game {
    pub fn new(canvas_size: CanvasSize, broquinhos_per_row: u16) -> Self {
        let calculated_broquinho_size =
            calculate_broquinhos_size(canvas_size.clone(), broquinhos_per_row);
        Game {
            paddle: Paddle::new(canvas_size.clone()),
            broquinhos_per_row: (broquinhos_per_row),
            broquinho_size: (calculated_broquinho_size),
            num_of_cols: calculate_num_of_cols(canvas_size.clone(), calculated_broquinho_size),
            broquinho_vec: vec![],
            CANVAS_SIZE: canvas_size.clone(),
            ball: Ball::new(
                Position {
                    x: (canvas_size.WIDTH / 2.0),
                    y: (canvas_size.HEIGHT - (SAFE_PADDLE_ZONE as f32 / 2.0)),
                },
                Position {
                    x: (0.0),
                    y: (-10.0),
                },
                BALL_RADIUS,
                calculated_broquinho_size,
            ),
        }
    }

    pub fn hit(&self, broquinho_ref: &mut Broquinho, damage: u8) {
        broquinho_ref.damage(damage)
    }

    pub fn move_right(&mut self, delta_time: &f32) {
        self.paddle
            .move_paddle(MovementDirection::Right, delta_time);
    }

    pub fn move_left(&mut self, delta_time: &f32) {
        self.paddle.move_paddle(MovementDirection::Left, delta_time);
    }

    pub fn get_broquinho_size(&self) -> f32 {
        self.broquinho_size
    }

    pub fn get_num_of_cols(&self) -> u16 {
        self.num_of_cols
    }

    pub fn set_broquinho_vec(&mut self, new_broquinho_vec: Vec<Broquinho>) {
        self.broquinho_vec = new_broquinho_vec;
    }

    fn kill_broquinho(&self, broquinho_ref: &mut Broquinho) {
        todo!()
    }

    pub fn process(&mut self, delta_time: &f32) {
        self.paddle.process(delta_time);
        self.ball.process(delta_time);

        let neighbor_broquinhos_indexes = get_neighbor_cells(self.clone());
        check_collision(
            &mut self.ball,
            &mut self.broquinho_vec,
            neighbor_broquinhos_indexes,
            &self.paddle,
        );
    }
}

fn calculate_broquinhos_size(canvas_size: CanvasSize, broquinhos_per_row: u16) -> f32 {
    canvas_size.WIDTH / broquinhos_per_row as f32
}

fn calculate_num_of_cols(canvas_size: CanvasSize, broquinho_size: f32) -> u16 {
    (canvas_size.HEIGHT - SAFE_PADDLE_ZONE) as u16 / broquinho_size as u16
}

pub fn get_neighbor_cells(game: Game) -> Vec<u32> {
    let mut neighbor_broquinhos_indexes: Vec<u32> = vec![];
    let pos = game.ball.get_pos();
    for y in 0..=2 {
        if pos.y == 0 && y == 0 {
            continue;
        }
        if pos.y == game.num_of_cols - 1 && y == 2 {
            continue;
        }

        for x in 0..=2 {
            if pos.x == 0 && x == 0 {
                continue;
            }
            if pos.x == game.broquinhos_per_row - 1 && x == 2 {
                continue;
            }
            let neighbor_pos = Position {
                x: { pos.x + x - 1 },
                y: { pos.y + y - 1 },
            };
            let neighbor_pos_1d = helper::pos_to_1d(&neighbor_pos, game.broquinhos_per_row);
            if neighbor_pos_1d as usize >= game.broquinho_vec.len() {
                continue;
            }
            neighbor_broquinhos_indexes.push(neighbor_pos_1d)
        }
    }
    return neighbor_broquinhos_indexes;
}

fn check_collision(
    ball: &mut Ball,
    broquinho_vec: &mut Vec<Broquinho>,
    neighbor_broquinhos_indexes: Vec<u32>,
    paddle: &Paddle,
) {
    if neighbor_broquinhos_indexes.len() > 2 {
        ball.ricocchet(CollisionDirection::Top);
        return;
    }
}
