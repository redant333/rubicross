use std::f32::consts::{FRAC_PI_2, PI};

use macroquad::texture::{load_texture, Texture2D};

use crate::{Button, Control, Sprite};

#[non_exhaustive]
pub struct Assets {
    pub img_board: Texture2D,
    pub img_arrow_linear: Texture2D,
    pub img_arrow_linear_hover: Texture2D,
    pub img_arrow_linear_pressed: Texture2D,
}

pub async fn load_assets() -> Assets {
    Assets {
        img_board: load_texture("assets/board.png").await.unwrap(),
        img_arrow_linear: load_texture("assets/arrow_linear.png").await.unwrap(),
        img_arrow_linear_hover: load_texture("assets/arrow_linear_hover.png").await.unwrap(),
        img_arrow_linear_pressed: load_texture("assets/arrow_linear_pressed.png")
            .await
            .unwrap(),
    }
}

pub fn initialize_controls(assets: &Assets) -> Vec<Box<dyn Control + '_>> {
    let new_linear_button = |x, y, rotation| {
        Button::new(
            &assets.img_arrow_linear,
            &assets.img_arrow_linear_hover,
            &assets.img_arrow_linear_pressed,
            x,
            y,
            rotation,
        )
    };

    vec![
        // Background
        Box::new(Sprite::new(&assets.img_board, 0., 0.)),
        // Linear buttons
        Box::new(new_linear_button(186.646, 4.920, 0.)),
        Box::new(new_linear_button(231.884, 4.920, 0.)),
        Box::new(new_linear_button(277.122, 4.920, 0.)),
        Box::new(new_linear_button(186.646, 457.298, PI)),
        Box::new(new_linear_button(231.884, 457.298, PI)),
        Box::new(new_linear_button(277.122, 457.298, PI)),
        Box::new(new_linear_button(4.920, 186.646, -FRAC_PI_2)),
        Box::new(new_linear_button(4.920, 231.884, -FRAC_PI_2)),
        Box::new(new_linear_button(4.920, 277.122, -FRAC_PI_2)),
        Box::new(new_linear_button(457.298, 186.646, FRAC_PI_2)),
        Box::new(new_linear_button(457.298, 231.884, FRAC_PI_2)),
        Box::new(new_linear_button(457.298, 277.122, FRAC_PI_2)),
    ]
}
