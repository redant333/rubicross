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
            animation: None,
            x,
            y,
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
        });
    }

    pub fn update(&mut self) {
        if let Some(animation) = self.animation.as_ref() {
            let time_elapsed = get_time() - animation.movement_start;
            let path_pos =
                bezier_rs::SubpathTValue::GlobalEuclidean(time_elapsed / animation.movement_time);
            let pos = animation.path.main_path.evaluate(path_pos);

            self.x = pos.x as f32;
            self.y = pos.y as f32;
        }
    }
}

impl<'a> Control for Piece<'a> {
    fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
