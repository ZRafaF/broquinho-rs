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
            ball: Ball::new(
                Position {
                    x: (canvas_size.width / 2.0),
                    y: (canvas_size.height - (SAFE_PADDLE_ZONE as f32 / 2.0)),
                },
                Position {
                    x: (0.0),
                    y: (100.0),
                },
                BALL_RADIUS,
                calculated_broquinho_size,
            ),
        }
    }

    pub fn move_right(&mut self, delta_time: &f32) {
        self.paddle
            .move_paddle(MovementDirection::Right, delta_time);
    }

    #[allow(dead_code)]
    pub fn get_broquinho_at(&self, idx: usize) -> &Broquinho {
        &self.broquinho_vec[idx]
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

    pub fn process(&mut self, delta_time: &f32) {
        self.paddle.process(delta_time);
        self.ball.process(delta_time);

        let neighbor_broquinhos_indexes = get_neighbor_cells(&self);

        let colliding_broquinhos_indexes: Vec<HitResult> = check_collision(
            &mut self.ball,
            &mut self.broquinho_vec,
            neighbor_broquinhos_indexes,
            &self.paddle,
            self.broquinho_size,
        );

        solve_collisions(
            colliding_broquinhos_indexes,
            &mut self.ball,
            &mut self.broquinho_vec,
            10,
        )
    }
}

fn calculate_broquinhos_size(canvas_size: CanvasSize, broquinhos_per_row: u16) -> f32 {
    canvas_size.width / broquinhos_per_row as f32
}

fn calculate_num_of_cols(canvas_size: CanvasSize, broquinho_size: f32) -> u16 {
    (canvas_size.height - SAFE_PADDLE_ZONE) as u16 / broquinho_size as u16
}

pub fn get_neighbor_cells(game: &Game) -> Vec<u32> {
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
            if game.broquinho_vec[neighbor_pos_1d as usize].get_life() == 0 {
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
    broquinho_size: f32,
) -> Vec<HitResult> {
    let ball_screen_pos = ball.get_screen_pos();
    let ball_radius = ball.get_radius();
    let mut colliding_broquinhos_indexes: Vec<HitResult> = vec![];

    if ball_screen_pos.y + ball_radius > paddle.get_screen_pos().y - paddle.get_paddle_height()
        && ball_screen_pos.y - ball_radius < paddle.get_screen_pos().y
        && ball_screen_pos.x + ball_radius > paddle.get_screen_pos().x
        && ball_screen_pos.x - ball_radius < paddle.get_screen_pos().x + paddle.length
    {
        ball.ricochet(&CollisionDirection::Down);
        return colliding_broquinhos_indexes;
    }

    for idx in neighbor_broquinhos_indexes {
        let usize_idx = idx as usize;
        let broquinho = &broquinho_vec[usize_idx];

        if ball_screen_pos.y - ball_radius < broquinho.get_screen_pos().y + broquinho_size
            && ball_screen_pos.y + ball_radius > broquinho.get_screen_pos().y
            && ball_screen_pos.x + ball_radius > broquinho.get_screen_pos().x
            && ball_screen_pos.x - ball_radius < broquinho.get_screen_pos().x + broquinho_size
        {
            colliding_broquinhos_indexes.push(HitResult(
                usize_idx,
                calculate_collision_direction(ball, broquinho, broquinho_size),
            ));
            continue;
        }
    }
    colliding_broquinhos_indexes
}

fn calculate_collision_direction(
    ball: &Ball,
    broquinho: &Broquinho,
    broquinho_size: f32,
) -> CollisionDirection {
    let broquinho_center: Position<f32> = Position {
        x: { broquinho.get_screen_pos().x + (broquinho_size / 2.0) },
        y: { broquinho.get_screen_pos().y + (broquinho_size / 2.0) },
    };

    let ball_center = ball.get_screen_pos();

    let delta_x = ball_center.x - broquinho_center.x;
    let delta_y = ball_center.y - broquinho_center.y;

    let delta_x_abs = delta_x.abs();
    let delta_y_abs = delta_y.abs();

    if delta_x_abs > delta_y_abs && delta_x >= 0.0 {
        return CollisionDirection::Left;
    }
    if delta_x_abs > delta_y_abs && delta_x < 0.0 {
        return CollisionDirection::Right;
    }
    if delta_y_abs > delta_x_abs && delta_y >= 0.0 {
        return CollisionDirection::Top;
    }
    if delta_y_abs > delta_x_abs && delta_y <= 0.0 {
        return CollisionDirection::Down;
    }

    CollisionDirection::Down
}

fn solve_collisions(
    colliding_broquinhos_indexes: Vec<HitResult>,
    ball: &mut Ball,
    broquinho_vec: &mut Vec<Broquinho>,
    damage: i16,
) {
    //! Here im using only the first hitResult even though im passing a vector
    //! You can add a solver for multiple hits at once easily

    if colliding_broquinhos_indexes.len() > 0 {
        let collision = &colliding_broquinhos_indexes[0];
        ball.ricochet(&collision.1);
        let broquinho_life = broquinho_vec[collision.0].get_life();
        if damage < broquinho_life {
            broquinho_vec[collision.0].set_life(broquinho_life - damage);
        } else {
            broquinho_vec[collision.0].set_life(0);
        }
    }
}
