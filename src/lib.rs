mod button;
pub use button::Button;

mod piece;
pub use piece::Piece;

pub mod initialization;

mod piece_logic;
pub use piece_logic::PieceError;
pub use piece_logic::Position;

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
