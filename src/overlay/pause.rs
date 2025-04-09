use std::time::Duration;

use macroquad::{color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}};
use crate::controls::{Action, ControlHandler};
use crate::gamedata::GameData;
use crate::overlay::{Overlay, OverlayAction, OverlayError};

#[derive(Debug)]
pub struct PauseOverlay {}

impl PauseOverlay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Overlay for PauseOverlay {

    fn update(&mut self, _delta_time: &Duration, _data: &mut GameData) -> Result<OverlayAction, OverlayError> {
        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(OverlayError::Update(e));
        }
        let control_handler = control_handler.unwrap();
        // handle on release to ensure pause key isn't spammed when held (was an issue)
        let actions = control_handler.get_actions();
        for action in actions {
            match action {
                Action::Pause => {
                    // if let Err(e) = self.previous_play_state.restore() {
                    //     return Err(e);
                    // }
                    // pop the top state
                    return Ok(OverlayAction::Exit);
                }
                // todo: UI stuff
                _ => {
                    // do nothing
                }
            }
        }
        Ok(OverlayAction::NoOp)
    }

    fn draw(&self, _data: &mut GameData) -> Result<(), OverlayError> {
        // draw a semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

        Ok(())
    }

    fn draw_below(&self) -> bool {
        true
    }

}