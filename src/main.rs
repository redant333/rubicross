use macroquad::prelude::*;
use rubicross::game::Game;
use rubicross::initialization::{
    initialize_buttons, initialize_paths, initialize_pieces, initialize_solved_markers, load_assets,
};

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
    let paths = initialize_paths();

    let mut game = Game {
        assets: &assets,
        buttons: initialize_buttons(&assets),
        solved_markers: initialize_solved_markers(&assets),
        pieces: initialize_pieces(&assets, &paths),
    };

    rand::srand(macroquad::miniquad::date::now() as u64);

    game.wait(1.0).await;
    game.perform_shuffle(1, 0.15).await;
    game.run_main_loop().await;
    game.wait(1.0).await;
}
