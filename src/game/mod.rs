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
pub const BALL_RADIUS: f32 = 4.0; // Size of the zone without blocks
pub const BALL_VELOCITY_ABS: f32 = 150.0;
const PHYSICS_SUB_STEP: u8 = 8;

#[derive(Debug, Clone)]
pub struct Game {
    pub broquinho_vec: Vec<Option<Broquinho>>,
    pub balls_vec: Vec<Ball>,
    pub paddle: Paddle,
    broquinhos_per_row: u16, // Defines the size of the broquinhos
    broquinho_size: f32,
    num_of_cols: u16,
    canvas_size: CanvasSize,
}

impl Game {
    pub fn new(canvas_size: CanvasSize, broquinhos_per_row: u16) -> Self {
        let calculated_broquinho_size =
            calculate_broquinhos_size(canvas_size.clone(), broquinhos_per_row);
        Game {
            paddle: Paddle::new(canvas_size.clone(), 150.0),
            broquinhos_per_row: (broquinhos_per_row),
            broquinho_size: (calculated_broquinho_size),
            num_of_cols: calculate_num_of_cols(canvas_size.clone(), calculated_broquinho_size),
            broquinho_vec: vec![],
            balls_vec: vec![Ball::new(
                Position {
                    x: (canvas_size.width / 2.0),
                    y: (canvas_size.height - (SAFE_PADDLE_ZONE / 2.0)),
                },
                Position {
                    x: (0.0),
                    y: (BALL_VELOCITY_ABS),
                },
                BALL_RADIUS,
                calculated_broquinho_size,
                10.0,
            )],
            canvas_size: canvas_size.clone(),
        }
    }

    pub fn move_right(&mut self, delta_time: &f32) {
        self.paddle
            .move_paddle(MovementDirection::Right, delta_time);
    }

    pub fn get_ball_at(&mut self, idx: usize) -> Option<&mut Ball> {
        if idx > self.balls_vec.len() {
            return None;
        }
        return Some(&mut self.balls_vec[idx]);
    }

    pub fn create_ball(&mut self, ball: Ball) {
        self.balls_vec.push(ball);
    }

    #[allow(dead_code)]
    pub fn get_broquinho_at(&self, idx: usize) -> &Option<Broquinho> {
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

    pub fn set_broquinho_vec(&mut self, new_broquinho_vec: Vec<Option<Broquinho>>) {
        self.broquinho_vec = new_broquinho_vec;
    }

    pub fn process(&mut self, delta_time: &f32) {
        let sub_step_delta_time = delta_time / PHYSICS_SUB_STEP as f32;

        // Check out of screen
        let mut indexes_to_remove: Vec<Option<usize>> = vec![None; self.balls_vec.len()];
        for idx in 0..self.balls_vec.len() {
            if self.balls_vec[idx].get_screen_pos().y - self.balls_vec[idx].get_radius()
                > self.canvas_size.height
            {
                indexes_to_remove.push(Some(idx));
            }
        }
        for idx in indexes_to_remove {
            if idx.is_some() {
                if idx.unwrap() < self.balls_vec.len() {
                    self.balls_vec.remove(idx.unwrap());
                }
            }
        }

        for _ in 0..PHYSICS_SUB_STEP {
            self.paddle.process(&sub_step_delta_time);

            for idx in 0..self.balls_vec.len() {
                if self.get_ball_at(idx).is_none() {
                    continue;
                }
                {}
                let ball = &mut self.balls_vec[idx];
                ball.process(&sub_step_delta_time);

                let neighbor_broquinhos_indexes = get_neighbor_cells(
                    ball,
                    self.num_of_cols.clone(),
                    self.broquinhos_per_row,
                    &self.broquinho_vec,
                );

                let colliding_broquinhos_indexes: Vec<HitResult> = check_collision(
                    ball,
                    &mut self.broquinho_vec,
                    neighbor_broquinhos_indexes,
                    &self.paddle,
                    self.broquinho_size,
                    &self.canvas_size,
                );

                solve_collisions(colliding_broquinhos_indexes, ball, &mut self.broquinho_vec)
            }
        }
    }
}

fn calculate_broquinhos_size(canvas_size: CanvasSize, broquinhos_per_row: u16) -> f32 {
    canvas_size.width / broquinhos_per_row as f32
}

fn calculate_num_of_cols(canvas_size: CanvasSize, broquinho_size: f32) -> u16 {
    (canvas_size.height - SAFE_PADDLE_ZONE) as u16 / broquinho_size as u16
}

pub fn get_neighbor_cells(
    ball: &Ball,
    num_of_cols: u16,
    broquinhos_per_row: u16,
    broquinho_vec: &Vec<Option<Broquinho>>,
) -> Vec<u32> {
    let mut neighbor_broquinhos_indexes: Vec<u32> = vec![];
    let pos = ball.get_pos();
    for y in 0..=2 {
        if pos.y == 0 && y == 0 {
            continue;
        }
        if pos.y == num_of_cols - 1 && y == 2 {
            continue;
        }

        for x in 0..=2 {
            if pos.x == 0 && x == 0 {
                continue;
            }
            if pos.x == broquinhos_per_row - 1 && x == 2 {
                continue;
            }
            let neighbor_pos = Position {
                x: { pos.x + x - 1 },
                y: { pos.y + y - 1 },
            };
            let neighbor_pos_1d = helper::pos_to_1d(&neighbor_pos, broquinhos_per_row);
            if neighbor_pos_1d as usize >= broquinho_vec.len() {
                continue;
            }
            if broquinho_vec[neighbor_pos_1d as usize].is_none() {
                continue;
            }
            neighbor_broquinhos_indexes.push(neighbor_pos_1d)
        }
    }
    return neighbor_broquinhos_indexes;
}

fn check_collision(
    ball: &mut Ball,
    broquinho_vec: &mut Vec<Option<Broquinho>>,
    neighbor_broquinhos_indexes: Vec<u32>,
    paddle: &Paddle,
    broquinho_size: f32,
    canvas_size: &CanvasSize,
) -> Vec<HitResult> {
    let ball_screen_pos = ball.get_screen_pos().clone();
    let ball_radius = ball.get_radius();
    let mut colliding_broquinhos_indexes: Vec<HitResult> = vec![];

    // Checking Paddle
    if ball_screen_pos.y + ball_radius > paddle.get_screen_pos().y - paddle.get_paddle_height()
        && ball_screen_pos.y - ball_radius < paddle.get_screen_pos().y
        && ball_screen_pos.x + ball_radius > paddle.get_screen_pos().x
        && ball_screen_pos.x - ball_radius < paddle.get_screen_pos().x + paddle.length
    {
        // Where the ball hit the paddle get an -1 to 1
        let delta_paddle_position = (ball_screen_pos.x
            - (paddle.get_screen_pos().x + (paddle.length / 2.0)))
            / (paddle.length / 2.0);

        ball.set_velocity(Position {
            x: (delta_paddle_position * BALL_VELOCITY_ABS),
            y: (-ball.get_velocity().y.abs()),
        });

        return colliding_broquinhos_indexes;
    }

    // Checking Walls
    if ball_screen_pos.y - ball_radius < 0.0 {
        ball.ricochet(&CollisionDirection::Top);
        return colliding_broquinhos_indexes;
    }
    if ball_screen_pos.x - ball_radius < 0.0 {
        ball.ricochet(&CollisionDirection::Left);
        return colliding_broquinhos_indexes;
    }
    if ball_screen_pos.x + ball_radius > canvas_size.width {
        ball.ricochet(&CollisionDirection::Right);
        return colliding_broquinhos_indexes;
    }

    // Checking neighboring broquinho
    for idx in neighbor_broquinhos_indexes {
        let usize_idx = idx as usize;

        let broquinho = &broquinho_vec[usize_idx].as_ref().unwrap();

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
    broquinho_vec: &mut Vec<Option<Broquinho>>,
) {
    //! Here im using only the first hitResult even though im passing a vector
    //! You can add a solver for multiple hits at once easily

    if colliding_broquinhos_indexes.len() > 0 {
        let ball_damage = ball.get_damage();

        let collision = &colliding_broquinhos_indexes[0];
        let broquinho = broquinho_vec[collision.0].as_mut().unwrap();
        ball.ricochet(&collision.1);
        let broquinho_life = broquinho.get_life();
        if ball_damage < broquinho_life {
            broquinho.set_life(broquinho_life - ball_damage);
        } else {
            broquinho_vec[collision.0] = None;
        }
    }
}
