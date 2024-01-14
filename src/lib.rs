mod button;
pub use button::Button;

mod sprite;
pub use sprite::Sprite;

pub mod initialization;

pub trait Draw {
    fn draw(&self);
}
