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

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn set_junctions(&mut self, junctions: Vec<Junction>) -> &mut Self {
        self.junctions = Some(junctions);
        self
    }

    pub fn set_tanks(&mut self, tanks: Vec<Tank>) -> &mut Self {
        self.tanks = Some(tanks);
        self
    }

    pub fn set_reservoir(&mut self, reservoirs: Vec<Reservoir>) -> &mut Self {
        self.reservoirs = Some(reservoirs);
        self
    }

    pub fn set_pipes(&mut self, pipes: Vec<Pipe>) -> &mut Self {
        self.pipes = Some(pipes);
        self
    }

    pub fn set_pumps(&mut self, pumps: Vec<Pump>) -> &mut Self {
        self.pumps = Some(pumps);
        self
    }

    pub fn set_valves(&mut self, valves: Vec<Valve>) -> &mut Self {
        self.valves = Some(valves);
        self
    }

    pub fn set_options(&mut self, options: Options) -> &mut Self {
        self.options = Some(options);
        self
    }

    pub fn build(&self) -> Network {
        // let nn : usize = match self.junctions {
        //     Some(junctions) => junctions.len(),
        //     None => 0,
        // };

        // let nt = match self.tanks{
        //     Some(tanks) => tanks.len(),
        //     None => 0,
        // };

        // let nr = match self.reservoirs{
        //     Some(reservoirs) => reservoirs.len(),
        //     None => 0,
        // };

        // let npip = match self.pipes{
        //     Some(pipes) => pipes.len(),
        //     None =>0,
        // };

        // let npmp = match self.pumps {
        //     Some(pumps) => pumps.len(),
        //     None => 0,
        // };

        // let nvlv = match self.valves {
        //     Some(valves) => valves.len(),
        //     None => 0,
        // };

        Network {
            title: self.title.clone(),
            junctions: self.junctions.clone(),
            tanks: self.tanks.clone(),
            reservoirs: self.reservoirs.clone(),
            pipes: self.pipes.clone(),
            pumps: self.pumps.clone(),
            valves: self.valves.clone(),
            options: self.options.clone(),
        }
    }
}
