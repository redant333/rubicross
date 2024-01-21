use macroquad::prelude::*;
use rubicross::{
    initialization::{initialize_controls, load_assets},
    Control, Event,
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

fn broadcast_input_events(controls: &mut [Box<dyn Control + '_>]) {
    let (x, y) = mouse_position();
    let mut events = vec![];

    if mouse_delta_position() != Vec2::ZERO {
        events.push(Event::MouseMoved { x, y });
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        events.push(Event::MousePressed { x, y });
    }

    if is_mouse_button_released(MouseButton::Left) {
        events.push(Event::MouseReleased);
    }

    for control in controls.iter_mut() {
        for event in &events {
            control.handle_event(event);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = load_assets().await;
    let mut controls = initialize_controls(&assets);

    loop {
        broadcast_input_events(&mut controls);

        clear_background(color_u8!(0xc5, 0xba, 0xaf, 0xff));
        for drawable in &controls {
            drawable.draw();
        }

        next_frame().await
    }
}
