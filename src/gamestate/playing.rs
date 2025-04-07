use std::time::Duration;

use macroquad::{color::{Color, BLACK}, math::vec2, window::clear_background};
use macroquad::prelude::{draw_text_ex, measure_text, screen_width};
use macroquad::text::TextParams;
use crate::controls::Action;
use crate::gamedata::GameData;
use crate::gamestate::pause::PauseGS;
use crate::util::{draw_ansi_text, remove_ansii_escape_codes};
use crate::world::interactable::Interactable;
use super::{GameState, GameStateAction, GameStateError};

#[derive(Clone, Debug)]
pub struct PlayingGS {
    paused: bool,
    debug: bool,
}

impl PlayingGS {
    pub fn new() -> Result<Box<Self>, GameStateError> {
        Ok(Box::new(Self {
            paused: false,
            debug: false,
        }))
    }
}

impl GameState for PlayingGS {

    fn update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameStateError> {

        // make the player rotate towards the mouse
        // not top down anymore
        //self.player.look_towards_mouse();

        // handle input and make the player respond accordingly
        let actions = data.control_handler.get_actions();
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
                    if let Some(id) = data.world.is_click_on_interactable(data) {
                        if let Some(interactable) = data.world.get_mut_interactable_by_id(id) {
                            return interactable.interact();
                        }
                    }
                }
                Action::Inventory => {
                    // todo: add an inventory system and open it here
                }
                Action::BasicAttack => {
                    // todo: add attacks
                }
                Action::Debug => {
                    self.debug = !self.debug;
                }
                Action::Pause => {
                    return Ok(GameStateAction::PushTopState(Box::new(PauseGS::new())))
                }
                _ => { /* Other actions are not used here */ }
            }
        }
        data.world.player.apply_movement(movement, &data.world.interactables, &data.assets, delta_time.as_millis());

        Ok(GameStateAction::NoOp)
    }

    fn pause(&mut self, data: &mut GameData) -> Result<(), GameStateError> {
        self.paused = true;
        Ok(())
    }

    fn restore(&mut self, data: &mut GameData) -> Result<(), GameStateError> {
        // Refresh everything that needs to be
        if let Err(e) = data.reload_controls() {
            return Err(GameStateError::RuntimeError(format!("Failed to reload controls: {}", e)));
        }
        self.paused = false;
        Ok(())
    }

    fn draw(&self, data: &mut GameData) -> Result<(), GameStateError> {
        // clear the background and give a default color
        clear_background(Color::from_hex(0x453e3d));
        // draw the FPS counter in the top right
        draw_text_ex(&format!("FPS: {}", data.fps.round()), 2.0, 12.0, TextParams {
            font: Some(&data.assets.font),
            font_size: 8,
            color: BLACK,
            ..Default::default()
        });

        // draw the player
        data.world.draw_player();

        // draw the interactables
        data.world.draw_interactables(&data.assets);

        if self.debug {
            draw_text_ex(&format!("Player Pos: {}", data.world.player.pos), 2.0, 28.0, TextParams {
                font: Some(&data.assets.font),
                font_size: 8,
                color: BLACK,
                ..Default::default()
            });
            draw_text_ex(&format!("Paused: {}", self.paused), 2.0, 44.0, TextParams {
                font: Some(&data.assets.font),
                font_size: 8,
                color: BLACK,
                ..Default::default()
            });
            let ansi_test = format!("Color Test: {}1{}2{}3{}4{}5{}6{}7{}8{}9{}0{}a{}b{}c{}d{}e{}f",
                                    better_term::Color::BrightBlue,
                                    better_term::Color::Green,
                                    better_term::Color::Cyan,
                                    better_term::Color::Red,
                                    better_term::Color::Purple,
                                    better_term::Color::Yellow,
                                    better_term::Color::White,
                                    better_term::Color::BrightBlack,
                                    better_term::Color::Blue,
                                    better_term::Color::Black,
                                    better_term::Color::BrightGreen,
                                    better_term::Color::BrightCyan,
                                    better_term::Color::BrightRed,
                                    better_term::Color::BrightPurple,
                                    better_term::Color::BrightYellow,
                                    better_term::Color::BrightWhite);
            //debug!("{}", ansi_test.escape_default());
            let raw_ansi_test = remove_ansii_escape_codes(&ansi_test);
            let text_size = measure_text(&raw_ansi_test, Some(&data.assets.font), 8, 1.0);
            draw_ansi_text(
                &ansi_test,
                vec2(screen_width() - (text_size.width + 10.0), 15.0),
                &data.assets,
                8,
                4.0,
            );
        }

        if !self.paused {
            data.world.handle_tooltips(data);
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "Playing".to_string()
    }

}