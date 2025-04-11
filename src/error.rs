use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum GameError {
    Initialization(String),
    Update(String),
    Draw(String),
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::Initialization(msg) => write!(f, "Initialization Error: {}", msg),
            GameError::Update(msg) => write!(f, "Update Error: {}", msg),
            GameError::Draw(msg) => write!(f, "Render Error: {}", msg),
        }
    }
}