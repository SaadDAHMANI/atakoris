use super::*;
use serde::{Deserialize, Serialize};
//-----------------------------------Tank-------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tank {
    pub id: usize,
    pub position: Position,
    pub name: Option<String>,
    pub elevation: f64,
    //pub head : Option<f64>,
    pub initial_level: f64,
    //pub min_level : f64,
    //pub max_level : f64,
    //pub diameter : f64,
    //pub min_volume : f64,
    //pub volume_curve_id : Option<usize>,
    //pub overflow_indicator : bool,
}

impl Tank {
    pub fn new(id: usize, elevation: f64, initial_level: f64) -> Self {
        Self {
            id,
            position: Position::default(),
            elevation,
            initial_level,
            name: None,
        }
    }

    pub fn head(&self) -> f64 {
        self.elevation + self.initial_level
    }
}

impl Node for Tank {
    fn get_id(&self) -> usize {
        self.id
    }

    fn default_with(id: usize, pos: Position) -> Self {
        let mut nd = Tank::default();
        nd.id = id;
        nd.position = pos;
        nd
    }
    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn node_type(&self) -> NodeType {
        NodeType::Tank
    }
    fn to_string(&self) -> String {
        format!(
            "id: {}, name: {:?}, ategory: {:?}, elevation: {}, initial-level: {:?}",
            self.id,
            self.name,
            self.node_type(),
            self.elevation,
            self.initial_level
        )
    }
}

impl Default for Tank {
    fn default() -> Self {
        Tank::new(0usize, 100.0f64, 2.0f64)
    }
}

pub struct TankBuilder {
    pub id: usize,
    pub position: Position,
    pub name: Option<String>,
    pub elevation: f64,
    //pub head : Option<f64>,
    pub initial_level: f64,
    //pub min_level : f64,
    //pub max_level : f64,
    //pub diameter : f64,
    //pub min_volume : f64,
    //pub volume_curve_id : Option<usize>,
    //pub overflow_indicator : bool,
}

impl TankBuilder {
    pub fn new() -> Self {
        TankBuilder {
            id: 0,
            position: Position::default(),
            name: None,
            elevation: 0.0f64,
            initial_level: 0.0f64,
        }
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_position(mut self, pos: Position) -> Self {
        self.position = pos;
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn set_elevation(mut self, elevation: f64) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn set_initial_level(mut self, initial_level: f64) -> Self {
        self.initial_level = initial_level;
        self
    }

    pub fn build(self) -> Tank {
        Tank {
            id: self.id,
            position: self.position,
            name: self.name,
            elevation: self.elevation,
            initial_level: self.initial_level,
        }
    }
}
