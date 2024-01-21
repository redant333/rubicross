use macroquad::prelude::*;
use rubicross::{
    initialization::{initialize_controls, load_assets},
    Event,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Rubicross".into(),
        window_height: 500,
        window_width: 500,
        window_resizable: true,
        ..Default::default()
    }
}

macro_rules! broadcast_event {
    ( $controls:expr, $condition:expr, $event:expr ) => {
        if $condition {
            for control in &mut $controls {
                control.handle_event(&$event);
            }
        }
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = load_assets().await;
    let mut controls = initialize_controls(&assets);

    loop {
        clear_background(color_u8!(0xc5, 0xba, 0xaf, 0xff));
        let (x, y) = mouse_position();

        broadcast_event!(
            controls,
            mouse_delta_position() != Vec2::ZERO,
            Event::MouseMoved { x, y }
        );

        broadcast_event!(
            controls,
            is_mouse_button_pressed(MouseButton::Left),
            Event::MousePressed { x, y }
        );

        broadcast_event!(
            controls,
            is_mouse_button_released(MouseButton::Left),
            Event::MouseReleased
        );

        for drawable in &controls {
            drawable.draw();
        }

        next_frame().await
    }
}
