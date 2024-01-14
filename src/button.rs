use macroquad::{
    color::WHITE,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::Draw;

pub struct Button<'a> {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,

    idle_texture: &'a Texture2D,
}

impl<'a> Button<'a> {
    pub fn new(idle_texture: &'a Texture2D, x: f32, y: f32, rotation: f32) -> Self {
        Self {
            x,
            y,
            rotation,
            idle_texture,
        }
    }
}

impl<'a> Draw for Button<'a> {
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
        );
    }
}
