use macroquad::prelude::*;
use rubicross::{Button, Draw, Sprite};

fn window_conf() -> Conf {
    Conf {
        window_title: "Rubicross".into(),
        window_height: 500,
        window_width: 500,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let board = load_texture("assets/board.png").await.unwrap();
    let arrow_linear = load_texture("assets/arrow_linear.png").await.unwrap();

    let background = Sprite::new(&board, 0., 0.);
    let button = Button::new(&arrow_linear, 50.0, 50.0);

    let drawables: Vec<Box<dyn Draw>> = vec![Box::new(background), Box::new(button)];

    loop {
        clear_background(color_u8!(0xc5, 0xba, 0xaf, 0xff));

        for drawable in &drawables {
            drawable.draw();
        }

        next_frame().await
    }
}
