use macroquad::prelude::*;
use miniquad::date::now;
use rand::rand;

use crate::{
    button::{ButtonEvent, ButtonId, MouseEvent},
    initialization::Assets,
    solved_marker::SolvedMarker,
    Button, Manipulation, PieceCollection,
};

pub struct Game<'a> {
    pub assets: &'a Assets,
    pub buttons: Vec<Button<'a>>,
    pub solved_markers: Vec<SolvedMarker<'a>>,
    pub pieces: PieceCollection<'a>,
}

fn broadcast_input_events(buttons: &mut [Button], new_events: &mut Vec<ButtonEvent>) {
    let (x, y) = mouse_position();
    let mut events = vec![];

    if mouse_delta_position() != Vec2::ZERO {
        events.push(MouseEvent::Moved { x, y });
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        events.push(MouseEvent::Pressed { x, y });
    }

    if is_mouse_button_released(MouseButton::Left) {
        events.push(MouseEvent::Released);
    }

    for control in buttons.iter_mut() {
        for event in &events {
            control.handle_event(event, new_events);
        }
    }
}

fn handle_events(new_events: &[ButtonEvent], pieces: &mut PieceCollection) {
    const ANIMATION_LENGTH: f64 = 0.35;
    for event in new_events.iter() {
        match event {
            ButtonEvent::Pressed(ButtonId::HorizontalRight(row)) => {
                pieces.apply_manipulation(Manipulation::SlideRight(*row), ANIMATION_LENGTH)
            }
            ButtonEvent::Pressed(ButtonId::HorizontalLeft(row)) => {
                pieces.apply_manipulation(Manipulation::SlideLeft(*row), ANIMATION_LENGTH)
            }
            ButtonEvent::Pressed(ButtonId::VerticalUp(col)) => {
                pieces.apply_manipulation(Manipulation::SlideUp(*col), ANIMATION_LENGTH)
            }
            ButtonEvent::Pressed(ButtonId::VerticalDown(col)) => {
                pieces.apply_manipulation(Manipulation::SlideDown(*col), ANIMATION_LENGTH)
            }
            ButtonEvent::Pressed(ButtonId::RotateClockwise(ring)) => {
                pieces.apply_manipulation(Manipulation::RotateClockwise(*ring), ANIMATION_LENGTH)
            }
            ButtonEvent::Pressed(ButtonId::RotateAnticlockwise(ring)) => pieces
                .apply_manipulation(Manipulation::RotateAnticlockwise(*ring), ANIMATION_LENGTH),
        }
    }
}

fn generate_shuffle_manipulations(count: usize) -> Vec<Manipulation> {
    use Manipulation::*;

    let all_manipulations = [
        RotateClockwise(0),
        RotateAnticlockwise(0),
        RotateClockwise(1),
        RotateAnticlockwise(1),
        RotateClockwise(2),
        RotateAnticlockwise(2),
        SlideLeft(3),
        SlideRight(3),
        SlideLeft(4),
        SlideRight(4),
        SlideLeft(5),
        SlideRight(5),
        SlideUp(3),
        SlideDown(3),
        SlideUp(4),
        SlideDown(4),
        SlideUp(5),
        SlideDown(5),
    ];

    let mut manipulations = vec![];
    let mut last_index = None;

    for _ in 0..count {
        let mut index = rand() as usize % all_manipulations.len();

        // Do not allow the same manipulation to happen twice or the two opposite
        // manipulations to happen in one after the other
        while last_index.is_some() && index / 2 == last_index.unwrap() / 2 {
            index = rand() as usize % all_manipulations.len();
        }

        manipulations.push(all_manipulations[index]);
        last_index = Some(index);
    }

    manipulations
}

impl<'a> Game<'a> {
    pub async fn run_shuffle(&mut self, shuffle_count: usize, animation_length: f64) {
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

            self.pieces.update();
            self.draw_all(false, false);
            next_frame().await
        }
    }

    pub async fn run_main_loop(&mut self) {
        loop {
            let mut new_events = vec![];
            broadcast_input_events(&mut self.buttons, &mut new_events);
            self.pieces.update();

            handle_events(&new_events, &mut self.pieces);

            self.draw_all(false, true);

            if self.pieces.is_solved() && !self.pieces.is_animating() {
                break;
            }
            next_frame().await
        }
    }

    pub async fn run_victory_loop(&mut self) {
        use Manipulation::*;

        // Make sure the buttons don't stay hovered
        for button in self.buttons.iter_mut() {
            button.handle_event(&MouseEvent::Moved { x: 0.0, y: 0.0 }, &mut vec![]);
        }

        loop {
            if !self.pieces.is_animating() {
                self.pieces.apply_manipulation(RotateClockwise(0), 1.0);
                self.pieces.apply_manipulation(RotateClockwise(1), 1.0);
                self.pieces.apply_manipulation(RotateClockwise(2), 1.0);
            }

            self.pieces.update();
            self.draw_all(false, true);

            // Additionally draw the victory marker
            draw_texture(&self.assets.img_victory_marker, 0., 0., WHITE);

            next_frame().await
        }
    }

    pub async fn run_blink_loop(&mut self, blink_time_sec: f64) {
        let start = now();

        while now() - start < blink_time_sec {
            self.draw_all(true, true);
            next_frame().await;
        }
    }

    pub async fn wait(&self, time_sec: f64) {
        let start = now();

        while now() - start < time_sec {
            self.draw_all(false, true);
            next_frame().await;
        }
    }

    fn draw_all(&self, draw_buttons_as_hovered: bool, draw_solved_markers: bool) {
        // Draw the background
        draw_texture(&self.assets.img_board, 0., 0., WHITE);

        // Draw solved markers
        if draw_solved_markers {
            self.solved_markers
                .iter()
                .filter(|marker| self.pieces.is_square_solved(marker.square()))
                .for_each(|marker| marker.draw());
        }

        // Draw the rotational buttons
        let rotational_buttons = self.buttons.iter().filter(|button| {
            matches!(
                button.id(),
                ButtonId::RotateClockwise(_) | ButtonId::RotateAnticlockwise(_)
            )
        });

        for button in rotational_buttons {
            button.draw(draw_buttons_as_hovered);
        }

        // Draw the pieces
        self.pieces.draw();

        // Draw surroundings and cover pieces outside the board
        draw_texture(&self.assets.img_surroundings, 0., 0., WHITE);

        // Draw the linear buttons
        let linear_buttons = self.buttons.iter().filter(|button| {
            !matches!(
                button.id(),
                ButtonId::RotateClockwise(_) | ButtonId::RotateAnticlockwise(_)
            )
        });

        for drawable in linear_buttons {
            drawable.draw(draw_buttons_as_hovered);
        }
    }
}
