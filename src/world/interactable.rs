use std::fmt::Display;
use macroquad::prelude::*;
use crate::gamestate::{GameStateAction, GameStateError};
use crate::player::Player;
use crate::ui::tooltip::ToolTipCard;

#[derive(Clone, Debug)]
pub struct Interactable {
    pub name: String,
    pub tooltip: ToolTipCard,
    pub pos: Vec2,
    pub sprite: Texture2D,
    pub rotation: f32,

    interaction: fn(&mut Player) -> Result<GameStateAction, GameStateError>,
}

impl Interactable {
    pub fn new(
        name: String,
        tooltip: ToolTipCard,
        pos: Vec2,
        sprite: Texture2D,
        rotation: f32,
        interaction: fn(&mut Player) -> Result<GameStateAction, GameStateError>,
    ) -> Self {
        Self {
            name,
            tooltip,
            pos,
            sprite,
            rotation,
            interaction,
        }
    }

    pub fn interact(&mut self, player: &mut Player) -> Result<GameStateAction, GameStateError> {
        (self.interaction)(player)
    }

    pub fn is_mouse_over(&self) -> bool {
        let mouse_pos = mouse_position();
        let pos = vec2(self.pos.x + self.sprite.width() / 2.0, self.pos.y + self.sprite.height() / 2.0);

        let dx = mouse_pos.0 - pos.x;
        let dy = mouse_pos.1 - pos.y;

        dx * dx + dy * dy < (self.sprite.width() / 2.0).powi(2)
    }
}

impl Display for Interactable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interactable '{}'({})", self.name, self.pos)
    }
}