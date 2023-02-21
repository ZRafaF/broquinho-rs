// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub struct Position {
    pub x: u16,
    pub y: u16,
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
