use std::time::Duration;

use macroquad::{color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}};
use crate::controls::{Action, ControlHandler};
use crate::gamedata::GameData;
use super::{GameState, GameStateAction, GameStateError};

#[derive(Debug)]
pub struct PauseGS {}

impl PauseGS {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameState for PauseGS {

    fn update(&mut self, _delta_time: &Duration, _data: &mut GameData) -> Result<GameStateAction, GameStateError> {
        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(GameStateError::RuntimeError(e));
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
                    return Ok(GameStateAction::PopTopState);
                }
                // todo: UI stuff
                _ => {
                    // do nothing
                }
            }
        }
        Ok(GameStateAction::NoOp)
    }

    fn pause(&mut self, _data: &mut GameData) -> Result<(), GameStateError> {
        Ok(())
    }

    fn restore(&mut self, _data: &mut GameData) -> Result<(), GameStateError> {
        Ok(())
    }

    fn draw(&self, _data: &mut GameData) -> Result<(), GameStateError> {
        // draw a semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

        Ok(())
    }

    fn get_name(&self) -> String {
        "Paused".to_string()
    }

    fn is_overlay(&self) -> bool { true }

}