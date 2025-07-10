use super::*;
use serde::{Deserialize, Serialize};
//-----------------------------------Reservoir----------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservoir {
    pub id: usize,
    pub name: Option<String>,
    pub head: f64,
    pub pattern: Option<String>,
}

impl Node for Reservoir {
    fn get_id(&self) -> usize {
        self.id
    }

    fn node_type(&self) -> NodeType {
        NodeType::Reservoir
    }

    fn to_string(&self) -> String {
        format!(
            "id: {}, name: {:?}, category: {:?}, head: {}, pattern: {:?}",
            self.id,
            self.name,
            self.node_type(),
            self.head,
            self.pattern
        )
    }
}

//-------------Reservoir builder

pub struct ReservoirBuilder {
    pub id: usize,
    pub name: Option<String>,
    pub head: f64,
    pub pattern: Option<String>,
}

impl ReservoirBuilder {
    pub fn new() -> Self {
        ReservoirBuilder {
            id: 0,
            head: 0.0f64,
            name: None,
            pattern: None,
        }
    }

    pub fn set_id(&mut self, id: usize) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_head(&mut self, head: f64) -> &mut Self {
        self.head = head;
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn set_pattern(&mut self, pattern: &str) -> &mut Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    pub fn build(&self) -> Reservoir {
        Reservoir {
            id: self.id,
            head: self.head,
            name: self.name.clone(),
            pattern: self.pattern.clone(),
        }
    }
}
