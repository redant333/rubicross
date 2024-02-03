use crate::{Control, Piece};

pub struct PieceCollection<'a> {
    pub pieces: Vec<Piece<'a>>,
}

impl<'a> Control for PieceCollection<'a> {
    fn draw(&self) {
        for piece in &self.pieces {
            piece.draw();
        }
    }
}
