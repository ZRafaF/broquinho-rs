// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
// Im using the macroquad crate https://github.com/not-fl3/macroquad/

use broquinho::{self, Paddle};
use macroquad::prelude::*;

const PADDLE_HEIGHT: f32 = 10.0;

#[macroquad::main("Broquinho")] // Name of the app
async fn main() {
    let mut game: broquinho::Game = broquinho::Game::new();
    println!("Paddle x: {}", game.paddle.x);
    loop {
        clear_background(GRAY);

        if is_key_down(KeyCode::A) {
            game.move_left();
        }
        if is_key_down(KeyCode::D) {
            game.move_right();
        }
        draw_rectangle(
            game.paddle.x as f32,
            screen_height() - PADDLE_HEIGHT - (5 as f32),
            game.paddle.length as f32,
            PADDLE_HEIGHT,
            RED,
        );

        next_frame().await
    }
}
