use macroquad::prelude::*;
use rand::ChooseRandom;

use crate::{
    initialization::Assets, solved_marker::SolvedMarker, Button, Control, ControlEvent, ControlId,
    InputEvent, Manipulation, PieceCollection,
};

pub struct Game<'a> {
    pub assets: &'a Assets,
    pub buttons: Vec<Button<'a>>,
    pub solved_markers: Vec<SolvedMarker<'a>>,
    pub pieces: PieceCollection<'a>,
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

fn generate_shuffle_manipulations(count: usize) -> Vec<Manipulation> {
    use Manipulation::*;

    let all_manipulations = vec![
        RotateClockwise(0),
        RotateClockwise(1),
        RotateClockwise(2),
        RotateAnticlockwise(0),
        RotateAnticlockwise(1),
        RotateAnticlockwise(2),
        SlideLeft(3),
        SlideLeft(4),
        SlideLeft(5),
        SlideRight(3),
        SlideRight(4),
        SlideRight(5),
        SlideUp(3),
        SlideUp(4),
        SlideUp(5),
        SlideDown(3),
        SlideDown(4),
        SlideDown(5),
    ];

    (0..count)
        .map(|_| all_manipulations.choose().unwrap())
        .cloned()
        .collect()
}

impl<'a> Game<'a> {
    pub async fn perform_shuffle(&mut self, shuffle_count: usize, animation_length: f64) {
        let shuffle_manipulations = generate_shuffle_manipulations(shuffle_count);
        let mut shuffle_manipulations = shuffle_manipulations.into_iter();

        loop {
            if !self.pieces.is_animating() {
                match shuffle_manipulations.next() {
                    Some(manipulation) => self
                        .pieces
                        .apply_manipulation(manipulation, animation_length),
                    None => break,
                }
            }

            let mut dummy = vec![];
            self.pieces.update(&mut dummy);
            self.draw_all();
            next_frame().await
        }
    }

    pub async fn run_main_loop(&mut self) {
        loop {
            let mut new_events = vec![];
            broadcast_input_events(&mut self.buttons, &mut new_events);
            self.pieces.update(&mut new_events);

            handle_events(&new_events, &mut self.pieces, &mut self.solved_markers);

            self.draw_all();
            next_frame().await
        }
    }

    fn draw_all(&self) {
        // Draw the background
        draw_texture(&self.assets.img_board, 0., 0., WHITE);

        // Draw solved markers
        for marker in &self.solved_markers {
            marker.draw();
        }

        // Draw the rotational buttons
        let rotational_buttons = self.buttons.iter().filter(|button| {
            matches!(
                button.id(),
                ControlId::RotateClockwise(_) | ControlId::RotateAnticlockwise(_)
            )
        });

        for drawable in rotational_buttons {
            drawable.draw();
        }

        // Draw the pieces
        self.pieces.draw();

        // Draw surroundings and cover pieces outside the board
        draw_texture(&self.assets.img_surroundings, 0., 0., WHITE);

        // Draw the linear buttons
        let linear_buttons = self.buttons.iter().filter(|button| {
            !matches!(
                button.id(),
                ControlId::RotateClockwise(_) | ControlId::RotateAnticlockwise(_)
            )
        });

        for drawable in linear_buttons {
            drawable.draw();
        }
    }
}
