mod button;
pub use button::Button;

mod sprite;
pub use sprite::Sprite;

pub mod initialization;

#[non_exhaustive]
pub enum InputEvent {
    MouseMoved { x: f32, y: f32 },
    MousePressed { x: f32, y: f32 },
    MouseReleased,
}

pub enum ControlId {
    LinearArrow,
}

pub enum ControlEvent {
    Pressed(ControlId),
}

pub trait Control {
    fn draw(&self);
    fn handle_event(&mut self, _event: &InputEvent, _generated_events: &mut Vec<ControlEvent>) {}
}
