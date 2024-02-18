use crate::{pieces::position, Control};
use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
};

pub struct SolvedMarker<'a> {
    x: f32,
    y: f32,
    visible: bool,
    idle_texture: &'a Texture2D,
    square: position::Square,
}

impl<'a> Control for SolvedMarker<'a> {
    fn draw(&self) {
        if self.visible {
            draw_texture(self.idle_texture, self.x, self.y, WHITE)
        }
    }
}

impl<'a> SolvedMarker<'a> {
    pub fn new(
        x: f32,
        y: f32,
        square: position::Square,
        visible: bool,
        idle_texture: &'a Texture2D,
    ) -> Self {
        Self {
            x,
            y,
            square,
            visible,
            idle_texture,
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn square(&self) -> &position::Square {
        &self.square
    }
}
