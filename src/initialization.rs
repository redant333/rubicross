use macroquad::texture::{load_texture, Texture2D};

use crate::{Button, Draw, Sprite};

#[non_exhaustive]
pub struct Assets {
    pub img_board: Texture2D,
    pub img_arrow_linear: Texture2D,
}

pub async fn load_assets() -> Assets {
    Assets {
        img_board: load_texture("assets/board.png").await.unwrap(),
        img_arrow_linear: load_texture("assets/arrow_linear.png").await.unwrap(),
    }
}

pub fn initialize_controls(assets: &Assets) -> Vec<Box<dyn Draw + '_>> {
    vec![
        // Background
        Box::new(Sprite::new(&assets.img_board, 0., 0.)),
        // Linear buttons
        Box::new(Button::new(&assets.img_arrow_linear, 186.646, 4.920)),
        Box::new(Button::new(&assets.img_arrow_linear, 231.884, 4.920)),
        Box::new(Button::new(&assets.img_arrow_linear, 277.122, 4.920)),
        Box::new(Button::new(&assets.img_arrow_linear, 186.646, 457.298)),
        Box::new(Button::new(&assets.img_arrow_linear, 231.884, 457.298)),
        Box::new(Button::new(&assets.img_arrow_linear, 277.122, 457.298)),
        Box::new(Button::new(&assets.img_arrow_linear, 4.920, 186.646)),
        Box::new(Button::new(&assets.img_arrow_linear, 4.920, 231.884)),
        Box::new(Button::new(&assets.img_arrow_linear, 4.920, 277.122)),
        Box::new(Button::new(&assets.img_arrow_linear, 457.298, 186.646)),
        Box::new(Button::new(&assets.img_arrow_linear, 457.298, 231.884)),
        Box::new(Button::new(&assets.img_arrow_linear, 457.298, 277.122)),
    ]
}
