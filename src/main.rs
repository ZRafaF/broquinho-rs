// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
// Im using the macroquad crate https://github.com/not-fl3/macroquad/

mod game;
use game::broquinho::Broquinho;

use helper::*;

use macroquad::prelude::*;

const BROQUINHOS_PER_ROW: u16 = 51;

// GRAPHICS OPTIONS

const BROQUINHO_OUTLINE_THICKNESS: f32 = 1.0;

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

#[macroquad::main(window_conf)] // Configs of the app
async fn main() {
    let mut game: game::Game = game::Game::new(CANVAS_SIZE.clone(), BROQUINHOS_PER_ROW);
    let broquinho_size = game.get_broquinho_size();
    let mut broquinhos_vector: Vec<Broquinho> = vec![
        Broquinho::new(
            Position { x: (0), y: (0) },
            Position { x: (0.0), y: (0.0) },
            1
        );
        BROQUINHOS_PER_ROW as usize
            * game.get_num_of_cols() as usize
    ];

    for i in 0..BROQUINHOS_PER_ROW {
        for j in 0..game.get_num_of_cols() {
            broquinhos_vector
                [pos_to_1d(&Position { x: (i), y: (j) }, BROQUINHOS_PER_ROW) as usize] =
                Broquinho::new(
                    Position { x: (i), y: (j) },
                    Position {
                        x: (i as f32 * broquinho_size) + (broquinho_size / 2.0),
                        y: (j as f32 * broquinho_size) + (broquinho_size / 2.0),
                    },
                    5,
                );
        }
    }

    println!("Broquinho size: {}", broquinho_size);

    game.set_broquinho_vec(broquinhos_vector);
    loop {
        let delta_time: f32 = get_frame_time();
        clear_background(GRAY);

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            game.move_left(&delta_time);
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            game.move_right(&delta_time);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < CANVAS_SIZE.WIDTH / 2.0 {
                game.move_left(&delta_time);
            } else {
                game.move_right(&delta_time);
            }
            game.ball.set_screen_pos(Position {
                x: (mouse_x),
                y: (mouse_y),
            });
        }

        game.process(&delta_time);

        let paddle_pos = game.paddle.get_screen_pos();

        draw_rectangle(
            paddle_pos.x,
            screen_height() - game.paddle.get_paddle_height() - paddle_pos.y,
            game.paddle.length as f32,
            game.paddle.get_paddle_height(),
            RED,
        );

        for broquinho in game.broquinho_vec.iter() {
            // Draw the broquinho it self
            draw_rectangle(
                broquinho.get_screen_pos().x - (broquinho_size / 2.0),
                broquinho.get_screen_pos().y - (broquinho_size / 2.0),
                broquinho_size,
                broquinho_size,
                BLUE,
            );

            // Draw outline
            draw_rectangle_lines(
                broquinho.get_screen_pos().x - (broquinho_size / 2.0),
                broquinho.get_screen_pos().y - (broquinho_size / 2.0),
                broquinho_size,
                broquinho_size,
                BROQUINHO_OUTLINE_THICKNESS,
                BLACK,
            );

            // Draw center of the cube
            draw_circle(
                broquinho.get_screen_pos().x,
                broquinho.get_screen_pos().y,
                1.0,
                RED,
            )
        }

        //let neighbor_broquinhos: Vec<&Broquinho> = game::get_neighbor_cells(&game);
        /*
        for broquinho in neighbor_broquinhos.iter() {
            draw_rectangle(
                broquinho.get_screen_pos().x - (broquinho_size / 2.0),
                broquinho.get_screen_pos().y - (broquinho_size / 2.0),
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
