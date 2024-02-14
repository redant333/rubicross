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

pub mod initialization;

#[non_exhaustive]
pub enum InputEvent {
    MouseMoved { x: f32, y: f32 },
    MousePressed { x: f32, y: f32 },
    MouseReleased,
}

#[derive(Debug, Clone)]
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

pub trait Control {
    fn draw(&self);
    fn handle_event(&mut self, _event: &InputEvent, _generated_events: &mut Vec<ControlEvent>) {}
}
