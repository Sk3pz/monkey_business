use std::collections::HashMap;
use macroquad::math::Rect;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D, Vec2, WHITE};
use crate::util::get_sprite_scale;

#[derive(Clone, Debug)]
pub struct Animation {
    pub start_frame: usize,
    pub frame_count: usize,
    pub frame_time: f32,
    pub looping: bool,
}

#[derive(Clone, Debug)]
pub struct Animator {
    pub texture: Texture2D,
    pub frame_size: Vec2,
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
    pub current_frame: usize,
    pub timer: f32,
    pub playing: bool,
}

impl Animator {
    pub fn new(texture: Texture2D, frame_size: Vec2) -> Self {
        Self {
            texture,
            frame_size,
            animations: HashMap::new(),
            current_animation: String::new(),
            current_frame: 0,
            timer: 0.0,
            playing: true,
        }
    }

    pub fn add_animation(&mut self, name: &str, start_frame: usize, frame_count: usize, frame_time: f32, looping: bool) {
        self.animations.insert(name.to_string(), Animation {
            start_frame,
            frame_count,
            frame_time,
            looping,
        });

        // Auto-set first added animation as current
        if self.current_animation.is_empty() {
            self.set_animation(name);
        }
    }

    pub fn set_animation(&mut self, name: &str) {
        if self.current_animation == name { return; }
        if self.animations.contains_key(name) {
            self.current_animation = name.to_string();
            self.current_frame = 0;
            self.timer = 0.0;
            self.playing = true;
        }
    }

    pub fn step(&mut self) {
        if let Some(anim) = self.animations.get(&self.current_animation) {
            self.current_frame += 1;

            if self.current_frame >= anim.frame_count {
                if anim.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = anim.frame_count - 1;
                    self.playing = false;
                }
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.playing { return; }

        if let Some(anim) = self.animations.get(&self.current_animation) {
            self.timer += delta;
            if self.timer >= anim.frame_time {
                self.timer -= anim.frame_time;
                self.step();
            }
        }
    }

    pub fn draw(&self, position: Vec2, rotation: Option<f32>, override_scale: Option<Vec2>) {
        if let Some(anim) = self.animations.get(&self.current_animation) {
            let frame_index = anim.start_frame + self.current_frame;
            let cols = (self.texture.width() / self.frame_size.x) as usize;
            let frame_x = (frame_index % cols) as f32 * self.frame_size.x;
            let frame_y = (frame_index / cols) as f32 * self.frame_size.y;

            let scale = if let Some(scale) = override_scale {
                scale
            } else {
                get_sprite_scale()
            };

            draw_texture_ex(
                &self.texture,
                position.x,
                position.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect {
                        x: frame_x,
                        y: frame_y,
                        w: self.frame_size.x,
                        h: self.frame_size.y,
                    }),
                    rotation: rotation.unwrap_or(0.0),
                    //dest_size: scale,
                    dest_size: Some(scale),
                    ..Default::default()
                },
            );
        }
    }
}