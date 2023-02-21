// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
// Im using the macroquad crate https://github.com/not-fl3/macroquad/

use broquinho;
use broquinho::CanvasSize;
use macroquad::prelude::*;

const PADDLE_HEIGHT: f32 = 10.0;

const CANVAS_SIZE: CanvasSize = CanvasSize {
    HEIGHT: 600.0,
    WIDTH: 800.0,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Broquinho".to_owned(),
        window_height: CANVAS_SIZE.HEIGHT as i32,
        window_width: CANVAS_SIZE.WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)] // Name of the app
async fn main() {
    let mut game: broquinho::Game = broquinho::Game::new(CANVAS_SIZE.clone());
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
