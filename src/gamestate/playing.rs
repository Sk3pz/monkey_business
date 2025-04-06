use std::time::Duration;

use macroquad::{color::{Color, BLACK, WHITE}, math::vec2, text::draw_text, texture::{draw_texture_ex, DrawTextureParams}, window::clear_background};

use crate::{controls::ControlHandler, debug, player};
use crate::controls::Action;
use super::{GameState, GameStateAction, GameStateError};

#[derive(Clone)]
pub struct PlayingGS {
    player: player::Player,
    control_handler: ControlHandler,
}

impl PlayingGS {
    pub async fn new() -> Result<Box<Self>, GameStateError> {
        let player = player::Player::new().await;
        if let Err(e) = player {
            return Err(GameStateError::InitializationError(format!("Failed to initialize player: {}", e)));
        }
        let player = player.unwrap();

        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(GameStateError::InitializationError(format!("{}", e)));
        }
        let control_handler = control_handler.unwrap();

        Ok(Box::new(Self {
            player,
            control_handler,
        }))
    }

    pub fn reload_controls(&mut self) -> Result<(), GameStateError> {
        let new_ct_handler = ControlHandler::load();
        if let Err(e) = new_ct_handler {
            return Err(GameStateError::InitializationError(format!("{}", e)));
        }
        let new_ct_handler = new_ct_handler.unwrap();
        self.control_handler = new_ct_handler;
        Ok(())
    }
}

impl GameState for PlayingGS {

    fn update(&mut self, delta_time: &Duration) -> Result<GameStateAction, GameStateError> {

        // make the player rotate towards the mouse
        self.player.look_towards_mouse();

        // handle input and make the player respond accordingly
        let actions = self.control_handler.get_actions();
        let mut movement = vec2(0.0, 0.0);
        // handle various movement types
        for action in actions {
            match action {
                // todo: add limits like obstacles
                Action::MoveUp => {
                    movement.y -= 1.0;
                }
                Action::MoveDown => {
                    movement.y += 1.0;
                }
                Action::MoveLeft => {
                    movement.x -= 1.0;
                }
                Action::MoveRight => {
                    movement.x += 1.0;
                }
                Action::Interact => {

                }
                Action::Inventory => {
                    // todo: add an inventory system and open it here
                    debug!("Opened inventory!");
                }
                Action::BasicAttack => {
                    // todo: add attacks
                    debug!("Attacked!");
                }
                Action::Pause => {
                    return Ok(GameStateAction::ChangeState(Box::new(super::pause::PauseGS::new(self.clone()))))
                }
                _ => { /* Other actions are not used here */ }
            }
        }
        self.player.apply_movement(movement, delta_time.as_millis());

        Ok(GameStateAction::NoOp)
    }

    fn draw(&self, fps: f32) -> Result<(), GameStateError> {
        // clear the background and give a default color
        clear_background(Color::from_rgba(222, 192, 138, 255));
        // draw the FPS counter in the top right
        draw_text(&format!("FPS: {}", fps.round()), 2.0, 12.0, 20.0, BLACK);

        // draw the player
        draw_texture_ex(
            &self.player.sprite, 
            self.player.pos.x, self.player.pos.y,  
            WHITE,
            DrawTextureParams {
                rotation: self.player.rotation,
                ..Default::default()
            }
        );

        Ok(())
    }

}