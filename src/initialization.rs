use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
};

use bezier_rs::Bezier;
use macroquad::texture::{load_texture, Texture2D};

use crate::{Button, ControlId, Path, Piece, PieceCollection, Position, SubpathNoId};

#[non_exhaustive]
pub struct Assets {
    pub img_board: Texture2D,
    pub img_arrow_linear: Texture2D,
    pub img_arrow_linear_hover: Texture2D,
    pub img_arrow_linear_pressed: Texture2D,
    pub img_arrow_rotational: Texture2D,
    pub img_arrow_rotational_hover: Texture2D,
    pub img_arrow_rotational_pressed: Texture2D,
    pub img_piece_yellow: Texture2D,
    pub img_piece_blue: Texture2D,
    pub img_piece_red: Texture2D,
    pub img_piece_purple: Texture2D,
    pub img_piece_green: Texture2D,
}

#[rustfmt::skip]
pub async fn load_assets() -> Assets {
    Assets {
        img_board: load_texture("assets/board.png").await.unwrap(),
        img_arrow_linear: load_texture("assets/arrow_linear.png").await.unwrap(),
        img_arrow_linear_hover: load_texture("assets/arrow_linear_hover.png").await.unwrap(),
        img_arrow_linear_pressed: load_texture("assets/arrow_linear_pressed.png").await.unwrap(),
        img_arrow_rotational: load_texture("assets/arrow_rotational.png").await.unwrap(),
        img_arrow_rotational_hover: load_texture("assets/arrow_rotational_hover.png").await.unwrap(),
        img_arrow_rotational_pressed: load_texture("assets/arrow_rotational_pressed.png").await.unwrap(),
        img_piece_yellow: load_texture("assets/piece_yellow.png").await.unwrap(),
        img_piece_blue: load_texture("assets/piece_blue.png").await.unwrap(),
        img_piece_red: load_texture("assets/piece_red.png").await.unwrap(),
        img_piece_purple: load_texture("assets/piece_purple.png").await.unwrap(),
        img_piece_green: load_texture("assets/piece_green.png").await.unwrap(),
    }
}

fn piece_location(row: i32, col: i32) -> (f32, f32) {
    const TOP_LEFT_X: f32 = 50.200;
    const TOP_LEFT_Y: f32 = 50.200;
    const PIECE_DISTANCE: f32 = 45.237;

    (
        TOP_LEFT_X + col as f32 * PIECE_DISTANCE,
        TOP_LEFT_Y + row as f32 * PIECE_DISTANCE,
    )
}

#[rustfmt::skip]
pub fn initialize_buttons(assets: &Assets) -> Vec<Button> {
    let new_linear_button = |x, y, rotation, id| {
        Button::new(
            id,
            &assets.img_arrow_linear,
            &assets.img_arrow_linear_hover,
            &assets.img_arrow_linear_pressed,
            x,
            y,
            rotation,
        )
    };

    let new_rotational_button = |x, y, rotation| {
        Button::new(
            ControlId::RotationalArrow,
            &assets.img_arrow_rotational,
            &assets.img_arrow_rotational_hover,
            &assets.img_arrow_rotational_pressed,
            x,
            y,
            rotation,
        )
    };

    vec![
        // Linear buttons
        new_linear_button(186.646, 4.920, 0., ControlId::VerticalUp(3)),
        new_linear_button(231.884, 4.920, 0., ControlId::VerticalUp(4)),
        new_linear_button(277.122, 4.920, 0., ControlId::VerticalUp(5)),
        new_linear_button(186.646, 457.298, PI, ControlId::VerticalDown(3)),
        new_linear_button(231.884, 457.298, PI, ControlId::VerticalDown(4)),
        new_linear_button(277.122, 457.298, PI, ControlId::VerticalDown(5)),
        new_linear_button(4.920, 186.646, -FRAC_PI_2, ControlId::HorizontalLeft(3)),
        new_linear_button(4.920, 231.884, -FRAC_PI_2, ControlId::HorizontalLeft(4)),
        new_linear_button(4.920, 277.122, -FRAC_PI_2, ControlId::HorizontalLeft(5)),
        new_linear_button(457.298, 186.646, FRAC_PI_2, ControlId::HorizontalRight(3)),
        new_linear_button(457.298, 231.884, FRAC_PI_2, ControlId::HorizontalRight(4)),
        new_linear_button(457.298, 277.122, FRAC_PI_2, ControlId::HorizontalRight(5)),
        // Rotational buttons
        new_rotational_button(85.462, 85.462, 0.),
        new_rotational_button(117.330, 117.330, 0.),
        new_rotational_button(146.381, 146.381, 0.),
        new_rotational_button(320.000, 320.000, PI),
        new_rotational_button(349.052, 349.052, PI),
        new_rotational_button(380.919, 380.919, PI),
        new_rotational_button(320.000, 146.381, -FRAC_PI_2),
        new_rotational_button(349.052, 117.330, -FRAC_PI_2),
        new_rotational_button(380.919, 85.462, -FRAC_PI_2),
        new_rotational_button(146.381, 320.000, FRAC_PI_2),
        new_rotational_button(117.330, 349.052, FRAC_PI_2),
        new_rotational_button(85.462, 380.919, FRAC_PI_2),
    ]
}

pub type PathMap = HashMap<(Position, Position), Path>;

pub fn initialize_paths() -> PathMap {
    let mut map = HashMap::new();

    let linear_path_between = |row_from, col_from, row_to, col_to| {
        let (x_from, y_from) = piece_location(row_from, col_from);
        let (x_to, y_to) = piece_location(row_to, col_to);

        SubpathNoId::from_bezier(&Bezier::from_linear_coordinates(
            x_from as f64,
            y_from as f64,
            x_to as f64,
            y_to as f64,
        ))
    };

    // Horizontal sliding right
    for row in 3..6 {
        // Without ghosts
        for col in 0..6 {
            let col_to = col + 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new(row, col_to).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32,
                        col as i32,
                        row as i32,
                        col_to as i32,
                    ),
                    ghost_path: None,
                },
            );
        }

        // With ghosts
        for col in 6..9 {
            let col_to = col + 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new(row, col_to % 9).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32,
                        col as i32 - 9,
                        row as i32,
                        col_to as i32 % 9,
                    ),
                    ghost_path: Some(linear_path_between(
                        row as i32,
                        col as i32,
                        row as i32,
                        col_to as i32,
                    )),
                },
            );
        }
    }

    map
}

pub fn initialize_pieces<'a>(assets: &'a Assets, paths: &'a PathMap) -> PieceCollection<'a> {
    let mut pieces = vec![];

    // Yellow pieces
    for row in 3..6 {
        for col in 3..6 {
            let (x, y) = piece_location(row as i32, col as i32);
            pieces.push(Piece::new(
                &assets.img_piece_yellow,
                Position::new(row, col).unwrap(),
                x,
                y,
            ));
        }
    }

    // Blue pieces
    for row in 3..6 {
        for col in 6..9 {
            let (x, y) = piece_location(row as i32, col as i32);
            pieces.push(Piece::new(
                &assets.img_piece_blue,
                Position::new(row, col).unwrap(),
                x,
                y,
            ));
        }
    }

    // Purple pieces
    for row in 3..6 {
        for col in 0..3 {
            let (x, y) = piece_location(row as i32, col as i32);
            pieces.push(Piece::new(
                &assets.img_piece_purple,
                Position::new(row, col).unwrap(),
                x,
                y,
            ));
        }
    }

    // Green pieces
    for row in 0..3 {
        for col in 3..6 {
            let (x, y) = piece_location(row as i32, col as i32);
            pieces.push(Piece::new(
                &assets.img_piece_green,
                Position::new(row, col).unwrap(),
                x,
                y,
            ));
        }
    }

    // Red pieces
    for row in 6..9 {
        for col in 3..6 {
            let (x, y) = piece_location(row as i32, col as i32);
            pieces.push(Piece::new(
                &assets.img_piece_red,
                Position::new(row, col).unwrap(),
                x,
                y,
            ));
        }
    }

    PieceCollection {
        pieces,
        path_map: paths,
    }
}
