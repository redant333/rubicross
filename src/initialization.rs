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
    pub img_surroundings: Texture2D,
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
        img_surroundings: load_texture("assets/surroundings.png").await.unwrap(),
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

    let new_rotational_button = |x, y, rotation, id| {
        Button::new(
            id,
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
        new_rotational_button(85.462, 85.462, 0., ControlId::RotateClockwise(2)),
        new_rotational_button(117.330, 117.330, 0.,ControlId::RotateClockwise(1)),
        new_rotational_button(146.381, 146.381, 0., ControlId::RotateClockwise(0)),
        new_rotational_button(320.000, 320.000, PI, ControlId::RotateClockwise(0)),
        new_rotational_button(349.052, 349.052, PI, ControlId::RotateClockwise(1)),
        new_rotational_button(380.919, 380.919, PI, ControlId::RotateClockwise(2)),
        new_rotational_button(320.000, 146.381, -FRAC_PI_2, ControlId::RotateAnticlockwise(0)),
        new_rotational_button(349.052, 117.330, -FRAC_PI_2, ControlId::RotateAnticlockwise(1)),
        new_rotational_button(380.919, 85.462, -FRAC_PI_2, ControlId::RotateAnticlockwise(2)),
        new_rotational_button(146.381, 320.000, FRAC_PI_2, ControlId::RotateAnticlockwise(0)),
        new_rotational_button(117.330, 349.052, FRAC_PI_2, ControlId::RotateAnticlockwise(1)),
        new_rotational_button(85.462, 380.919, FRAC_PI_2, ControlId::RotateAnticlockwise(2)),
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

    // Horizontal
    for row in 3..6 {
        // Right, without ghosts
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

        // Right, with ghosts
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

        // Left, without ghosts
        for col in 3..9 {
            let col_to = col - 3;

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

        // Left, with ghosts
        for col in 0..3 {
            let col_to = col as i32 - 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new(row, (col_to + 9) as u8).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32,
                        col as i32 + 9,
                        row as i32,
                        col_to + 9,
                    ),
                    ghost_path: Some(linear_path_between(
                        row as i32, col as i32, row as i32, col_to,
                    )),
                },
            );
        }
    }

    // Vertical
    for col in 3..6 {
        // Down, without ghosts
        for row in 0..6 {
            let row_to = row + 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new(row_to, col).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32,
                        col as i32,
                        row_to as i32,
                        col as i32,
                    ),
                    ghost_path: None,
                },
            );
        }

        // Down, with ghosts
        for row in 6..9 {
            let row_to = row + 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new(row_to % 9, col).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32 - 9,
                        col as i32,
                        row_to as i32 % 9,
                        col as i32,
                    ),
                    ghost_path: Some(linear_path_between(
                        row as i32,
                        col as i32,
                        row_to as i32,
                        col as i32,
                    )),
                },
            );
        }

        // Up, without ghosts
        for row in 3..9 {
            let row_to = row - 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new(row_to, col).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32,
                        col as i32,
                        row_to as i32,
                        col as i32,
                    ),
                    ghost_path: None,
                },
            );
        }

        // Up, with ghosts
        for row in 0..3 {
            let row_to = row as i32 - 3;

            let from = Position::new(row, col).unwrap();
            let to = Position::new((row_to + 9) as u8, col).unwrap();

            map.insert(
                (from, to),
                Path {
                    main_path: linear_path_between(
                        row as i32 + 9,
                        col as i32,
                        row_to + 9,
                        col as i32,
                    ),
                    ghost_path: Some(linear_path_between(
                        row as i32, col as i32, row_to, col as i32,
                    )),
                },
            );
        }
    }

    // Rotational
    #[rustfmt::skip]
    let rotational_params = [
        // Northern square, clockwise
        (/*from*/ 0, 3, /*arc from*/ 0, 5, /*arc through*/ 0, 8, /*arc to*/ 3, 8, /*to*/ 3, 8),
        (/*from*/ 0, 4, /*arc from*/ 0, 5, /*arc through*/ 0, 8, /*arc to*/ 3, 8, /*to*/ 4, 8),
        (/*from*/ 0, 5, /*arc from*/ 0, 5, /*arc through*/ 0, 8, /*arc to*/ 3, 8, /*to*/ 5, 8),
        (/*from*/ 1, 3, /*arc from*/ 1, 5, /*arc through*/ 1, 7, /*arc to*/ 3, 7, /*to*/ 3, 7),
        (/*from*/ 1, 4, /*arc from*/ 1, 5, /*arc through*/ 1, 7, /*arc to*/ 3, 7, /*to*/ 4, 7),
        (/*from*/ 1, 5, /*arc from*/ 1, 5, /*arc through*/ 1, 7, /*arc to*/ 3, 7, /*to*/ 5, 7),
        (/*from*/ 2, 3, /*arc from*/ 2, 5, /*arc through*/ 2, 6, /*arc to*/ 3, 6, /*to*/ 3, 6),
        (/*from*/ 2, 4, /*arc from*/ 2, 5, /*arc through*/ 2, 6, /*arc to*/ 3, 6, /*to*/ 4, 6),
        (/*from*/ 2, 5, /*arc from*/ 2, 5, /*arc through*/ 2, 6, /*arc to*/ 3, 6, /*to*/ 5, 6),

        // Northern square, anti-clockwise
        (/*from*/ 0, 3, /*arc from*/ 0, 3, /*arc through*/ 0, 0, /*arc to*/ 3, 0, /*to*/ 5, 0),
        (/*from*/ 0, 4, /*arc from*/ 0, 3, /*arc through*/ 0, 0, /*arc to*/ 3, 0, /*to*/ 4, 0),
        (/*from*/ 0, 5, /*arc from*/ 0, 3, /*arc through*/ 0, 0, /*arc to*/ 3, 0, /*to*/ 3, 0),
        (/*from*/ 1, 3, /*arc from*/ 1, 3, /*arc through*/ 1, 1, /*arc to*/ 3, 1, /*to*/ 5, 1),
        (/*from*/ 1, 4, /*arc from*/ 1, 3, /*arc through*/ 1, 1, /*arc to*/ 3, 1, /*to*/ 4, 1),
        (/*from*/ 1, 5, /*arc from*/ 1, 3, /*arc through*/ 1, 1, /*arc to*/ 3, 1, /*to*/ 3, 1),
        (/*from*/ 2, 3, /*arc from*/ 2, 3, /*arc through*/ 2, 2, /*arc to*/ 3, 2, /*to*/ 5, 2),
        (/*from*/ 2, 4, /*arc from*/ 2, 3, /*arc through*/ 2, 2, /*arc to*/ 3, 2, /*to*/ 4, 2),
        (/*from*/ 2, 5, /*arc from*/ 2, 3, /*arc through*/ 2, 2, /*arc to*/ 3, 2, /*to*/ 3, 2),

        // Southern square, clockwise
        (/*from*/ 6, 3, /*arc from*/ 6, 3, /*arc through*/ 6, 2, /*arc to*/ 5, 2, /*to*/ 3, 2),
        (/*from*/ 6, 4, /*arc from*/ 6, 3, /*arc through*/ 6, 2, /*arc to*/ 5, 2, /*to*/ 4, 2),
        (/*from*/ 6, 5, /*arc from*/ 6, 3, /*arc through*/ 6, 2, /*arc to*/ 5, 2, /*to*/ 5, 2),
        (/*from*/ 7, 3, /*arc from*/ 7, 3, /*arc through*/ 7, 1, /*arc to*/ 5, 1, /*to*/ 3, 1),
        (/*from*/ 7, 4, /*arc from*/ 7, 3, /*arc through*/ 7, 1, /*arc to*/ 5, 1, /*to*/ 4, 1),
        (/*from*/ 7, 5, /*arc from*/ 7, 3, /*arc through*/ 7, 1, /*arc to*/ 5, 1, /*to*/ 5, 1),
        (/*from*/ 8, 3, /*arc from*/ 8, 3, /*arc through*/ 8, 0, /*arc to*/ 5, 0, /*to*/ 3, 0),
        (/*from*/ 8, 4, /*arc from*/ 8, 3, /*arc through*/ 8, 0, /*arc to*/ 5, 0, /*to*/ 4, 0),
        (/*from*/ 8, 5, /*arc from*/ 8, 3, /*arc through*/ 8, 0, /*arc to*/ 5, 0, /*to*/ 5, 0),

        // Southern square, anti-clockwise
        (/*from*/ 6, 3, /*arc from*/ 6, 5, /*arc through*/ 6, 6, /*arc to*/ 5, 6, /*to*/ 5, 6),
        (/*from*/ 6, 4, /*arc from*/ 6, 5, /*arc through*/ 6, 6, /*arc to*/ 5, 6, /*to*/ 4, 6),
        (/*from*/ 6, 5, /*arc from*/ 6, 5, /*arc through*/ 6, 6, /*arc to*/ 5, 6, /*to*/ 3, 6),
        (/*from*/ 7, 3, /*arc from*/ 7, 5, /*arc through*/ 7, 7, /*arc to*/ 5, 7, /*to*/ 5, 7),
        (/*from*/ 7, 4, /*arc from*/ 7, 5, /*arc through*/ 7, 7, /*arc to*/ 5, 7, /*to*/ 4, 7),
        (/*from*/ 7, 5, /*arc from*/ 7, 5, /*arc through*/ 7, 7, /*arc to*/ 5, 7, /*to*/ 3, 7),
        (/*from*/ 8, 3, /*arc from*/ 8, 5, /*arc through*/ 8, 8, /*arc to*/ 5, 8, /*to*/ 5, 8),
        (/*from*/ 8, 4, /*arc from*/ 8, 5, /*arc through*/ 8, 8, /*arc to*/ 5, 8, /*to*/ 4, 8),
        (/*from*/ 8, 5, /*arc from*/ 8, 5, /*arc through*/ 8, 8, /*arc to*/ 5, 8, /*to*/ 3, 8),

        // Western square, clockwise
        (/*from*/ 3, 0, /*arc from*/ 3, 0, /*arc through*/ 0, 0, /*arc to*/ 0, 3, /*to*/ 0, 5),
        (/*from*/ 4, 0, /*arc from*/ 3, 0, /*arc through*/ 0, 0, /*arc to*/ 0, 3, /*to*/ 0, 4),
        (/*from*/ 5, 0, /*arc from*/ 3, 0, /*arc through*/ 0, 0, /*arc to*/ 0, 3, /*to*/ 0, 3),
        (/*from*/ 3, 1, /*arc from*/ 3, 1, /*arc through*/ 1, 1, /*arc to*/ 1, 3, /*to*/ 1, 5),
        (/*from*/ 4, 1, /*arc from*/ 3, 1, /*arc through*/ 1, 1, /*arc to*/ 1, 3, /*to*/ 1, 4),
        (/*from*/ 5, 1, /*arc from*/ 3, 1, /*arc through*/ 1, 1, /*arc to*/ 1, 3, /*to*/ 1, 3),
        (/*from*/ 3, 2, /*arc from*/ 3, 2, /*arc through*/ 2, 2, /*arc to*/ 2, 3, /*to*/ 2, 5),
        (/*from*/ 4, 2, /*arc from*/ 3, 2, /*arc through*/ 2, 2, /*arc to*/ 2, 3, /*to*/ 2, 4),
        (/*from*/ 5, 2, /*arc from*/ 3, 2, /*arc through*/ 2, 2, /*arc to*/ 2, 3, /*to*/ 2, 3),

        // Western square, anti-clockwise
        (/*from*/ 3, 0, /*arc from*/ 5, 0, /*arc through*/ 8, 0, /*arc to*/ 8, 3, /*to*/ 8, 3),
        (/*from*/ 4, 0, /*arc from*/ 5, 0, /*arc through*/ 8, 0, /*arc to*/ 8, 3, /*to*/ 8, 4),
        (/*from*/ 5, 0, /*arc from*/ 5, 0, /*arc through*/ 8, 0, /*arc to*/ 8, 3, /*to*/ 8, 5),
        (/*from*/ 3, 1, /*arc from*/ 5, 1, /*arc through*/ 7, 1, /*arc to*/ 7, 3, /*to*/ 7, 3),
        (/*from*/ 4, 1, /*arc from*/ 5, 1, /*arc through*/ 7, 1, /*arc to*/ 7, 3, /*to*/ 7, 4),
        (/*from*/ 5, 1, /*arc from*/ 5, 1, /*arc through*/ 7, 1, /*arc to*/ 7, 3, /*to*/ 7, 5),
        (/*from*/ 3, 2, /*arc from*/ 5, 2, /*arc through*/ 6, 2, /*arc to*/ 6, 3, /*to*/ 6, 3),
        (/*from*/ 4, 2, /*arc from*/ 5, 2, /*arc through*/ 6, 2, /*arc to*/ 6, 3, /*to*/ 6, 4),
        (/*from*/ 5, 2, /*arc from*/ 5, 2, /*arc through*/ 6, 2, /*arc to*/ 6, 3, /*to*/ 6, 5),

        // Eastern square, clockwise
        (/*from*/ 3, 6, /*arc from*/ 6, 6, /*arc through*/ 6, 6, /*arc to*/ 6, 5, /*to*/ 6, 5),
        (/*from*/ 4, 6, /*arc from*/ 6, 6, /*arc through*/ 6, 6, /*arc to*/ 6, 5, /*to*/ 6, 4),
        (/*from*/ 5, 6, /*arc from*/ 6, 6, /*arc through*/ 6, 6, /*arc to*/ 6, 5, /*to*/ 6, 3),
        (/*from*/ 3, 7, /*arc from*/ 6, 7, /*arc through*/ 7, 7, /*arc to*/ 7, 5, /*to*/ 7, 5),
        (/*from*/ 4, 7, /*arc from*/ 6, 7, /*arc through*/ 7, 7, /*arc to*/ 7, 5, /*to*/ 7, 4),
        (/*from*/ 5, 7, /*arc from*/ 6, 7, /*arc through*/ 7, 7, /*arc to*/ 7, 5, /*to*/ 7, 3),
        (/*from*/ 3, 8, /*arc from*/ 6, 8, /*arc through*/ 8, 8, /*arc to*/ 8, 5, /*to*/ 8, 5),
        (/*from*/ 4, 8, /*arc from*/ 6, 8, /*arc through*/ 8, 8, /*arc to*/ 8, 5, /*to*/ 8, 4),
        (/*from*/ 5, 8, /*arc from*/ 6, 8, /*arc through*/ 8, 8, /*arc to*/ 8, 5, /*to*/ 8, 3),

        // Eastern square, anti-clockwise
        (/*from*/ 3, 6, /*arc from*/ 3, 6, /*arc through*/ 2, 6, /*arc to*/ 2, 5, /*to*/ 2, 3),
        (/*from*/ 4, 6, /*arc from*/ 3, 6, /*arc through*/ 2, 6, /*arc to*/ 2, 5, /*to*/ 2, 4),
        (/*from*/ 5, 6, /*arc from*/ 3, 6, /*arc through*/ 2, 6, /*arc to*/ 2, 5, /*to*/ 2, 5),
        (/*from*/ 3, 7, /*arc from*/ 3, 7, /*arc through*/ 1, 7, /*arc to*/ 1, 5, /*to*/ 1, 3),
        (/*from*/ 4, 7, /*arc from*/ 3, 7, /*arc through*/ 1, 7, /*arc to*/ 1, 5, /*to*/ 1, 4),
        (/*from*/ 5, 7, /*arc from*/ 3, 7, /*arc through*/ 1, 7, /*arc to*/ 1, 5, /*to*/ 1, 5),
        (/*from*/ 3, 8, /*arc from*/ 3, 8, /*arc through*/ 0, 8, /*arc to*/ 0, 5, /*to*/ 0, 3),
        (/*from*/ 4, 8, /*arc from*/ 3, 8, /*arc through*/ 0, 8, /*arc to*/ 0, 5, /*to*/ 0, 4),
        (/*from*/ 5, 8, /*arc from*/ 3, 8, /*arc through*/ 0, 8, /*arc to*/ 0, 5, /*to*/ 0, 5),
    ];

    for (
        row_from,
        col_from,
        row_arc_from,
        col_arc_from,
        row_arc_through,
        col_arc_through,
        row_arc_to,
        col_arc_to,
        row_to,
        col_to,
    ) in rotational_params
    {
        let from = Position::new(row_from, col_from).unwrap();
        let to = Position::new(row_to, col_to).unwrap();

        // TODO Make sure that arc start and stop get a half piece offsest, so
        // that they fit the paths perfectly.
        let (x_from, y_from) = piece_location(row_from as i32, col_from as i32);
        let (x_arc_from, y_arc_from) = piece_location(row_arc_from, col_arc_from);
        let (x_arc_through, y_arc_through) = piece_location(row_arc_through, col_arc_through);
        let (x_arc_to, y_arc_to) = piece_location(row_arc_to, col_arc_to);
        let (x_to, y_to) = piece_location(row_to as i32, col_to as i32);

        let path = SubpathNoId::from_beziers(
            &[
                Bezier::from_linear_coordinates(
                    x_from as f64,
                    y_from as f64,
                    x_arc_from as f64,
                    y_arc_from as f64,
                ),
                Bezier::from_quadratic_coordinates(
                    x_arc_from as f64,
                    y_arc_from as f64,
                    x_arc_through as f64,
                    y_arc_through as f64,
                    x_arc_to as f64,
                    y_arc_to as f64,
                ),
                Bezier::from_linear_coordinates(
                    x_arc_to as f64,
                    y_arc_to as f64,
                    x_to as f64,
                    y_to as f64,
                ),
            ],
            false,
        );

        map.insert(
            (from, to),
            Path {
                main_path: path,
                ghost_path: None,
            },
        );
    }

    map
}

pub fn initialize_pieces<'a>(assets: &'a Assets, paths: &'a PathMap) -> PieceCollection<'a> {
    let mut pieces = vec![];

    let piece_groups = vec![
        (&assets.img_piece_green, 0, 3),
        (&assets.img_piece_purple, 3, 0),
        (&assets.img_piece_yellow, 3, 3),
        (&assets.img_piece_blue, 3, 6),
        (&assets.img_piece_red, 6, 3),
    ];

    for (texture, row_offset, col_offset) in piece_groups {
        for row in row_offset..row_offset + 3 {
            for col in col_offset..col_offset + 3 {
                let (x, y) = piece_location(row as i32, col as i32);
                pieces.push(Piece::new(texture, Position::new(row, col).unwrap(), x, y));
            }
        }
    }

    PieceCollection {
        pieces,
        path_map: paths,
    }
}
