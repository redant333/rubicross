use crate::{initialization::PathMap, Control, ControlEvent, Piece};

use super::position;

pub struct PieceCollection<'a> {
    pub path_map: &'a PathMap,
    pub pieces: Vec<Piece<'a>>,

    south_solved: bool,
    north_solved: bool,
    center_solved: bool,
    east_solved: bool,
    west_solved: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum Manipulation {
    RotateClockwise(u8),
    RotateAnticlockwise(u8),
    SlideLeft(u8),
    SlideRight(u8),
    SlideUp(u8),
    SlideDown(u8),
}

impl<'a> PieceCollection<'a> {
    pub fn new(path_map: &'a PathMap, pieces: Vec<Piece<'a>>) -> Self {
        Self {
            path_map,
            pieces,
            south_solved: false,
            north_solved: false,
            center_solved: false,
            east_solved: false,
            west_solved: false,
        }
    }

    pub fn apply_manipulation(&mut self, manipulation: Manipulation, animation_length: f64) {
        let (pieces, piece_manipulation): (Vec<&mut Piece>, position::Manipulation) =
            match manipulation {
                Manipulation::RotateClockwise(ring) => {
                    let filter =
                        move |piece: &&mut Piece<'a>| piece.position().ring() == Some(ring);
                    let pieces = self.pieces.iter_mut().filter(filter).collect();
                    let piece_manipulation = position::Manipulation::RotateClockwise;
                    (pieces, piece_manipulation)
                }
                Manipulation::RotateAnticlockwise(ring) => {
                    let filter =
                        move |piece: &&mut Piece<'a>| piece.position().ring() == Some(ring);
                    let pieces = self.pieces.iter_mut().filter(filter).collect();
                    let piece_manipulation = position::Manipulation::RotateAnticlockwise;
                    (pieces, piece_manipulation)
                }
                Manipulation::SlideLeft(row) => {
                    let filter = move |piece: &&mut Piece<'a>| piece.position().row() == row;
                    let pieces = self.pieces.iter_mut().filter(filter).collect();
                    let piece_manipulation = position::Manipulation::SlideLeft;
                    (pieces, piece_manipulation)
                }
                Manipulation::SlideRight(row) => {
                    let filter = move |piece: &&mut Piece<'a>| piece.position().row() == row;
                    let pieces = self.pieces.iter_mut().filter(filter).collect();
                    let piece_manipulation = position::Manipulation::SlideRight;
                    (pieces, piece_manipulation)
                }
                Manipulation::SlideUp(col) => {
                    let filter = move |piece: &&mut Piece<'a>| piece.position().col() == col;
                    let pieces = self.pieces.iter_mut().filter(filter).collect();
                    let piece_manipulation = position::Manipulation::SlideUp;
                    (pieces, piece_manipulation)
                }
                Manipulation::SlideDown(col) => {
                    let filter = move |piece: &&mut Piece<'a>| piece.position().col() == col;
                    let pieces = self.pieces.iter_mut().filter(filter).collect();
                    let piece_manipulation = position::Manipulation::SlideDown;
                    (pieces, piece_manipulation)
                }
            };

        for piece in pieces {
            let position_before = *piece.position();
            piece
                .position_mut()
                .apply_manipulation(piece_manipulation)
                .unwrap();
            let position_after = *piece.position();

            if let Some(paths) = self.path_map.get(&(position_before, position_after)) {
                piece.start_moving_along(paths, animation_length)
            } else {
                println!(
                    "Missing path for {:?} -> {:?}",
                    position_before, position_after
                );
            }
        }
    }

    pub fn update(&mut self, generated_events: &mut Vec<ControlEvent>) {
        // Update the pieces
        for piece in &mut self.pieces {
            piece.update();
        }

        // Check if solved status changed
        use position::Square::*;
        for (square, store_var) in [
            (North, &mut self.north_solved),
            (East, &mut self.east_solved),
            (Center, &mut self.center_solved),
            (West, &mut self.west_solved),
            (South, &mut self.south_solved),
        ] {
            let mut square_pieces = self
                .pieces
                .iter()
                .filter(|piece| piece.position().square() == square);

            let first_piece = square_pieces.next().unwrap();
            let square_solved = square_pieces.all(|piece| first_piece.has_same_color_as(piece));

            if square_solved != *store_var {
                generated_events.push(ControlEvent::SquareStatusChanged(square, square_solved));
                *store_var = square_solved;
            }
        }
    }

    pub fn is_animating(&self) -> bool {
        self.pieces.iter().any(|piece| piece.is_animating())
    }

    pub fn is_solved(&self) -> bool {
        self.east_solved
            && self.west_solved
            && self.north_solved
            && self.south_solved
            && self.center_solved
    }
}

impl<'a> Control for PieceCollection<'a> {
    fn draw(&self) {
        for piece in &self.pieces {
            piece.draw();
        }
    }
}
