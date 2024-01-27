use std::{error::Error, fmt::Display};

const DIMENSION: u8 = 9;
const SQUARE_SIZE: u8 = DIMENSION / 3;

#[derive(Debug)]
#[non_exhaustive]
pub enum PieceError {
    InvalidRowCol,
    CantRotate,
    CantSlide,
}

impl Display for PieceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceError::InvalidRowCol => write!(f, "Invalid combination of row and column"),
            PieceError::CantRotate => write!(f, "Trying to rotate a piece in the center square"),
            PieceError::CantSlide => write!(f, "Trying to slide a piece in invalid direction"),
        }
    }
}

impl Error for PieceError {}

pub struct Position {
    row: u8,
    col: u8,
}

pub enum RotationDirection {
    Clockwise,
    Anticlockwise,
}

pub enum SlideDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Position {
    pub fn new(row: u8, col: u8) -> Result<Self, PieceError> {
        let ret = Self { row, col };

        if ret.is_row_in_middle() || ret.is_col_in_middle() {
            Ok(ret)
        } else {
            Err(PieceError::InvalidRowCol)
        }
    }

    pub fn rotate(&mut self, direction: RotationDirection) -> Result<(), PieceError> {
        if self.is_row_in_middle() && self.is_col_in_middle() {
            return Err(PieceError::CantRotate);
        }

        let flip_row_and_swap = |row: u8, col: u8| (col, DIMENSION - 1 - row);
        let flip_col_and_swap = |row: u8, col: u8| (DIMENSION - 1 - col, row);

        let (new_row, new_col) = match direction {
            RotationDirection::Clockwise => flip_row_and_swap(self.row, self.col),
            RotationDirection::Anticlockwise => flip_col_and_swap(self.row, self.col),
        };

        self.row = new_row;
        self.col = new_col;

        Ok(())
    }

    pub fn slide(&mut self, direction: SlideDirection) -> Result<(), PieceError> {
        use SlideDirection::*;

        match direction {
            Left | Right if !self.is_row_in_middle() => return Err(PieceError::CantSlide),
            Up | Down if !self.is_col_in_middle() => return Err(PieceError::CantSlide),
            _ => (),
        }

        match direction {
            Left => self.col = (DIMENSION + self.col - SQUARE_SIZE) % DIMENSION,
            Right => self.col = (self.col + SQUARE_SIZE) % DIMENSION,
            Up => self.row = (DIMENSION + self.row - SQUARE_SIZE) % DIMENSION,
            Down => self.row = (self.row + SQUARE_SIZE) % DIMENSION,
        }

        Ok(())
    }

    fn is_row_in_middle(&self) -> bool {
        self.row >= SQUARE_SIZE && self.row < 2 * SQUARE_SIZE
    }

    fn is_col_in_middle(&self) -> bool {
        self.col >= SQUARE_SIZE && self.col < 2 * SQUARE_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0, 0, false ; "upper left corner")]
    #[test_case(8, 8, false; "bottom right corner")]
    #[test_case(8, 0, false; "bottom left corner")]
    #[test_case(0, 8, false; "upper right corner")]
    #[test_case(0, 2, false; "upper left square")]
    #[test_case(6, 2, false; "bottom left square")]
    #[test_case(7, 8, false; "bottom right square")]
    #[test_case(2, 8, false; "upper right square")]
    #[test_case(0, 3, true; "northern square")]
    #[test_case(3, 0, true; "western square")]
    #[test_case(4, 3, true; "eastern square")]
    #[test_case(5, 8, true; "southern square")]
    #[test_case(3, 3, true; "center square")]

    fn new_returns_expected_value(row: u8, col: u8, expected: bool) {
        let position = Position::new(row, col);

        assert_eq!(position.is_ok(), expected);
    }

    #[test]
    fn rotate_fails_for_center_square() {
        let position = Position::new(4, 4);
        let ret = position.unwrap().rotate(RotationDirection::Clockwise);

        assert!(matches!(ret.err().unwrap(), PieceError::CantRotate));
    }

    #[test_case(0, 3, RotationDirection::Clockwise, 3, 8; "northern square, outer ring, clockwise")]
    #[test_case(1, 4, RotationDirection::Clockwise, 4, 7; "northern square, middle ring, clockwise")]
    #[test_case(2, 5, RotationDirection::Clockwise, 5, 6; "northern square, innner ring, clockwise")]
    #[test_case(5, 8, RotationDirection::Clockwise, 8, 3; "eastern square, outer ring, clockwise")]
    #[test_case(4, 7, RotationDirection::Clockwise, 7, 4; "eastern square, middle ring, clockwise")]
    #[test_case(3, 6, RotationDirection::Clockwise, 6, 5; "eastern square, innner ring, clockwise")]
    #[test_case(8, 3, RotationDirection::Clockwise, 3, 0; "southern square, outer ring, clockwise")]
    #[test_case(7, 4, RotationDirection::Clockwise, 4, 1; "southern square, middle ring, clockwise")]
    #[test_case(6, 5, RotationDirection::Clockwise, 5, 2; "southern square, innner ring, clockwise")]
    #[test_case(3, 0, RotationDirection::Clockwise, 0, 5; "western square, outer ring, clockwise")]
    #[test_case(4, 1, RotationDirection::Clockwise, 1, 4; "western square, middle ring, clockwise")]
    #[test_case(5, 2, RotationDirection::Clockwise, 2, 3; "western square, innner ring, clockwise")]
    #[test_case(3, 8, RotationDirection::Anticlockwise, 0, 3; "northern square, outer ring, anti-clockwise")]
    #[test_case(4, 7, RotationDirection::Anticlockwise, 1, 4; "northern square, middle ring, anti-clockwise")]
    #[test_case(5, 6, RotationDirection::Anticlockwise, 2, 5; "northern square, innner ring, anti-clockwise")]
    #[test_case(8, 3, RotationDirection::Anticlockwise, 5, 8; "eastern square, outer ring, anti-clockwise")]
    #[test_case(7, 4, RotationDirection::Anticlockwise, 4, 7; "eastern square, middle ring, anti-clockwise")]
    #[test_case(6, 5, RotationDirection::Anticlockwise, 3, 6; "eastern square, innner ring, anti-clockwise")]
    #[test_case(3, 0, RotationDirection::Anticlockwise, 8, 3; "southern square, outer ring, anti-clockwise")]
    #[test_case(4, 1, RotationDirection::Anticlockwise, 7, 4; "southern square, middle ring, anti-clockwise")]
    #[test_case(5, 2, RotationDirection::Anticlockwise, 6, 5; "southern square, innner ring, anti-clockwise")]
    #[test_case(0, 5, RotationDirection::Anticlockwise, 3, 0; "western square, outer ring, anti-clockwise")]
    #[test_case(1, 4, RotationDirection::Anticlockwise, 4, 1; "western square, middle ring, anti-clockwise")]
    #[test_case(2, 3, RotationDirection::Anticlockwise, 5, 2; "western square, innner ring, anti-clockwise")]
    fn rotate_returns_expected_value(
        row: u8,
        col: u8,
        direction: RotationDirection,
        expected_row: u8,
        expected_col: u8,
    ) {
        let mut position = Position::new(row, col).unwrap();
        position.rotate(direction).unwrap();

        assert_eq!(position.row, expected_row);
        assert_eq!(position.col, expected_col);
    }

    #[test_case(3, 0, SlideDirection::Up; "western square, up")]
    #[test_case(4, 1, SlideDirection::Down; "western square, down")]
    #[test_case(3, 6, SlideDirection::Up; "eastern square, up")]
    #[test_case(4, 7, SlideDirection::Down; "eastern square, down")]
    #[test_case(0, 3, SlideDirection::Left; "northern square, left")]
    #[test_case(1, 4, SlideDirection::Right; "northern square, right")]
    #[test_case(6, 3, SlideDirection::Left; "southern square, left")]
    #[test_case(7, 4, SlideDirection::Right; "southern square, right")]
    fn sliding_in_invalid_direction_fails(row: u8, col: u8, direction: SlideDirection) {
        let mut position = Position::new(row, col).unwrap();

        let result = position.slide(direction);

        assert!(matches!(result, Err(PieceError::CantSlide)));
    }

    #[test_case(1, 4, SlideDirection::Down, 4, 4; "northern square, down")]
    #[test_case(1, 4, SlideDirection::Up, 7, 4; "northern square, up")]
    #[test_case(4, 7, SlideDirection::Left, 4, 4; "eastern square, left")]
    #[test_case(4, 7, SlideDirection::Right, 4, 1; "eastern square, right")]
    #[test_case(7, 4, SlideDirection::Down, 1, 4; "southern square, down")]
    #[test_case(7, 4, SlideDirection::Up, 4, 4; "southern square, up")]
    #[test_case(4, 1, SlideDirection::Left, 4, 7; "western square, left")]
    #[test_case(4, 1, SlideDirection::Right, 4, 4; "western square, right")]
    fn sliding_in_valid_direction_returns_expected_result(
        row:u8,
        col:u8,
        direction: SlideDirection,
        expected_row:u8,
        expected_col:u8
    ) {
        let mut position = Position::new(row, col).unwrap();

        position.slide(direction).unwrap();

        assert_eq!(position.row, expected_row);
        assert_eq!(position.col, expected_col);
    }
}
