use macroquad::color::{DARKGRAY, GREEN};
use macroquad::prelude::{clear_background, draw_texture_ex, get_time, next_frame, screen_height, screen_width, vec2, DrawTextureParams, FilterMode, Texture2D, BLACK, WHITE};
use crate::assets;
use crate::error::GameError;
use crate::gamedata::GameData;
use crate::settings::Settings;
use crate::util::draw_rounded_rect;

const BANANA_BYTES: &[u8] = include_bytes!("../assets/sprites/engine_logo.png");

async fn draw_loading_screen_frame(percentage: usize, banana_texture: &Texture2D) {
    clear_background(BLACK);

    let banana_scale = 512.0;

    // draw the banana
    draw_texture_ex(
        &banana_texture,
        screen_width() / 2.0 - banana_scale / 2.0,
        screen_height() / 2.0 - banana_scale / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(banana_scale, banana_scale)),
            ..Default::default()
        },
    );

    let progress_bar_width = 480.0;
    let progress_bar_height = 16.0;

    let progress_bar_radius = 2.0;

    // let progress_bar_x = screen_width() / 2.0 - progress_bar_width / 2.0;
    // let progress_bar_y = screen_height() / 2.0 + banana_scale / 2.0 + 10.0;
    let progress_bar_x = screen_width() / 2.0 - progress_bar_width / 2.0;
    let progress_bar_y = screen_height() / 2.0 - progress_bar_height / 2.0;
    let progress_bar_inner_width = (progress_bar_width * (percentage as f32 / 100.0)).round();
    let progress_bar_inner_x = progress_bar_x;
    let progress_bar_inner_y = progress_bar_y;

    draw_rounded_rect(
        vec2(progress_bar_x, progress_bar_y),
        vec2(progress_bar_width, progress_bar_height),
        progress_bar_radius,
        DARKGRAY,
        false,
        None,
    );
    draw_rounded_rect(
        vec2(progress_bar_inner_x, progress_bar_inner_y),
        vec2(progress_bar_inner_width, progress_bar_height),
        progress_bar_radius,
        GREEN,
        false,
        None,
    );

    next_frame().await;
}

pub async fn sleep(secs: f64) {
    let start = get_time();
    while get_time() - start < secs {
        next_frame().await;
    }
}

pub async fn startup_loading_screen() -> Result<GameData, GameError> {
    let banana_texture = Texture2D::from_file_with_format(BANANA_BYTES, Some(macroquad::prelude::ImageFormat::Png));
    banana_texture.set_filter(FilterMode::Nearest);

    draw_loading_screen_frame(0, &banana_texture).await;

    // Load assets
    let assets = match assets::GlobalAssets::load().await {
        Ok(a) => a,
        Err(e) => {
            return Err(GameError::Initialization(format!("Failed to load assets: {}", e)));
        }
    };

    draw_loading_screen_frame(1, &banana_texture).await;

    //let settings = crate::settings::Settings::load();
    let settings = Settings {
        volume: 1.0,
        mute: false,
    };

    let control_handler = match crate::controls::ControlHandler::load() {
        Ok(c) => c,
        Err(e) => {
            return Err(GameError::Initialization(format!("Failed to load control handler: {}", e)))
        }
    };

    draw_loading_screen_frame(5, &banana_texture).await;

    let world = match crate::world::World::new(&assets).await {
        Ok(w) => w,
        Err(e) => {
            return Err(GameError::Initialization(format!("Failed to create world: {}", e)));
        }
    };

    draw_loading_screen_frame(10, &banana_texture).await;


    // Simulate loading
    for i in 10..=100 {
        draw_loading_screen_frame(i, &banana_texture).await;
        for _j in 0..10000000 {
            // avoid compiler optimizations:
            let _ = i * 2;
        }
    }

    Ok(GameData {
        fps: 0.0,
        assets,
        settings,
        control_handler,
        world,
    })
}