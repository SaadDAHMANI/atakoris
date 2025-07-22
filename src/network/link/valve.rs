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
