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

#[non_exhaustive]
pub enum InputEvent {
    MouseMoved { x: f32, y: f32 },
    MousePressed { x: f32, y: f32 },
    MouseReleased,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlId {
    HorizontalLeft(u8),
    HorizontalRight(u8),
    VerticalUp(u8),
    VerticalDown(u8),
    RotateClockwise(u8),
    RotateAnticlockwise(u8),
}

#[derive(Debug)]
pub enum ControlEvent {
    Pressed(ControlId),
}
