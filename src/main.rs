use macroquad::prelude::*;
use rubicross::initialization::{initialize_controls, load_assets};

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
    let assets = load_assets().await;
    let drawables = initialize_controls(&assets);

    loop {
        clear_background(color_u8!(0xc5, 0xba, 0xaf, 0xff));

        for drawable in &drawables {
            drawable.draw();
        }

        next_frame().await
    }
}
