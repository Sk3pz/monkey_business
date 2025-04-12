use std::time::Instant;
use gamestate::GameState;
use macroquad::prelude::*;
use crate::overlay::OverlayManager;
use crate::startup::startup_loading_screen;
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

mod controls;
mod gamestate;
mod logging;
mod world;
mod ui;
mod util;
mod assets;
mod gamedata;
mod settings;
mod animation;
mod overlay;
mod minigame;
mod error;
mod startup;
/***
 * TODO:
 *   - Better error handling + log files
 *   - Plan Story
 *   - Add a scene/world system
 *   - Add a UI system - could use new overlay system or be drawn by the current gamestate
 *   - Particle System
***/

const FPS_SMOOTHING_FRAMES: usize = 30;

const DEBUG_OUTPUT: bool = cfg!(debug_assertions);

const BASE_WINDOW_SIZE: (i32, i32) = (1000, 700);

// todo: make it default fullscreen?
fn window_config() -> Conf {
    Conf {
        window_title: "Monkey Business".to_string(),
        window_width: BASE_WINDOW_SIZE.0,
        window_height: BASE_WINDOW_SIZE.1,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // windowed fullscreen cheat
    // let (width, height) = (screen_width(), screen_height());
    // set_fullscreen(false);
    // request_new_screen_size(width, height);

    // create a dynamic gamestate object
    let gamestate = gamestate::playing::PlayingGS::new();
    if let Err(e) = gamestate {
        return error!("{}", e);
    }
    let mut gamestate: Box<dyn GameState> = gamestate.unwrap();

    // create the overlay manager
    let mut overlay_manager = OverlayManager::new();

    let gamedata = startup_loading_screen().await;
    if let Err(e) = gamedata {
        return error!("Failed to load game data: {}", e);
    }
    let mut gamedata = gamedata.unwrap();

    // // create the global assets object
    // let global_assets = assets::GlobalAssets::load().await;
    // if let Err(e) = global_assets {
    //     return error!("Failed to load global assets: {}", e);
    // }
    // let global_assets = global_assets.unwrap();
    //
    // load the settings
    // todo: let settings = settings::Settings::load();
    // let settings = settings::Settings {
    //     volume: 1.0,
    //     mute: false,
    // };
    //
    // // load the control handler
    // let control_handler = controls::ControlHandler::load();
    // if let Err(e) = control_handler {
    //     return error!("Failed to load control handler: {}", e);
    // }
    // let mut control_handler = control_handler.unwrap();
    // // TODO: remove this when settings are implemented properly.
    // //   this changes the bind based on the controls.dat file, which is currently the only way
    // //   to change sprinting between toggled and held
    // control_handler.set_sprint_toggle(control_handler.is_sprint_toggle());
    //
    // // create the world
    // let world = World::new(&global_assets).await;
    // if let Err(e) = world {
    //     return error!("Failed to create world: {}", e);
    // }
    // let world = world.unwrap();
    //
    // // create the gamedata object
    // let mut gamedata = gamedata::GameData {
    //     fps: 0.0,
    //     assets: global_assets.clone(),
    //     settings,
    //     control_handler,
    //     world,
    // };

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

        // === UPDATE ===
        // run the gamestate's persistent update function
        if let Err(e) = gamestate.persistent_update(&delta_time, &mut gamedata) {
            return error!("Failed to run persistent update: {}", e);
        }
        // update the overlay
        if let Some(r) = overlay_manager.update(&delta_time, &mut gamedata) {
            match r {
                Ok(action) => match action {
                    overlay::OverlayAction::Exit => {
                        // pop the top overlay
                        overlay_manager.pop();
                        // resume the gamestate
                        if overlay_manager.len() == 0 {
                            if let Err(e) = gamestate.restore(&mut gamedata) {
                                return error!("Failed to restore gamestate: {}", e);
                            }
                        }
                    }
                    overlay::OverlayAction::SpawnOverlay(overlay) => match overlay_manager.push(overlay, &mut gamedata) {
                        Ok(_) => {}
                        Err(e) => return error!("Failed to push overlay: {}", e),
                    },
                    overlay::OverlayAction::NoOp => {}
                },
                Err(e) => return error!("Failed to update overlay: {}", e),
            }
        } else {
            // update the gamestate
            match gamestate.update(&delta_time, &mut gamedata) {
                Ok(action) => match action {
                    gamestate::GameStateAction::ChangeState(new_state) => gamestate = new_state,
                    gamestate::GameStateAction::SpawnOverlay(overlay) =>{
                        match overlay_manager.push(overlay,&mut gamedata) {
                            Ok(_) => {}
                            Err(e) => return error!("Failed to push overlay: {}", e),
                        }
                        // pause the gamestate
                        if let Err(e) = gamestate.pause(&mut gamedata) {
                            return error!("Failed to pause gamestate: {}", e);
                        }
                    },
                    gamestate::GameStateAction::NoOp => {}
                },
                Err(e) => return error!("Failed to update gamestate: {}", e),
            }
        }

        // === RENDER ===
        // draw the gamestate
        if overlay_manager.should_draw_gamestate() {
            if let Err(e) = gamestate.draw(&delta_time, &mut gamedata) {
                return error!("Failed to draw top gamestate: {}", e);
            }
        }

        // draw all overlays on top
        if let Err(e) = overlay_manager.draw(&mut gamedata) {
            return error!("Failed to draw overlay: {}", e);
        }

        next_frame().await;
    }
}
