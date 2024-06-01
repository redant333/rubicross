mod button;
pub use button::Button;

mod pieces;
pub use pieces::collection::Manipulation;
pub use pieces::collection::PieceCollection;
pub use pieces::path::Path;
pub use pieces::path::SubpathNoId;
pub use pieces::piece::Piece;
pub use pieces::position::PieceError;
pub use pieces::position::Position;

pub mod game;
pub mod initialization;
pub mod solved_marker;

use macroquad::prelude::*;
use game::Game;
use initialization::{
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
    game.run_shuffle(1, 0.15).await;
    game.run_main_loop().await;
    game.wait(1.0).await;
    game.run_victory_loop().await;
}
