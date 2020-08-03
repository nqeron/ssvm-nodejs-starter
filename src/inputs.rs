
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SearchQueryInput{
    pub player: PlayerColor,
    pub difficulty: String,
}

#[derive(Serialize, Deserialize)]
pub enum PlayerColor{
    WHITE,
    BLACK
}

impl PlayerColor{
    
    pub fn as_str(&self) -> &str {
        match self {
            PlayerColor::WHITE => "WHITE",
            PlayerColor::BLACK => "BLACK"
        }
    }
}