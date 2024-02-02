use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
};

use crate::Control;

pub struct Piece<'a> {
    texture: &'a Texture2D,
    x: f32,
    y: f32,
}

impl<'a> Piece<'a> {
    pub fn new(texture: &'a Texture2D, x: f32, y: f32) -> Self {
        Self { texture, x, y }
    }
}

impl<'a> Control for Piece<'a> {
    fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}

pub struct PieceCollection<'a> {
    pub pieces: Vec<Piece<'a>>,
}

impl<'a> Control for PieceCollection<'a> {
    fn draw(&self) {
        for piece in &self.pieces {
            piece.draw();
        }
    }
}
