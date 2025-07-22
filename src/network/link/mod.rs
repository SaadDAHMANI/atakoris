use serde::{Deserialize, Serialize};

use super::Position;

pub mod pipe;
pub mod pump;
pub mod valve;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum LinkType {
    Pipe,
    Pump,
    Valve(ValveType),
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum LinkStatus {
    Open,
    Closed,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ValveType {
    FCV,
    PBV,
    PRV,
    TCV,
    PSV,
    GPV,
}

pub trait Link {
    fn link_type(&self) -> LinkType;
    fn get_id(&self) -> usize;
    fn get_vertices(&self) -> Option<&Vec<Position>>;
    fn get_start_node(&self) -> usize;
    fn get_end_node(&self) -> usize;

    //fn resistance(&self)->f64;
    fn to_string(&self) -> String;
    fn print(&self) {
        println!("{}", self.to_string());
    }
}
