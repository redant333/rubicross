use bezier_rs::SubpathTValue;
use macroquad::{
    color::WHITE,
    texture::{draw_texture, Texture2D},
    time::get_time,
};

use crate::{Control, Path, Position};

struct AnimationParams<'a> {
    path: &'a Path,
    movement_start: f64,
    movement_time: f64,
    ghost_x: Option<f32>,
    ghost_y: Option<f32>,
}

pub struct Piece<'a> {
    texture: &'a Texture2D,
    position: Position,
    x: f32,
    y: f32,
    animation: Option<AnimationParams<'a>>,
}

impl<'a> Piece<'a> {
    pub fn new(texture: &'a Texture2D, position: Position, x: f32, y: f32) -> Self {
        Self {
            position,
            texture,
            x,
            y,
            animation: None,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn start_moving_along(&mut self, path: &'a Path, time: f64) {
        self.animation = Some(AnimationParams {
            path,
            movement_start: get_time(),
            movement_time: time,
            ghost_x: None,
            ghost_y: None,
        });
    }

    pub fn update(&mut self) {
        let Some(animation) = self.animation.as_mut() else {
            return;
        };

        let time_elapsed = get_time() - animation.movement_start;
        let path_pos = SubpathTValue::GlobalEuclidean(time_elapsed / animation.movement_time);

        let pos = animation.path.main_path.evaluate(path_pos);

        self.x = pos.x as f32;
        self.y = pos.y as f32;

        let Some(ghost_path) = animation.path.ghost_path.as_ref() else {
            return;
        };

        let pos = ghost_path.evaluate(path_pos);
        animation.ghost_x = Some(pos.x as f32);
        animation.ghost_y = Some(pos.y as f32);
    }
}

impl<'a> Control for Piece<'a> {
    fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);

        let Some(animation) = self.animation.as_ref() else {
            return;
        };

        let Some(ghost_x) = animation.ghost_x else {
            return;
        };

        let Some(ghost_y) = animation.ghost_y else {
            return;
        };

        draw_texture(self.texture, ghost_x, ghost_y, WHITE);
    }
}
