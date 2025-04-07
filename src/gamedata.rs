use crate::assets::GlobalAssets;
use crate::controls::ControlHandler;
use crate::world::World;

pub struct GameData {
    // Render Data
    pub fps: f32,
    // Global Data
    pub assets: GlobalAssets,
    pub control_handler: ControlHandler,
    pub world: World,
}

impl GameData {

    pub fn reload_controls(&mut self) -> Result<(), String> {
        let control_handler = ControlHandler::load();
        if let Err(e) = control_handler {
            return Err(format!("Failed to load control handler: {}", e));
        }
        self.control_handler = control_handler?;
        Ok(())
    }

}