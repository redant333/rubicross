use crate::{pieces::position, Control};
use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
};

#[derive(Debug)]
pub struct SolvedMarker<'a> {
    x: f32,
    y: f32,
    idle_texture: &'a Texture2D,
    square: position::Square,
}

impl<'a> Control for SolvedMarker<'a> {
    fn draw(&self) {
        draw_texture(self.idle_texture, self.x, self.y, WHITE)
    }
}

impl<'a> SolvedMarker<'a> {
    pub fn new(x: f32, y: f32, square: position::Square, idle_texture: &'a Texture2D) -> Self {
        Self {
            x,
            y,
            square,
            idle_texture,
        }
    }

    pub fn square(&self) -> &position::Square {
        &self.square
    }
}
