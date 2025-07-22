use super::*;
use serde::{Deserialize, Serialize};
// ----------------------- Pump -----------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pump {
    pub id: usize,
    pub name: Option<String>,
    pub start: usize,
    pub end: usize,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub flow: Option<f64>,
    //velocity : Option<f64>,
    pub status: LinkStatus,
    pub parameters: Option<String>,
}

impl Pump {
    #[allow(dead_code)]
    fn head_of(&self, flow: f64) -> f64 {
        return self.alpha * flow.powi(2) + self.beta * flow + self.gamma;
    }

    fn head(&self) -> Option<f64> {
        let _hq = match self.flow {
            Some(q) => Some(self.alpha * q.powi(2) + self.beta * q + self.gamma),
            None => None,
        };
        _hq
    }

    pub fn get_rq(&self, flow: f64) -> f64 {
        if self.status == LinkStatus::Open {
            self.alpha * flow + self.beta + (self.gamma / flow)
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

    fn to_string(&self) -> String {
        format!(
            "id: {}, name: {:?}, category: {:?} , {}--->{}, alpha: {}, beta: {}, gamma: {}, Q: {:?}, H: {:?}",
            self.id,
            self.name,
            self.link_type(),
            self.start,
            self.end,
            self.alpha,
            self.beta,
            self.gamma,
            self.flow,
            self.head()
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
    pub status: LinkStatus,
    pub parameters: Option<String>,
}

impl PumpBuilder {
    pub fn new() -> Self {
        PumpBuilder::default()
    }

    pub fn set_id(&mut self, id: usize) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_start(&mut self, start_node: usize) -> &mut Self {
        self.start = start_node;
        self
    }

    pub fn set_end(&mut self, end_node: usize) -> &mut Self {
        self.end = end_node;
        self
    }

    pub fn set_alpha(&mut self, alpha: f64) -> &mut Self {
        self.alpha = alpha;
        self
    }

    pub fn set_beta(&mut self, beta: f64) -> &mut Self {
        self.beta = beta;
        self
    }

    pub fn set_gamma(&mut self, gamma: f64) -> &mut Self {
        self.gamma = gamma;
        self
    }

    pub fn set_status(&mut self, status: LinkStatus) -> &mut Self {
        self.status = status;
        self
    }

    pub fn set_parameters(&mut self, parameters: Option<String>) -> &mut Self {
        self.parameters = parameters;
        self
    }

    pub fn build(&self) -> Pump {
        Pump {
            id: self.id,
            name: self.name.clone(),
            start: self.start,
            end: self.end,
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
            flow: None,
            status: self.status,
            parameters: self.parameters.clone(),
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
            status: LinkStatus::Open,
            parameters: None,
        }
    }
}
