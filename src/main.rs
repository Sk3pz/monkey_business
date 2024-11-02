use std::time::Instant;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::prelude::*;

use crate::controls::{Action, ControlHandler};

/***
 # SAVED RESOURCES:
   https://macroquad.rs/articles/fish-tutorial/
   https://docs.rs/macroquad/latest/macroquad/
   https://github.com/not-fl3/macroquad/blob/master/examples/ui.rs
   https://github.com/not-fl3/macroquad/blob/master/examples/texture.rs
   https://macroquad.rs/articles/docker/
   https://macroquad.rs/articles/android/
 ***/

// if true, controls will be relative to mouse direction rather than up / down / left / right
const MOVEMENT_RELATIVE_TO_MOUSE: bool = false;
// if true, player will always move towards the mouse
const MOVEMENT_RELATIVE_TO_MOUSE_MODIFIED: bool = false;

mod player;
mod controls;

#[macroquad::main("Monkey Business")]
async fn main() {

    // todo: move this to its own music handler
    let Ok(music) = load_sound("assets/audio/music/Ghouls.wav").await else {
        return eprintln!("Failed to load sound");
    };
    
    // get the control mapping
    let control_handler = ControlHandler::load();
    
    // create the player
    let mut player = {
        let res = player::Player::new().await;
        if let Err(e) = res {
            return eprintln!("Failed to initialize player: {}", e);
        }
        res.unwrap()
    };

    let mut last_time = Instant::now();
    
    play_sound(&music, PlaySoundParams {
        looped: true,
        volume: 0.1,
    });
    
    loop {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now - last_time;
        last_time = now;
        // Convert delta time to seconds as a float
        let delta_seconds = delta_time.as_secs_f32();
        // Use delta_seconds for movement, animation, etc.
        let fps = if delta_seconds > 0.0 {
            1.0 / delta_seconds
        } else {
            0.0
        };
        println!("FPS: {}", fps);

        // clear the background and give a default color
        clear_background(Color::from_rgba(222, 192, 138, 255));

        // draw the player texture
        draw_texture_ex(
            &player.sprite,
            player.pos.x, player.pos.y,
            WHITE,
            DrawTextureParams {
                rotation: player.rotation,
                ..Default::default()
            }
        );

        // handle input
        player.look_towards_mouse();

        let actions = control_handler.get_keys_down();
        let mut movement = vec2(0.0, 0.0);
        if MOVEMENT_RELATIVE_TO_MOUSE && MOVEMENT_RELATIVE_TO_MOUSE_MODIFIED {
            if !player.is_on_mouse() && !control_handler.is_action_pressed(Action::MoveDown) {
                movement.x += (player.rotation - std::f32::consts::PI / 2.0).cos();
                movement.y += (player.rotation - std::f32::consts::PI / 2.0).sin();
            }
        }
        for action in actions {
            match action {
                // todo: add limits like obstacles
                Action::MoveUp => {
                    if MOVEMENT_RELATIVE_TO_MOUSE {
                        if !player.is_on_mouse() && !MOVEMENT_RELATIVE_TO_MOUSE_MODIFIED {
                            movement.x += (player.rotation - std::f32::consts::PI / 2.0).cos();
                            movement.y += (player.rotation - std::f32::consts::PI / 2.0).sin();
                        }
                    } else {
                        movement.y -= 1.0;
                    }
                }
                Action::MoveDown => {
                    if MOVEMENT_RELATIVE_TO_MOUSE {
                        movement.x += (player.rotation + std::f32::consts::PI / 2.0).cos();
                            movement.y += (player.rotation + std::f32::consts::PI / 2.0).sin();
                    } else {
                        movement.y += 1.0;
                    }
                }
                Action::MoveLeft => {
                    if MOVEMENT_RELATIVE_TO_MOUSE {
                        movement.x -= player.rotation.cos();
                        movement.y -= player.rotation.sin();
                    } else {
                        movement.x -= 1.0;
                    }
                }
                Action::MoveRight => {
                    if MOVEMENT_RELATIVE_TO_MOUSE {
                        movement.x += player.rotation.cos();
                        movement.y += player.rotation.sin();
                    } else {
                        movement.x += 1.0;
                    }
                }
                Action::Pause => {
                    todo!()
                }
            }
        }
        player.apply_movement(movement, delta_time.as_millis());

        // todo: use std::thread::sleep and delta_time to cap framerate if needed
        
        // call the next frame
        next_frame().await
    }
}
