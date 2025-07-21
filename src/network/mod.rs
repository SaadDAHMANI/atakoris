//pub mod network;
use core::panic;
use serde::{Deserialize, Serialize};
pub mod link;
pub mod node;
pub mod position;

use node::junction::Junction;
use node::reservoir::Reservoir;
use node::tank::Tank;

use link::pipe::Pipe;
use link::pump::Pump;
use link::valve::Valve;

pub use position::Position;

use super::parsers::inpfileparser::InpFileParser;

include!("options.rs");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub title: Option<String>,
    pub junctions: Option<Vec<Junction>>,
    pub tanks: Option<Vec<Tank>>,
    pub reservoirs: Option<Vec<Reservoir>>,
    pub pipes: Option<Vec<Pipe>>,
    pub pumps: Option<Vec<Pump>>,
    pub valves: Option<Vec<Valve>>,
    pub options: Option<Options>,
}

impl Network {
    pub fn read_from_file(file: &str) -> Result<Network, std::io::Error> {
        let wdn = InpFileParser::new(&file).read()?;
        Ok(wdn)
    }

    pub fn get_pipes_resistances(&self) -> Option<Vec<f64>> {
        //resistance for pipes
        let _resistance = match self.pipes.clone() {
            Some(pipes) => {
                let mut _r = vec![0.0f64; pipes.len()];
                for j in 0..pipes.len() {
                    _r[j] = pipes[j].resistance();
                }
                Some(_r)
            }
            None => None,
        };
        _resistance
    }

    //#[cfg(feature = "optimization")]
    pub fn update_pipes_diameters(&mut self, diameters: &[f64]) {
        match &mut self.pipes {
            None => {}
            Some(pipes) => {
                if diameters.len() < pipes.len() {
                    panic!("no sufficient diameters to update network !!!")
                }

                let mut i: usize = 0;
                for p in pipes.iter_mut() {
                    p.diameter = diameters[i];
                    //p.set_diameter(diameters[i].clone());
                    i += 1;
                }
            }
        };
    }

    pub fn get_empty() -> Self {
        Network {
            title: None,
            junctions: None,
            tanks: None,
            reservoirs: None,
            pipes: None,
            pumps: None,
            valves: None,
            options: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkBuilder {
    pub title: Option<String>,
    pub junctions: Option<Vec<Junction>>,
    pub tanks: Option<Vec<Tank>>,
    pub reservoirs: Option<Vec<Reservoir>>,
    pub pipes: Option<Vec<Pipe>>,
    pub pumps: Option<Vec<Pump>>,
    pub valves: Option<Vec<Valve>>,
    pub options: Option<Options>,
}

impl NetworkBuilder {
    pub fn new() -> Self {
        let wdnb = NetworkBuilder {
            title: None,
            junctions: None,
            tanks: None,
            reservoirs: None,
            pipes: None,
            pumps: None,
            valves: None,
            options: None,
        };

        wdnb
    }

    pub fn set_title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    pub fn set_junctions(mut self, junctions: Option<Vec<Junction>>) -> Self {
        self.junctions = junctions;
        self
    }

    pub fn set_tanks(mut self, tanks: Option<Vec<Tank>>) -> Self {
        self.tanks = tanks;
        self
    }

    pub fn set_reservoir(mut self, reservoirs: Option<Vec<Reservoir>>) -> Self {
        self.reservoirs = reservoirs;
        self
    }

    pub fn set_pipes(mut self, pipes: Option<Vec<Pipe>>) -> Self {
        self.pipes = pipes;
        self
    }

    pub fn set_pumps(mut self, pumps: Option<Vec<Pump>>) -> Self {
        self.pumps = pumps;
        self
    }

    pub fn set_valves(mut self, valves: Option<Vec<Valve>>) -> Self {
        self.valves = valves;
        self
    }

    pub fn set_options(mut self, options: Option<Options>) -> Self {
        self.options = options;
        self
    }

    pub fn build(self) -> Network {
        Network {
            title: self.title,
            junctions: self.junctions,
            tanks: self.tanks,
            reservoirs: self.reservoirs,
            pipes: self.pipes,
            pumps: self.pumps,
            valves: self.valves,
            options: self.options,
        }
    }
}
