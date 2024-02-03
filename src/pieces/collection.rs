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
    pub fn apply_manipulation(&mut self, manipulation: Manipulation) {
        let (filter, piece_manipulation) = match manipulation {
            Manipulation::RotateClockwise(_) => todo!(),
            Manipulation::RotateAnticlockwise(_) => todo!(),
            Manipulation::SlideLeft(_) => todo!(),
            Manipulation::SlideRight(row) => {
                let filter = move |piece: &&mut Piece<'a>| piece.position().row() == row;
                let piece_manipulation = position::Manipulation::SlideRight;
                (filter, piece_manipulation)
            }
            Manipulation::SlideUp(_) => todo!(),
            Manipulation::SlideDown(_) => todo!(),
        };

        let pieces = self.pieces.iter_mut().filter(filter);

        for piece in pieces {
            let position_before = *piece.position();
            piece
                .position_mut()
                .apply_manipulation(piece_manipulation)
                .unwrap();
            let position_after = *piece.position();

            if let Some(paths) = self.path_map.get(&(position_before, position_after)) {
                println!(
                    "Found path {:?} for {:?}, {:?}",
                    paths, position_before, position_after
                )
            }
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
