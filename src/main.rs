use macroquad::prelude::*;
use rubicross::{
    initialization::{initialize_controls, load_assets},
    Control, ControlEvent, InputEvent,
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

fn broadcast_input_events(
    controls: &mut [Box<dyn Control + '_>],
    new_events: &mut Vec<ControlEvent>,
) {
    let (x, y) = mouse_position();
    let mut events = vec![];

    if mouse_delta_position() != Vec2::ZERO {
        events.push(InputEvent::MouseMoved { x, y });
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        events.push(InputEvent::MousePressed { x, y });
    }

    if is_mouse_button_released(MouseButton::Left) {
        events.push(InputEvent::MouseReleased);
    }

    for control in controls.iter_mut() {
        for event in &events {
            control.handle_event(event, new_events);
        }
    }
}

fn handle_events(new_events: &[ControlEvent]) {
    for event in new_events.iter() {
        println!("{:?}", event);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = load_assets().await;
    let mut controls = initialize_controls(&assets);

    loop {
        let mut new_events = vec![];
        broadcast_input_events(&mut controls, &mut new_events);
        handle_events(&new_events);

        clear_background(color_u8!(0xc5, 0xba, 0xaf, 0xff));
        for drawable in &controls {
            drawable.draw();
        }

        next_frame().await
    }
}
