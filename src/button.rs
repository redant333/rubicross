use macroquad::{
    color::WHITE,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

#[derive(Debug)]
pub enum ButtonEvent {
    Pressed(ButtonId),
}

#[non_exhaustive]
pub enum InputEvent {
    MouseMoved { x: f32, y: f32 },
    MousePressed { x: f32, y: f32 },
    MouseReleased,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonId {
    HorizontalLeft(u8),
    HorizontalRight(u8),
    VerticalUp(u8),
    VerticalDown(u8),
    RotateClockwise(u8),
    RotateAnticlockwise(u8),
}

pub struct Button<'a> {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,

    idle_texture: &'a Texture2D,
    hover_texture: &'a Texture2D,
    pressed_texture: &'a Texture2D,

    id: ButtonId,

    hovered: bool,
    pressed: bool,
}

impl<'a> Button<'a> {
    pub fn new(
        id: ButtonId,
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

    pub fn id(&self) -> ButtonId {
        self.id
    }

    pub fn draw(&self) {
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

    pub fn handle_event(&mut self, event: &InputEvent, new_events: &mut Vec<ButtonEvent>) {
        use InputEvent::*;

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
                    new_events.push(ButtonEvent::Pressed(self.id));
                }
                self.pressed = true;
            }
            MouseReleased => self.pressed = false,
            _ => (),
        }
    }
}
