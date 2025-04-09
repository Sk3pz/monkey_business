use std::fmt::Display;
use std::time::Duration;
use crate::gamedata::GameData;

pub mod pause;

pub enum OverlayAction {
    NoOp,
    Exit,
    SpawnOverlay(Box<dyn Overlay>),
}

#[derive(Debug, Clone)]
pub enum OverlayError {
    Initialization(String),
    Update(String),
    Draw(String),
}

impl Display for OverlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayError::Initialization(msg) => write!(f, "Overlay Initialization Error: {}", msg),
            OverlayError::Update(msg) => write!(f, "Overlay Update Error: {}", msg),
            OverlayError::Draw(msg) => write!(f, "Overlay Render Error: {}", msg),
        }
    }
}

pub trait Overlay {

    fn update(&mut self, delta_time: &Duration, data: &mut GameData) -> Result<OverlayAction, OverlayError>;
    fn draw(&self, data: &mut GameData) -> Result<(), OverlayError>;
    fn draw_below(&self) -> bool {
        false
    }
}

pub struct OverlayManager {
    overlays: Vec<Box<dyn Overlay>>,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self {
            overlays: Vec::new(),
        }
    }

    pub fn push(&mut self, overlay: Box<dyn Overlay>) {
        self.overlays.push(overlay);
    }

    pub fn pop(&mut self) {
        self.overlays.pop();
    }

    pub fn get_top(&self) -> Option<&Box<dyn Overlay>> {
        self.overlays.last()
    }

    pub fn get_top_mut(&mut self) -> Option<&mut Box<dyn Overlay>> {
        self.overlays.last_mut()
    }

    /// returns None if there are no overlays to update (signaling to update the gamestate)
    pub fn update(&mut self, delta_time: &Duration, data: &mut GameData) -> Option<Result<OverlayAction, OverlayError>> {
        if let Some(overlay) = self.get_top_mut() {
            Some(overlay.update(delta_time, data))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.overlays.len()
    }

    pub fn should_draw_gamestate(&self) -> bool {
        let draw = true;

        for o in self.overlays.iter().rev() {
            if !o.draw_below() {
                return false;
            }
        }

        draw
    }

    /// draws overlays until it reaches the bottom or an overlay that does not draw below
    /// @return true if the gamestate should be drawn
    pub fn draw(&self, data: &mut GameData) -> Result<(), OverlayError> {
        // get how deep to draw (until an overlay that does not draw below or until there are no overlays)
        let draw_from = self.overlays.iter().rev()
            .position(|o| !o.draw_below())
            .map_or(0, |pos| self.overlays.len() - pos - 1);

        // draw from bottom up
        for overlay in &self.overlays[draw_from..self.overlays.len()] {
            overlay.draw(data)?;
        }

        Ok(())
    }
}