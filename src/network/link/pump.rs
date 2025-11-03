use crate::FLOW_EPSILON;

use super::*;
use serde::{Deserialize, Serialize};
// ----------------------- Pump -----------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pump {
    pub id: usize,
    pub name: Option<String>,
    pub start: usize,
    pub end: usize,

    /// alpha x Q^2 + beta x Q + gamma.
    pub alpha: f64,

    /// alpha x Q^2 + beta x Q + gamma.
    pub beta: f64,

    /// alpha x Q^2 + beta x Q + gamma.
    pub gamma: f64,

    /// Power rating in KW.
    pub power_rating: f64,

    pub flow: Option<f64>,

    /// Velocity : Option<f64>,
    pub status: LinkStatus,
    pub parameters: Option<String>,
    pub flow_unit: FlowUnits,
}

impl Pump {
    fn head_of(&mut self, flow: f64, flow_unit_multiplier: f64) -> f64 {
        if flow > FLOW_EPSILON {
            if self.alpha != 0.0 {
                return self.alpha * (flow / flow_unit_multiplier).powi(2)
                    + self.beta * (flow / flow_unit_multiplier)
                    + self.gamma;
            } else {
                return self.power_rating / (9.81 * f64::max(flow.abs(), FLOW_EPSILON));
            }
        } else {
            self.status = LinkStatus::Closed;
            FLOW_EPSILON
        }
    }

    fn head(&self, flow_unit_multiplier: f64) -> Option<f64> {
        let _hq = match self.flow {
            Some(q) => {
                if self.alpha != 0.0 {
                    Some(
                        self.alpha * (q / flow_unit_multiplier).powi(2)
                            + self.beta * (q / flow_unit_multiplier)
                            + self.gamma,
                    )
                } else {
                    Some(self.power_rating / (9.81 * f64::max(q.abs(), FLOW_EPSILON)))
                }
            }

            None => None,
        };
        _hq
    }

    /// Compute the generated head/Q
    pub fn get_r_of_q0(&self, flow: f64, flow_unit_multiplier: f64) -> f64 {
        if self.status == LinkStatus::Open {
            if self.alpha != 0.0 {
                self.alpha * (flow / flow_unit_multiplier)
                    + self.beta
                    + (self.gamma * flow_unit_multiplier / flow)
            } else {
                let q = f64::max(flow.abs(), FLOW_EPSILON);
                self.power_rating / (9.81 * q.powi(2))
            }
        } else {
            10.00f64.powi(20)
        }
    }
}

impl Link for Pump {
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
        LinkType::Pump
    }

    fn set_flow_unit(&mut self, flow_unit: FlowUnits) {
        self.flow_unit = flow_unit;
    }

    fn get_flow_unit(&self) -> FlowUnits {
        self.flow_unit
    }
    fn default_with(
        id: usize,
        start_node: usize,
        end_node: usize,
        _vertices: Option<Vec<Position>>,
    ) -> Self {
        PumpBuilder::new()
            .set_id(id)
            .set_start(start_node)
            .set_end(end_node)
            .build()
    }

    fn to_string(&self) -> String {
        format!(
            "id: {}, name: {:?}, category: {:?} , {}--->{}, alpha: {}, beta: {}, gamma: {}, power_rating: {}, Q: {:?}.",
            self.id,
            self.name,
            self.link_type(),
            self.start,
            self.end,
            self.alpha,
            self.beta,
            self.gamma,
            self.power_rating,
            self.flow,
        )
    }
}

//------------------------------------------

#[derive(Debug, Clone)]
pub struct PumpBuilder {
    pub id: usize,
    pub name: Option<String>,
    pub start: usize,
    pub end: usize,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub power_rating: f64,
    pub status: LinkStatus,
    pub parameters: Option<String>,
    pub flow_unit: FlowUnits,
}

impl PumpBuilder {
    pub fn new() -> Self {
        PumpBuilder::default()
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
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

    pub fn set_alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn set_beta(mut self, beta: f64) -> Self {
        self.beta = beta;
        self
    }

    pub fn set_gamma(mut self, gamma: f64) -> Self {
        self.gamma = gamma;
        self
    }

    /// Set the power rating in KW.
    pub fn set_power_rating(mut self, power_kw: f64) -> Self {
        self.gamma = power_kw;
        self
    }

    pub fn set_status(mut self, status: LinkStatus) -> Self {
        self.status = status;
        self
    }

    pub fn set_parameters(mut self, parameters: Option<String>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn set_flow_unit(mut self, flow_unit: FlowUnits) -> Self {
        self.flow_unit = flow_unit;
        self
    }

    pub fn build(self) -> Pump {
        Pump {
            id: self.id,
            name: self.name,
            start: self.start,
            end: self.end,
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
            power_rating: self.power_rating,
            flow: None,
            status: self.status,
            parameters: self.parameters,
            flow_unit: self.flow_unit,
        }
    }
}

impl Default for PumpBuilder {
    fn default() -> Self {
        PumpBuilder {
            id: 0,
            name: None,
            start: 0,
            end: 0,
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
            power_rating: 0.0,
            status: LinkStatus::Open,
            parameters: None,
            flow_unit: FlowUnits::default(),
        }
    }
}
