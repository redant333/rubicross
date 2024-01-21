mod button;
pub use button::Button;

mod sprite;
pub use sprite::Sprite;

pub mod initialization;

#[non_exhaustive]
pub enum Event {
    MouseMoved { x: f32, y: f32 },
    MousePressed { x: f32, y: f32 },
    MouseReleased,
}

pub trait Control {
    fn draw(&self);
    fn handle_event(&mut self, _event: &Event) {}
}
