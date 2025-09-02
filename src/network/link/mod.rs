use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::FlowUnits;

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

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
pub enum LinkStatus {
    Open,
    Closed,
}

impl Display for LinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValveType {
    FCV,
    PBV,
    PRV,
    TCV,
    PSV,
    GPV,
}

impl Display for ValveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FCV => write!(f, "FCV"),
            Self::PBV => write!(f, "PBV"),
            Self::PRV => write!(f, "PRV"),
            Self::TCV => write!(f, "TCV"),
            Self::PSV => write!(f, "PSV"),
            Self::GPV => write!(f, "GPV"),
        }
    }
}

pub trait Link {
    fn default_with(
        id: usize,
        start_node: usize,
        end_node: usize,
        vertices: Option<Vec<Position>>,
    ) -> Self;
    fn link_type(&self) -> LinkType;
    fn get_id(&self) -> usize;
    fn get_vertices(&self) -> Option<&Vec<Position>>;
    fn get_start_node(&self) -> usize;
    fn get_end_node(&self) -> usize;
    fn get_diameter(&self) -> f64 {
        100.0f64
    }

    fn get_length(&self) -> f64 {
        0.0001f64
    }

    //fn resistance(&self)->f64;
    fn to_string(&self) -> String;
    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn set_flow_unit(&mut self, flow_unit: FlowUnits);
    fn get_flow_unit(&self) -> FlowUnits;
}
