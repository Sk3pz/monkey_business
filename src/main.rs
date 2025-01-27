
use std::time::Instant;
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

mod player;
mod controls;
mod networking;
mod gamestate;

const FPS_SMOOTHING_FRAMES: usize = 30;

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
        return eprintln!("Failed to load sound");
    };

    // create a dynamic gamestate object
    let gamestate = gamestate::playing::PlayingGS::new().await;
    if let Err(e) = gamestate {
        return eprintln!("Failed to initialize gamestate: {}", e);
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

        // call the gamestate's update function
        let update_result = gamestate.update(&delta_time);
        if let Err(update_error) = update_result {
            // todo: maybe don't always crash if there's an error here?
            return eprintln!("Failed to update gamestate: {}", update_error);
        }
        let gamestate_action = update_result.unwrap();
        match gamestate_action {
            gamestate::GameStateAction::ChangeState(new_state) => {
                gamestate = new_state;
            }
            gamestate::GameStateAction::NoOp => {}
        }

        // draw should be delayed relative to self not to the game loop because update ticks are different from frames
        // call the gamestate's draw funtion
        if let Err(draw_error) = gamestate.draw(smoothed_fps) {
            // todo: maybe don't always crash here?
            return eprintln!("Failed to draw gamestate: {}", draw_error);
        }
        
        // call the next frame
        next_frame().await
    }
}
