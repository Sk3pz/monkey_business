use macroquad::prelude::*;

#[derive(Clone)]
pub struct GlobalAssets {
    pub font: Font,
}

impl GlobalAssets {

    pub async fn load() -> Result<Self, String> {
        let Ok(font) = load_ttf_font("./assets/fonts/PressStart2P-Regular.ttf").await else {
            return Err("Failed to load font".to_string());
        };

        Ok(Self {
            font,
        })
    }

}