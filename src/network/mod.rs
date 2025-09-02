//pub mod network;
use core::panic;
use serde::{Deserialize, Serialize};
pub mod link;
pub mod node;
pub mod position;

pub use node::junction::{Junction, JunctionBuilder};
pub use node::reservoir::{Reservoir, ReservoirBuilder};
pub use node::tank::{Tank, TankBuilder};

pub use link::pipe::Pipe;
pub use link::pump::Pump;
pub use link::valve::Valve;

pub use position::Position;

use super::parsers::inpfileparser::InpFileParser;
//------------------------------------------------
/*
FlowUnits::Lps => 0.001,
               FlowUnits::Afd => 0.014276394,
               FlowUnits::Cfs => 1.0,
               FlowUnits::Cmd => 1.0,
               FlowUnits::Cmh => 1.0 / 3600.0,
               FlowUnits::Gpm => 1.0,
               FlowUnits::Imgd => 1.0,
               FlowUnits::Lpm => 1.0,
               FlowUnits::Mgd => 1.0,
               FlowUnits::Mld => 1.0,
               FlowUnits::Cms => 1.0,
               */
pub(crate) const AFD_FACTOR: f64 = 0.014276394;
pub(crate) const LPS_FACTOR: f64 = 0.001;
pub(crate) const LPM_FACTOR: f64 = 0.001 / 60.0;
pub(crate) const CMH_FACTOR: f64 = 1.0 / 3600.0;
// -----------------------------------------------
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
    pub options: Options,
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
}

impl Default for Network {
    fn default() -> Self {
        Network {
            title: None,
            junctions: None,
            tanks: None,
            reservoirs: None,
            pipes: None,
            pumps: None,
            valves: None,
            options: Options::default(),
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
    pub options: Options,
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
            options: Options::default(),
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

    pub fn set_reservoirs(mut self, reservoirs: Option<Vec<Reservoir>>) -> Self {
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

    pub fn set_options(mut self, options: Options) -> Self {
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
