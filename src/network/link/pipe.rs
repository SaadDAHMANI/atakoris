use crate::network::Position;

use super::*;
use serde::{Deserialize, Serialize};
// ----------------------- Pipe -----------------------------
const CHW: f64 = 10.65;
// const CHW: f64 = 10.5088;
// const CHW: f64 = 10.6744;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pipe {
    pub id: usize,
    pub name: Option<String>,
    pub vertices: Option<Vec<Position>>,
    pub start: usize,
    pub end: usize,

    ///
    /// Pipe length in meter (m).
    pub length: f64,

    ///
    /// Pipe diameter in mm.
    ///
    pub diameter: f64,

    ///
    /// roughness = Chw if Hazen-Williams (Hw) formula is used.
    /// roughness = D-W roughness if Darcy-Weisbach formula is used.
    ///
    pub roughness: f64,
    pub minor_loss: f64,

    ///
    /// Pipe flow in m3/s (IS system)
    ///
    pub flow: Option<f64>,
    //velocity : Option<f64>,
    pub status: LinkStatus,
    pub check_valve: bool,
}

impl Pipe {
    pub fn headloss(&self) -> Option<f64> {
        let hl = match self.flow {
            Some(q) => Some(self.resistance() * q.abs().powf(1.852)),
            None => None,
        };
        hl
    }
    pub fn unit_headloss(&self) -> f64 {
        let uhl = match self.headloss() {
            Some(hl) => hl / self.length,
            None => 0.0000001f64,
        };
        uhl
    }

    pub fn resistance(&self) -> f64 {
        if self.status == LinkStatus::Open {
            (CHW * self.length)
                / (self.roughness.powf(1.852) * (self.diameter * 0.001).powf(4.8704))
        } else {
            99.99f64.powi(20)
        }
    }

    #[allow(dead_code)]
    pub fn get_r_of_q(&self, flow: f64) -> f64 {
        if self.status == LinkStatus::Open {
            if self.check_valve {
                if flow < 0.0 {
                    99.99f64.powi(20)
                } else {
                    (CHW * self.length)
                        / (self.roughness.powf(1.852) * (self.diameter * 0.001).powf(4.8704))
                }
            } else {
                (CHW * self.length)
                    / (self.roughness.powf(1.852) * (self.diameter * 0.001).powf(4.8704))
            }
        } else {
            99.99f64.powi(20)
        }
    }

    #[allow(dead_code)]
    pub fn velocity(&self) -> Option<f64> {
        let v = match self.flow {
            Some(q) => Some((4.0 * q) / (std::f64::consts::PI * (self.diameter * 0.001).powi(2))),
            None => None,
        };
        v
    }

    pub fn get_vertices(&self) -> Option<&Vec<Position>> {
        match &self.vertices {
            None => None,
            Some(vertices) => Some(&vertices),
        }
    }
}

impl Link for Pipe {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_start_node(&self) -> usize {
        self.start
    }

    fn get_end_node(&self) -> usize {
        self.end
    }

    fn get_vertices(&self) -> Option<&Vec<Position>> {
        match &self.vertices {
            None => None,
            Some(vrtxs) => Some(&vrtxs),
        }
    }

    fn get_diameter(&self) -> f64 {
        self.diameter
    }

    fn get_length(&self) -> f64 {
        self.length
    }

    fn link_type(&self) -> LinkType {
        LinkType::Pipe
    }

    fn to_string(&self) -> String {
        format!(
            "id: {}, name: {:?}, category: {:?} , {}--->{} : diametre: {}, length: {}, R: {}, flow: {:?}",
            self.id,
            self.name,
            self.link_type(),
            self.start,
            self.end,
            self.diameter,
            self.length,
            self.resistance(),
            self.flow
        )
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe {
            id: 0,
            name: None,
            vertices: None,
            start: 0,
            end: 1,
            length: 100.0,
            diameter: 100.0,
            roughness: 130.0,
            minor_loss: 0.0,
            flow: None,
            status: LinkStatus::Open,
            check_valve: false,
        }
    }
}

//-------------------------
#[derive(Debug, Clone)]
pub struct PipeBuilder {
    pub id: usize,
    pub name: Option<String>,
    pub vertices: Option<Vec<Position>>,
    pub start: usize,
    pub end: usize,
    pub length: f64,
    pub diameter: f64,
    pub roughness: f64,
    pub minor_loss: f64,
    pub flow: Option<f64>,
    //velocity : Option<f64>,
    pub status: LinkStatus,
    pub check_valve: bool,
}

impl PipeBuilder {
    pub fn new() -> Self {
        PipeBuilder::default()
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_vertices(mut self, vertices: Option<Vec<Position>>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn set_start(mut self, start_node: usize) -> Self {
        self.start = start_node;
        self
    }

    pub fn set_end(mut self, end_node: usize) -> Self {
        self.end = end_node;
        self
    }

    pub fn set_length(mut self, length: f64) -> Self {
        self.length = f64::max(1.0, length);
        self
    }

    pub fn set_diameter(mut self, diameter: f64) -> Self {
        self.diameter = f64::max(diameter, 0.001);
        self
    }

    pub fn set_roughness(mut self, roughness: f64) -> Self {
        self.roughness = f64::max(roughness, 0.00001);
        self
    }

    pub fn set_minorloss(mut self, minloss: f64) -> Self {
        self.minor_loss = f64::max(0.0, minloss);
        self
    }

    pub fn set_status(mut self, status: LinkStatus) -> Self {
        self.status = status;
        self
    }

    pub fn set_check_valve(mut self, check_valve: bool) -> Self {
        self.check_valve = check_valve;
        self
    }

    pub fn build(self) -> Pipe {
        Pipe {
            id: self.id,
            name: self.name,
            vertices: self.vertices,
            start: self.start,
            end: self.end,
            length: self.length,
            diameter: self.diameter,
            roughness: self.roughness,
            minor_loss: self.minor_loss,
            flow: None,
            status: self.status,
            check_valve: self.check_valve,
        }
    }
}

impl Default for PipeBuilder {
    fn default() -> Self {
        PipeBuilder {
            id: 0,
            name: None,
            vertices: None,
            start: 0,
            end: 0,
            length: 100.0,
            diameter: 100.0,
            roughness: 130.0,
            minor_loss: 0.0,
            flow: None,
            status: LinkStatus::Open,
            check_valve: false,
        }
    }
}
