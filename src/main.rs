use std::time::Instant;
use gamestate::GameState;
use macroquad::prelude::*;
use crate::world::World;
/***
 # SAVED RESOURCES:
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
mod logging;
mod world;
mod ui;
mod util;
mod assets;
mod gamedata;

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

    // create the global assets object
    let global_assets = assets::GlobalAssets::load().await;
    if let Err(e) = global_assets {
        return error!("Failed to load global assets: {}", e);
    }
    let global_assets = global_assets.unwrap();

    // create a dynamic gamestate object
    let gamestate = gamestate::playing::PlayingGS::new();
    if let Err(e) = gamestate {
        return error!("{}", e);
    }
    let gamestate: Box<dyn GameState> = gamestate.unwrap();

    // the previous gamestate
    let mut gamestate_manager = gamestate::GameStateManager::new(gamestate);

    // load the control handler
    let control_handler = controls::ControlHandler::load();
    if let Err(e) = control_handler {
        return error!("Failed to load control handler: {}", e);
    }
    let control_handler = control_handler.unwrap();

    // create the world
    let world = World::new().await;
    if let Err(e) = world {
        return error!("Failed to create world: {}", e);
    }
    let world = world.unwrap();

    // create the gamedata object
    let mut gamedata = gamedata::GameData {
        fps: 0.0,
        assets: global_assets.clone(),
        control_handler,
        world,
    };

    // handle FPS calculations
    let mut last_time = Instant::now();
    let mut fps_values = vec![0.0; FPS_SMOOTHING_FRAMES];
    let mut fps_index = 0;
    let mut fps_sum = 0.0;

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
        gamedata.fps = {
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
        let update_result = gamestate_manager.get_top_state().update(&delta_time, &mut gamedata);
        if let Err(update_error) = &update_result {
            error!("Failed to update gamestate: {}", update_error);
        }
        let gamestate_action = update_result.unwrap();
        match gamestate_action {
            gamestate::GameStateAction::ChangeState(new_state) => {
                if let Err(e) = gamestate_manager.change_play_state(new_state, &mut gamedata) {
                    error!("Failed to change gamestate: {}", e);
                }
            }
            gamestate::GameStateAction::PushTopState(mut state) => {
                if let Err(e) = gamestate_manager.get_top_state().pause(&mut gamedata) {
                    error!("Failed to pause gamestate: {}", e);
                }
                if let Err(e) = state.restore(&mut gamedata) {
                    error!("Failed to restore gamestate: {}", e);
                }
                if let Err(e) = gamestate_manager.push_top_state(state, &mut gamedata) {
                    error!("Failed to push gamestate: {}", e);
                }
                debug!("Gamestate stack: ps[{:?}] stack{:?}", gamestate_manager.play_state, gamestate_manager.top_states);
            }
            gamestate::GameStateAction::PopTopState => {
                if let Err(e) = gamestate_manager.get_top_state().pause(&mut gamedata) {
                    error!("Failed to pause gamestate: {}", e);
                }
                if let Err(e) = gamestate_manager.pop_top_state(&mut gamedata) {
                    error!("Failed to pop gamestate: {}", e);
                }
            }
            gamestate::GameStateAction::NoOp => { /* do nothing */ }
        }

        // == RENDER ==

        // call the gamestate's draw function

        // Determine how many states (from the top down) are overlays
        let overlay_count = gamestate_manager
            .top_states
            .iter()
            .rev()
            .take_while(|s| s.is_overlay())
            .count();

        // Calculate the starting index to draw from
        let start_index = gamestate_manager.top_states.len().saturating_sub(overlay_count);

        // If all states are overlays, we need to draw the play_state first
        if overlay_count == gamestate_manager.top_states.len() {
            if let Err(e) = gamestate_manager.play_state.draw(&mut gamedata) {
                error!("Failed to draw play_state: {}", e);
            }
        }

        // Draw from the lowest relevant top_state up to the top
        for state in gamestate_manager.top_states.iter().skip(start_index) {
            if let Err(e) = state.draw(&mut gamedata) {
                error!("Failed to draw gamestate: {}", e);
            }
        }

        // Draw the top state (optional redundancy check, since it was already drawn above if it's the top)
        if let Err(e) = gamestate_manager.get_top_state().draw(&mut gamedata) {
            error!("Failed to draw top gamestate: {}", e);
        }

        next_frame().await;
    }
}
