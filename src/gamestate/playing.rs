use std::time::Duration;

use macroquad::{color::Color, math::vec2, window::clear_background};
use macroquad::prelude::{draw_text_ex, measure_text, screen_width};
use macroquad::text::TextParams;
use crate::controls::Action;
use crate::gamedata::GameData;
use crate::overlay::pause::PauseOverlay;
use crate::world::player::PlayerFacing;
use crate::util::{draw_ansi_text, remove_ansii_escape_codes};
use super::{GameState, GameStateAction, GameStateError};

#[derive(Clone, Debug)]
pub struct PlayingGS {
    paused: bool,
    debug: bool, // todo: move this to gamedata
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

        let sprinting_toggled = data.control_handler.is_sprint_toggle();
        if !sprinting_toggled {
            // if sprinting is not in toggle mode, it will be set to true each frame only if the button is pressed
            // this is a limitation of Banana Engine that should be addressed in the future, possibly with a control handler rework
            data.world.player.sprinting = false;
        }

        // handle input and make the player respond accordingly
        let actions = data.control_handler.get_actions();
        let mut movement = vec2(0.0, 0.0);
        // handle various movement types
        for action in actions {
            match action {
                // todo: add limits like obstacles
                Action::MoveUp => {
                    movement.y -= 1.0;
                    data.world.player.facing = match data.world.player.facing {
                        PlayerFacing::UpRight => PlayerFacing::UpRight,
                        PlayerFacing::UpLeft => PlayerFacing::UpLeft,
                        PlayerFacing::DownLeft => PlayerFacing::UpLeft,
                        PlayerFacing::DownRight => PlayerFacing::UpRight,
                    };
                }
                Action::MoveDown => {
                    movement.y += 1.0;
                    data.world.player.facing = match data.world.player.facing {
                        PlayerFacing::UpRight => PlayerFacing::DownRight,
                        PlayerFacing::UpLeft => PlayerFacing::DownLeft,
                        PlayerFacing::DownLeft => PlayerFacing::DownLeft,
                        PlayerFacing::DownRight => PlayerFacing::DownRight,
                    };
                }
                Action::MoveLeft => {
                    movement.x -= 1.0;
                    data.world.player.facing = match data.world.player.facing {
                        PlayerFacing::UpRight => PlayerFacing::UpLeft,
                        PlayerFacing::UpLeft => PlayerFacing::UpLeft,
                        PlayerFacing::DownLeft => PlayerFacing::DownLeft,
                        PlayerFacing::DownRight => PlayerFacing::DownLeft,
                    };
                }
                Action::MoveRight => {
                    movement.x += 1.0;
                    data.world.player.facing = match data.world.player.facing {
                        PlayerFacing::UpRight => PlayerFacing::UpRight,
                        PlayerFacing::UpLeft => PlayerFacing::UpRight,
                        PlayerFacing::DownLeft => PlayerFacing::DownRight,
                        PlayerFacing::DownRight => PlayerFacing::DownRight,
                    };
                }
                Action::Interact => {
                    if let Some(id) = data.world.is_click_on_interactable(data) {
                        if let Some(interactable) = data.world.get_mut_interactable_by_id(id) {
                            return interactable.interact();
                        }
                    }
                }
                Action::Sprint => {
                    if sprinting_toggled {
                        data.world.player.sprinting = !data.world.player.sprinting;
                    } else {
                        data.world.player.sprinting = true;
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
                    self.paused = true;
                    return Ok(GameStateAction::SpawnOverlay(Box::new(PauseOverlay::new())))
                }
                _ => { /* Other actions are not used here */ }
            }
        }
        data.world.player.apply_movement(movement, &data.world.interactables, &data.assets, delta_time.as_millis());

        Ok(GameStateAction::NoOp)
    }

    fn persistent_update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<GameStateAction, GameStateError> {
        // not yet implemented
        Ok(GameStateAction::NoOp)
    }

    fn pause(&mut self, _data: &mut GameData) -> Result<(), GameStateError> {
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

        // draw the player
        data.world.draw_player();

        // draw the interactables
        data.world.draw_interactables(&data.assets);

        if self.debug {
            let spacing = 4.0;
            let debug_info = vec![
                format!("FPS: {}", data.fps.round()),
                format!("Player Pos: {}", data.world.player.pos.round()),
                format!("Paused: {}", self.paused),
                format!("Sprinting: {}", data.world.player.sprinting),
                format!("Facing: {:?}", data.world.player.facing),
            ];
            for (i, info) in debug_info.iter().enumerate() {
                let text_size = measure_text(info, Some(&data.assets.font), 8, 1.0);
                draw_text_ex(
                    info,
                    spacing,
                    (text_size.height + spacing) * (i + 1) as f32,
                    TextParams {
                        font: Some(&data.assets.font),
                        font_size: 8,
                        color: Color::from_rgba(255, 255, 255, 180),
                        ..Default::default()
                    },
                );
            }

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
                vec2(screen_width() - (text_size.width + spacing), text_size.height + spacing),
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