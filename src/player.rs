use std::f32::consts::PI;
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{load_texture, Texture2D};
use macroquad::window::{screen_height, screen_width};

pub const PLAYER_SPEED: f32 = 5.0;
const PLAYER_SCALE: (f32, f32) = (32.0, 32.0);

#[derive(Clone)]
pub struct Player {
    pub pos: Vec2,
    pub sprite: Texture2D,
    pub rotation: f32
}

impl Player {
    
    pub async fn new() -> Result<Self, String> {
        
        // player texture
        let player = load_texture("assets/sprites/monke.png").await;
        if let Err(e) = player {
            return Err(format!("Failed to load texture files: {}", e));
        }
        let player = player.unwrap();
        
        Ok(Self {
            pos: vec2(0.0, 0.0),
            sprite: player,
            rotation: 0.0,
        })
    }

    pub fn apply_movement(&mut self, mut movement: Vec2, delta_time: u128) {
        if movement.length() > 0.0 {
            movement = movement.normalize() * (PLAYER_SPEED / delta_time as f32);
        }

        self.pos += movement;

        // hard cap position at window bounds
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        } else if self.pos.x > screen_width() - PLAYER_SCALE.0 {
            self.pos.x = screen_width() - PLAYER_SCALE.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        } else if self.pos.y > screen_height() - PLAYER_SCALE.0 {
            self.pos.y = screen_height() - PLAYER_SCALE.0;
        }
    }

    pub fn look_towards_mouse(&mut self) {
        let mouse_pos = mouse_position();

        let pos = vec2(self.pos.x + PLAYER_SCALE.0 / 2.0, self.pos.y + PLAYER_SCALE.1 / 2.0);

        let dx = mouse_pos.0 - pos.x;
        let dy = mouse_pos.1 - pos.y;
        
        self.rotation = dy.atan2(dx) + (PI / 2.0);
    }

    pub fn is_on_mouse(&self) -> bool {
        let mouse_pos = mouse_position();
        let pos = vec2(self.pos.x + PLAYER_SCALE.0 / 2.0, self.pos.y + PLAYER_SCALE.1 / 2.0);

        let dx = mouse_pos.0 - pos.x;
        let dy = mouse_pos.1 - pos.y;

        dx.abs() < PLAYER_SCALE.0 / 2.0 && dy.abs() < PLAYER_SCALE.1 / 2.0
    }
    
}