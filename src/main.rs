use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};
use gamestate::GameState;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::prelude::*;

/***
 # SAVED RESOURCES:
   https://macroquad.rs/articles/fish-tutorial/
   https://docs.rs/macroquad/latest/macroquad/
   https://github.com/not-fl3/macroquad/blob/master/examples/ui.rs
   https://github.com/not-fl3/macroquad/blob/master/examples/texture.rs
   https://macroquad.rs/articles/docker/
   https://macroquad.rs/articles/android/

   NETWORKING:
   https://crates.io/crates/ggrs
***/

/* TODO BUGS:
    * Fix reoccurring issue where the player randomly gains unreasonable velocity
*/

mod player;
mod controls;
mod networking;
mod gamestate;
mod logging;
mod world;

const FPS_SMOOTHING_FRAMES: usize = 30;

const DEBUG_OUTPUT: bool = cfg!(debug_assertions);

fn window_config() -> Conf {
    Conf {
        window_title: "Monkey Business".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {

    // todo: move this to its own music handler
    let Ok(music) = load_sound("assets/audio/music/Ghouls.wav").await else {
        return error!("Failed to load sound");
    };

    // create a dynamic gamestate object
    let gamestate = gamestate::playing::PlayingGS::new().await;
    if let Err(e) = gamestate {
        return error!("{}", e);
    }
    let mut gamestate: Box<dyn GameState> = gamestate.unwrap();

    // handle FPS calculations
    let mut last_time = Instant::now();
    let mut fps_values = vec![0.0; FPS_SMOOTHING_FRAMES];
    let mut fps_index = 0;
    let mut fps_sum = 0.0;
    
    // play_sound(&music, PlaySoundParams {
    //     looped: true,
    //     volume: 0.1,
    // });

    // render loop
    loop {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now - last_time;
        last_time = now;
        // Convert delta time to seconds as a float
        let delta_seconds = delta_time.as_secs_f32();
        // Use delta_seconds for movement, animation, etc.

        // Calculate the averaged FPS
        let smoothed_fps = {
            // FPS calculations
            let fps = if delta_seconds > 0.0 {
                1.0 / delta_seconds
            } else {
                0.0
            };
            // Update the moving average
            fps_sum -= fps_values[fps_index];
            fps_values[fps_index] = fps;
            fps_sum += fps;
            fps_index = (fps_index + 1) % FPS_SMOOTHING_FRAMES;

            fps_sum / FPS_SMOOTHING_FRAMES as f32
        };

        // == UPDATE ==

        // call the gamestate's update function
        let update_result = gamestate.update(&delta_time);
        if let Err(update_error) = update_result {
            // todo: maybe don't always crash if there's an error here?
            return error!("Failed to update gamestate: {}", update_error);
        }
        let gamestate_action = update_result.unwrap();
        match gamestate_action {
            gamestate::GameStateAction::ChangeState(new_state) => {
                gamestate = new_state;
            }
            gamestate::GameStateAction::NoOp => { /* do nothing */ }
        }

        // == RENDER ==

        // call the gamestate's draw function
        if let Err(draw_error) = gamestate.draw(smoothed_fps) {
            // todo: maybe don't always crash here?
            return error!("Failed to draw gamestate: {}", draw_error);
        }
        next_frame().await
    }
}
