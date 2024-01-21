use macroquad::{
    color::WHITE,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{Control, ControlEvent, ControlId, InputEvent};

pub struct Button<'a> {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,

    idle_texture: &'a Texture2D,
    hover_texture: &'a Texture2D,
    pressed_texture: &'a Texture2D,

    id: ControlId,

    hovered: bool,
    pressed: bool,
}

impl<'a> Button<'a> {
    pub fn new(
        id: ControlId,
        idle_texture: &'a Texture2D,
        hover_texture: &'a Texture2D,
        pressed_texture: &'a Texture2D,
        x: f32,
        y: f32,
        rotation: f32,
    ) -> Self {
        Self {
            x,
            y,
            rotation,
            idle_texture,
            hover_texture,
            pressed_texture,
            id,
            pressed: false,
            hovered: false,
        }
    }
}

impl<'a> Control for Button<'a> {
    fn draw(&self) {
        let texture = if self.pressed {
            self.pressed_texture
        } else if self.hovered {
            self.hover_texture
        } else {
            self.idle_texture
        };

        draw_texture_ex(
            texture,
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

    fn handle_event(&mut self, event: &InputEvent, new_events: &mut Vec<ControlEvent>) {
        use crate::InputEvent::*;

        let (width, height) = self.idle_texture.size().into();

        match *event {
            MouseMoved { x, y } => {
                let x_inside = self.x <= x && x <= self.x + width;
                let y_inside = self.y <= y && y <= self.y + height;

                self.hovered = x_inside && y_inside;

                if !self.hovered {
                    self.pressed = false;
                }
            }
            MousePressed { .. } if self.hovered => {
                if !self.pressed {
                    new_events.push(ControlEvent::Pressed(self.id.clone()));
                }
                self.pressed = true;
            }
            MouseReleased => self.pressed = false,
            _ => (),
        }
    }
}
