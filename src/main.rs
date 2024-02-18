use macroquad::prelude::*;
use rubicross::initialization::initialize_solved_markers;
use rubicross::solved_marker::SolvedMarker;
use rubicross::Manipulation;
use rubicross::{
    initialization::{initialize_buttons, initialize_paths, initialize_pieces, load_assets},
    Button, Control, ControlEvent, ControlId, InputEvent, PieceCollection,
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

fn broadcast_input_events(controls: &mut [Button], new_events: &mut Vec<ControlEvent>) {
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

fn handle_events(
    new_events: &[ControlEvent],
    pieces: &mut PieceCollection,
    markers: &mut [SolvedMarker],
) {
    const ANIMATION_LENGTH: f64 = 0.35;
    for event in new_events.iter() {
        match event {
            ControlEvent::Pressed(ControlId::HorizontalRight(row)) => {
                pieces.apply_manipulation(Manipulation::SlideRight(*row), ANIMATION_LENGTH)
            }
            ControlEvent::Pressed(ControlId::HorizontalLeft(row)) => {
                pieces.apply_manipulation(Manipulation::SlideLeft(*row), ANIMATION_LENGTH)
            }
            ControlEvent::Pressed(ControlId::VerticalUp(col)) => {
                pieces.apply_manipulation(Manipulation::SlideUp(*col), ANIMATION_LENGTH)
            }
            ControlEvent::Pressed(ControlId::VerticalDown(col)) => {
                pieces.apply_manipulation(Manipulation::SlideDown(*col), ANIMATION_LENGTH)
            }
            ControlEvent::Pressed(ControlId::RotateClockwise(ring)) => {
                pieces.apply_manipulation(Manipulation::RotateClockwise(*ring), ANIMATION_LENGTH)
            }
            ControlEvent::Pressed(ControlId::RotateAnticlockwise(ring)) => pieces
                .apply_manipulation(Manipulation::RotateAnticlockwise(*ring), ANIMATION_LENGTH),
            ControlEvent::SquareStatusChanged(square, solved) => {
                markers
                    .iter_mut()
                    .find(|marker| marker.square() == square)
                    .unwrap()
                    .set_visible(*solved);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = load_assets().await;
    let paths = initialize_paths();
    let mut buttons = initialize_buttons(&assets);
    let mut solved_markers = initialize_solved_markers(&assets);
    let mut pieces = initialize_pieces(&assets, &paths);

    loop {
        let mut new_events = vec![];
        broadcast_input_events(&mut buttons, &mut new_events);
        pieces.update(&mut new_events);

        handle_events(&new_events, &mut pieces, &mut solved_markers);

        // Draw the background
        draw_texture(&assets.img_board, 0., 0., WHITE);

        // Draw solved markers
        for marker in &solved_markers {
            marker.draw();
        }

        // Draw the rotational buttons
        let rotational_buttons = buttons.iter_mut().filter(|button| {
            matches!(
                button.id(),
                ControlId::RotateClockwise(_) | ControlId::RotateAnticlockwise(_)
            )
        });

        for drawable in rotational_buttons {
            drawable.draw();
        }

        // Draw the pieces
        pieces.draw();

        // Draw surroundings and cover pieces outside the board
        draw_texture(&assets.img_surroundings, 0., 0., WHITE);

        // Draw the linear buttons
        let linear_buttons = buttons.iter_mut().filter(|button| {
            !matches!(
                button.id(),
                ControlId::RotateClockwise(_) | ControlId::RotateAnticlockwise(_)
            )
        });

        for drawable in linear_buttons {
            drawable.draw();
        }

        next_frame().await
    }
}
