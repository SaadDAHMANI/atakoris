use serde::{Deserialize, Serialize};
pub mod junction;
pub mod reservoir;
pub mod tank;

use super::Position;

//use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Junction,
    Reservoir,
    Tank,
}

//------------------------------------Node-------------------------------

pub trait Node {
    fn get_id(&self) -> usize;

    fn node_type(&self) -> NodeType;

    fn get_position(&self) -> Position {
        Position::new(0.0, 0.0)
    }

    fn get_head(&self) -> Option<f64> {
        None
    }

    fn get_elevation(&self) -> f64 {
        0.0f64
    }

    fn pressure(&self) -> Option<f64> {
        match self.get_head() {
            None => None,
            Some(h) => Some(h - self.get_elevation()),
        }
    }

    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String;
}
