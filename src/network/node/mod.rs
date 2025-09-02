use serde::{Deserialize, Serialize};
pub mod junction;
pub mod reservoir;
pub mod tank;

pub use junction::{Junction, JunctionBuilder};
pub use reservoir::{Reservoir, ReservoirBuilder};
pub use tank::{Tank, TankBuilder};

pub use super::{FlowUnits, Position};

//use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum NodeType {
    Junction,
    Reservoir,
    Tank,
}

//------------------------------------Node-------------------------------

pub trait Node {
    fn default_with(id: usize, pos: Position, flow_unit: FlowUnits) -> Self;

    fn get_id(&self) -> usize;

    fn node_type(&self) -> NodeType;

    fn get_position(&self) -> (f32, f32) {
        (0.0f32, 0.0f32)
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

    fn set_flow_unit(&mut self, flow_unit: FlowUnits);

    fn get_flow_unit(&self) -> FlowUnits;

    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String;
}
