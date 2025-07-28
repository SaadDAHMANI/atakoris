use super::*;
use serde::{Deserialize, Serialize};
// ----------------------- Pipe -----------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Valve {
    pub id: usize,
    pub name: Option<String>,
    pub start: usize,
    pub end: usize,
    pub length: f64,
    pub diameter: f64,
    pub roughness: f64,
    pub minor_loss: f64,
    pub flow: Option<f64>,
    pub k_value: f64,
    //velocity : Option<f64>,
    pub status: LinkStatus,
    pub valvetype: ValveType,
}

impl Valve {
    //fn resistance(&self, flow :f64)->f64 {

    //    let rq : f64 = match self.valvetype {
    //       ValveType::CV => {
    //               if flow > 0.0 {self.k_value }
    //              else { 10.0f64.powi(15) }
    //      }
    //       _ => self.k_value
    //   };
    //  rq
    // }

    pub fn get_rq(&self, flow: f64) -> f64 {
        if self.status == LinkStatus::Open {
            let rq: f64 = match self.valvetype {
                ValveType::FCV => {
                    if flow > 0.0 {
                        self.k_value * flow
                    } else {
                        10.0f64.powi(15)
                    }
                }
                _ => self.k_value * flow,
            };
            rq
        } else {
            if flow < 0.000001 {
                return 10.0f64.powi(15);
            } else {
                return 10.0f64.powi(25);
            }
        }
    }
}

impl Link for Valve {
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
        None
    }

    fn link_type(&self) -> LinkType {
        LinkType::Valve(self.valvetype)
    }
    fn default_with(
        id: usize,
        start_node: usize,
        end_node: usize,
        _vertices: Option<Vec<Position>>,
    ) -> Self {
        ValveBuilder::new()
            .set_id(id)
            .set_start(start_node)
            .set_end(end_node)
            .build()
    }
    fn to_string(&self) -> String {
        format!(
            "id: {}, name: {:?}, type: {:?}, {:?}, From {}--->{} : diametre: {}",
            self.id,
            self.name,
            self.link_type(),
            self.valvetype,
            self.start,
            self.end,
            self.diameter
        )
    }
}
//--------------------------------------------

impl Default for Valve {
    fn default() -> Self {
        Valve {
            id: 0,
            name: None,
            start: 0,
            end: 1,
            length: 100.0,
            diameter: 100.0,
            roughness: 130.0,
            minor_loss: 0.0,
            flow: None,
            status: LinkStatus::Open,
            k_value: 0.0,
            valvetype: ValveType::GPV,
        }
    }
}

//-------------------------
#[derive(Debug, Clone)]
pub struct ValveBuilder {
    pub id: usize,
    pub name: Option<String>,
    pub start: usize,
    pub end: usize,
    pub length: f64,
    pub diameter: f64,
    pub roughness: f64,
    pub minor_loss: f64,
    pub flow: Option<f64>,
    pub k_value: f64,
    //velocity : Option<f64>,
    pub status: LinkStatus,
    pub valvetype: ValveType,
}

impl ValveBuilder {
    pub fn new() -> Self {
        ValveBuilder::default()
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
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

    pub fn set_k_value(mut self, k: f64) -> Self {
        self.k_value = k;
        self
    }

    pub fn set_valve_type(mut self, valv_type: ValveType) -> Self {
        self.valvetype = valv_type;
        self
    }

    pub fn build(self) -> Valve {
        Valve {
            id: self.id,
            name: self.name,
            start: self.start,
            end: self.end,
            length: self.length,
            diameter: self.diameter,
            roughness: self.roughness,
            minor_loss: self.minor_loss,
            flow: None,
            status: self.status,
            k_value: self.k_value,
            valvetype: self.valvetype,
        }
    }
}

impl Default for ValveBuilder {
    fn default() -> Self {
        ValveBuilder {
            id: 0,
            name: None,
            start: 0,
            end: 0,
            length: 100.0,
            diameter: 100.0,
            roughness: 130.0,
            minor_loss: 0.0,
            flow: None,
            status: LinkStatus::Open,
            k_value: 0.0,
            valvetype: ValveType::GPV,
        }
    }
}
