use crate::{pieces::position, Control};
use macroquad::{
    color::WHITE,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

#[derive(Debug)]
pub struct SolvedMarker<'a> {
    x: f32,
    y: f32,
    idle_texture: &'a Texture2D,
    square: position::Square,
    rotation: f32,
}

impl<'a> Control for SolvedMarker<'a> {
    fn draw(&self) {
        draw_texture_ex(
            self.idle_texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation: self.rotation,
                pivot: None,
                ..Default::default()
            },
        )
    }
}

impl<'a> SolvedMarker<'a> {
    pub fn new(
        x: f32,
        y: f32,
        square: position::Square,
        idle_texture: &'a Texture2D,
        rotation: f32,
    ) -> Self {
        Self {
            x,
            y,
            square,
            idle_texture,
            rotation,
        }
    }

    pub fn square(&self) -> &position::Square {
        &self.square
    }
}
