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

#[macroquad::main(window_conf)]
async fn main() {
    let assets = load_assets().await;
    let mut controls = initialize_controls(&assets);

    loop {
        clear_background(color_u8!(0xc5, 0xba, 0xaf, 0xff));
        let (mouse_x, mouse_y) = mouse_position();

        if mouse_delta_position() != Vec2::ZERO {
            for control in &mut controls {
                control.handle_event(&Event::MouseMoved {
                    x: mouse_x,
                    y: mouse_y,
                });
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let event = Event::MousePressed {
                x: mouse_x,
                y: mouse_y,
            };

            for control in &mut controls {
                control.handle_event(&event);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            for control in &mut controls {
                control.handle_event(&Event::MouseReleased);
            }
        }

        for drawable in &controls {
            drawable.draw();
        }

        next_frame().await
    }
}
