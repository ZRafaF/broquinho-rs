// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
// Im using the macroquad crate https://github.com/not-fl3/macroquad/

mod game;
use game::broquinho::Broquinho;

use helper::*;

use macroquad::prelude::*;

const BROQUINHOS_PER_ROW: u16 = 81;

const BROQUINHO_OUTLINE_THICKNESS: f32 = 1.0;

const STARTING_LIFE: f32 = 15.0;

const CANVAS_SIZE: CanvasSize = CanvasSize {
    height: 600.0,
    width: 800.0,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Broquinho".to_owned(),
        window_height: CANVAS_SIZE.height as i32,
        window_width: CANVAS_SIZE.width as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)] // Configs of the app
async fn main() {
    let mut game: game::Game = game::Game::new(CANVAS_SIZE.clone(), BROQUINHOS_PER_ROW);
    let broquinho_size = game.get_broquinho_size();
    let mut broquinhos_vector: Vec<Broquinho> =
        vec![
            Broquinho::new(Position { x: (0.0), y: (0.0) }, STARTING_LIFE);
            BROQUINHOS_PER_ROW as usize * game.get_num_of_cols() as usize
        ];

    for i in 0..BROQUINHOS_PER_ROW {
        for j in 0..game.get_num_of_cols() {
            broquinhos_vector
                [pos_to_1d(&Position { x: (i), y: (j) }, BROQUINHOS_PER_ROW) as usize] =
                Broquinho::new(
                    Position {
                        x: (i as f32 * broquinho_size),
                        y: (j as f32 * broquinho_size),
                    },
                    STARTING_LIFE,
                );
        }
    }

    game.set_broquinho_vec(broquinhos_vector);
    loop {
        let delta_time: f32 = get_frame_time();
        clear_background(GRAY);

        let (mouse_x, mouse_y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            if mouse_x < CANVAS_SIZE.width / 2.0 {
                game.move_left(&delta_time);
            } else {
                game.move_right(&delta_time);
            }
        } else {
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                game.move_left(&delta_time);
            }
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                game.move_right(&delta_time);
            }
        }

        if is_key_down(KeyCode::R) || is_key_down(KeyCode::Right) {
            game.ball.set_screen_pos(Position {
                x: (mouse_x),
                y: (mouse_y),
            });
        }

        game.process(&delta_time);

        let paddle_pos = game.paddle.get_screen_pos();

        draw_rectangle(
            paddle_pos.x,
            paddle_pos.y - game.paddle.get_paddle_height(),
            game.paddle.length as f32,
            game.paddle.get_paddle_height(),
            RED,
        );

        for broquinho in game.broquinho_vec.iter() {
            if broquinho.get_life() == 0.0 {
                continue;
            }

            // Draw the broquinho it self
            draw_rectangle(
                broquinho.get_screen_pos().x + BROQUINHO_OUTLINE_THICKNESS,
                broquinho.get_screen_pos().y + BROQUINHO_OUTLINE_THICKNESS,
                broquinho_size - (BROQUINHO_OUTLINE_THICKNESS * 2.0),
                broquinho_size - (BROQUINHO_OUTLINE_THICKNESS * 2.0),
                Color {
                    r: (broquinho.get_life() / STARTING_LIFE),
                    g: (0.6),
                    b: (1.0),
                    a: (1.0),
                },
            );

            // Draw outline
            draw_rectangle_lines(
                broquinho.get_screen_pos().x,
                broquinho.get_screen_pos().y,
                broquinho_size,
                broquinho_size,
                BROQUINHO_OUTLINE_THICKNESS,
                BLACK,
            );

            /*
            // Draw center of the cube
            draw_circle(
                broquinho.get_screen_pos().x + (broquinho_size / 2.0),
                broquinho.get_screen_pos().y + (broquinho_size / 2.0),
                1.0,
                RED,
            )
            */
        }

        /*
        let neighbor_broquinhos_indexes: Vec<u32> = game::get_neighbor_cells(&game);
        for idx in neighbor_broquinhos_indexes {
            let broquinho = game.get_broquinho_at(idx as usize);
            draw_rectangle(
                broquinho.get_screen_pos().x,
                broquinho.get_screen_pos().y,
                broquinho_size,
                broquinho_size,
                RED,
            );
        }
        */

        draw_circle(
            game.ball.get_screen_pos().x,
            game.ball.get_screen_pos().y,
            game.ball.get_radius(),
            YELLOW,
        );
        next_frame().await
    }
}
