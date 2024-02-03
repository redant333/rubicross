mod button;
use bezier_rs::Identifier;
use bezier_rs::Subpath;
pub use button::Button;

mod pieces;
pub use pieces::collection::PieceCollection;
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
    RotationalArrow,
}

#[derive(Debug)]
pub enum ControlEvent {
    Pressed(ControlId),
}

pub trait Control {
    fn draw(&self);
    fn handle_event(&mut self, _event: &InputEvent, _generated_events: &mut Vec<ControlEvent>) {}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EmptyId;

impl Identifier for EmptyId {
    fn new() -> Self {
        Self
    }
}

type Path = Subpath<EmptyId>;
