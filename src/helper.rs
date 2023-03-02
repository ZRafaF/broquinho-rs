// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Position<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasSize {
    pub HEIGHT: f32,
    pub WIDTH: f32,
}

#[derive(Debug)]
pub struct HitResult(pub usize, pub CollisionDirection);

pub enum MovementDirection {
    Right,
    Left,
}

#[derive(Debug)]
pub enum CollisionDirection {
    Left,
    Right,
    Top,
    Down,
}

pub fn pos_to_1d(position: &Position<u16>, broquinhos_per_row: u16) -> u32 {
    (broquinhos_per_row as u32 * position.y as u32) + position.x as u32
}

pub fn screen_pos_to_pos(screen_pos: Position<f32>, broquinho_size: f32) -> Position<u16> {
    Position {
        x: ((screen_pos.x / broquinho_size) as u16),
        y: ((screen_pos.y / broquinho_size) as u16),
    }
}
