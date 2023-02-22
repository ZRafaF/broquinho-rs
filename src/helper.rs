// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[derive(Debug, Clone, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct PositionScreen {
    pub x: f32,
    pub y: f32,
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

pub struct PositionTuple(Position, PositionScreen);

pub fn pos_to_1d(position: Position, broquinhos_per_row: u16) -> u32 {
    (broquinhos_per_row as u32 * position.y as u32) + position.x as u32
}
