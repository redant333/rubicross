use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
};

use crate::Control;

pub struct Sprite<'a> {
    texture: &'a Texture2D,
    x: f32,
    y: f32,
}

impl<'a> Sprite<'a> {
    pub fn new(texture: &'a Texture2D, x: f32, y: f32) -> Sprite {
        Self { texture, x, y }
    }
}

impl<'a> Control for Sprite<'a> {
    fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
