

pub enum PopupLocation {
    Top, Bottom, BottomLeft, BottomRight,
}

pub struct PopupCard {
    pub title: String,
    pub lines: Vec<String>,
    pub location: PopupLocation,
}

impl PopupCard {
    pub fn new(title: String, lines: Vec<String>, location: PopupLocation) -> Self {
        Self { title, lines, location }
    }
}