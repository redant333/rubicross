use crate::{initialization::PathMap, Control, Piece};

use super::position;

pub struct PieceCollection<'a> {
    pub path_map: &'a PathMap,
    pub pieces: Vec<Piece<'a>>,
}

pub enum Manipulation {
    RotateClockwise(u8),
    RotateAnticlockwise(u8),
    SlideLeft(u8),
    SlideRight(u8),
    SlideUp(u8),
    SlideDown(u8),
}

impl<'a> PieceCollection<'a> {
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

    pub fn update(&mut self) {
        for piece in &mut self.pieces {
            piece.update();
        }
    }
}

impl<'a> Control for PieceCollection<'a> {
    fn draw(&self) {
        for piece in &self.pieces {
            piece.draw();
        }
    }
}
