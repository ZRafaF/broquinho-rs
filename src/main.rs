// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
// Im using the macroquad crate https://github.com/not-fl3/macroquad/

mod game;
use game::broquinho::Broquinho;

use std::collections::HashMap;

use helper::*;

use macroquad::prelude::*;

const PADDLE_HEIGHT: f32 = 10.0;
const BROQUINHOS_PER_ROW: u16 = 100;

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
    let mut game: game::Game = game::Game::new(CANVAS_SIZE.clone(), BROQUINHOS_PER_ROW);
    let broquinho_size = game.get_broquinho_size();
    let mut broquinhos_vector: Vec<Broquinho> = vec![
        Broquinho::new(Position { x: (0), y: (0) }, 1);
        BROQUINHOS_PER_ROW as usize
            * game.get_num_of_cols() as usize
    ];

    println!("Len: {}", broquinhos_vector.len());

    for i in 0..BROQUINHOS_PER_ROW {
        for j in 0..game.get_num_of_cols() - 5 {
            broquinhos_vector
                [pos_to_1d(Position { x: (i), y: (j) }, BROQUINHOS_PER_ROW) as usize] =
                Broquinho::new(Position { x: (i), y: (j) }, 5);
        }
    }

    game.set_broquinho_vec(broquinhos_vector);

    loop {
        let delta_time: f32 = get_frame_time();
        clear_background(GRAY);

        if is_key_down(KeyCode::A) {
            game.move_left(&delta_time);
        }
        if is_key_down(KeyCode::D) {
            game.move_right(&delta_time);
        }
        draw_rectangle(
            game.paddle.x as f32,
            screen_height() - PADDLE_HEIGHT - (5 as f32),
            game.paddle.length as f32,
            PADDLE_HEIGHT,
            RED,
        );

        for broquinho in game.broquinho_vec.iter() {
            draw_rectangle(
                broquinho.get_pos_x() * broquinho_size,
                broquinho.get_pos_y() * broquinho_size,
                broquinho_size,
                broquinho_size,
                BLUE,
            )
        }

        next_frame().await
    }
}
