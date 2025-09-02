use super::*;
use serde::{Deserialize, Serialize};
//-----------------------------------Junction----------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Junction {
    pub id: usize,
    pub position: Position,
    pub elevation: f64,
    pub demand: f64,
    pub pattern: Option<usize>,
    pub name: Option<String>,
    pub head: Option<f64>,
    #[cfg(feature = "optimization")]
    target_head: Option<f64>,
    // pressure: Option<f64>,
}

impl Junction {
    pub fn new(id: usize, elevation: f64, demand: f64) -> Self {
        Self {
            id,
            position: Position::default(),
            elevation,
            demand,
            name: None,
            head: None,
            pattern: None,
            #[cfg(feature = "optimization")]
            target_head: None,
        }
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    #[cfg(feature = "optimization")]
    pub fn set_target_head(&mut self, required_head: f64) {
        self.target_head = Some(required_head);
    }

    #[cfg(feature = "optimization")]
    pub fn get_target_head(&self) -> Option<f64> {
        self.target_head
    }
}

impl Node for Junction {
    fn get_id(&self) -> usize {
        self.id
    }

    fn default_with(id: usize, pos: Position) -> Self {
        let mut nd = Junction::default();
        nd.id = id;
        nd.position = pos;
        nd
    }
    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn node_type(&self) -> NodeType {
        NodeType::Junction
    }
    fn to_string(&self) -> String {
        format!(
            "id: {}, categ.: {:?}, demand: {}, elev.: {}, name: {:?}, head: {:?}, pressure: {:?}",
            self.id,
            self.node_type(),
            self.demand,
            self.elevation,
            self.name,
            self.head,
            self.pressure()
        )
    }

    fn get_elevation(&self) -> f64 {
        self.elevation
    }

    fn get_head(&self) -> Option<f64> {
        self.head
    }
}

impl Default for Junction {
    fn default() -> Self {
        Self::new(0, 0.0f64, 0.0f64)
    }
}

pub struct JunctionBuilder {
    id: usize,
    position: Position,
    elevation: f64,
    demand: f64,
    pattern: Option<usize>,
    name: Option<String>,
    head: Option<f64>,
    #[cfg(feature = "optimization")]
    target_head: Option<f64>,
}
impl JunctionBuilder {
    pub fn new() -> Self {
        let jb = JunctionBuilder {
            id: 0,
            position: Position::default(),
            elevation: 0.0f64,
            demand: 0.0f64,
            pattern: None,
            name: None,
            head: None,
            #[cfg(feature = "optimization")]
            target_head: None,
        };
        jb
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_position(mut self, pos: Position) -> Self {
        self.position = pos;
        self
    }

    pub fn set_elevation(mut self, elevation: f64) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn set_demand(mut self, demand: f64) -> Self {
        self.demand = demand;
        self
    }

    pub fn set_pattern(mut self, pattern: Option<usize>) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    #[cfg(feature = "optimization")]
    pub fn set_target_head(mut self, required_head: f64) -> Self {
        self.target_head = Some(required_head);
        self
    }

    pub fn build(self) -> Junction {
        Junction {
            id: self.id,
            position: self.position,
            name: self.name,
            elevation: self.elevation,
            demand: self.demand,
            head: self.head,
            pattern: self.pattern,

            #[cfg(feature = "optimization")]
            target_head: self.target_head,
        }
    }
}
