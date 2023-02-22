// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[derive(Debug, Clone)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone)]
pub struct CanvasSize {
    pub HEIGHT: f32,
    pub WIDTH: f32,
}

pub enum MovementDirection {
    Right,
    Left,
}

pub fn pos_to_1d(position: Position<u16>, broquinhos_per_row: u16) -> u32 {
    (broquinhos_per_row as u32 * position.y as u32) + position.x as u32
}
