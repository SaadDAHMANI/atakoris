use std::fs::File;
use std::io::prelude::*;

use crate::network::Network;
use crate::network::NetworkBuilder;
use crate::network::Position;
use crate::network::link::LinkStatus;
use crate::network::link::pipe::*;
use crate::network::link::pump::*;
use crate::network::node::junction::*;
use crate::network::node::reservoir::*;
use crate::network::node::tank::*;
use crate::network::{FlowUnits, HeadlossFormula, Options, OptionsBuilder, link::Link, node::Node};

///
/// This is a parser for "*.inp" files (Epanet file format)
/// Look at : http://wateranalytics.org/EPANET/_inp_file.html
///
pub struct InpFileParser<'a> {
    pub file_path: &'a str,
}

impl<'a> InpFileParser<'a> {
    pub fn new(file_path: &'a str) -> Self {
        InpFileParser { file_path }
    }

    pub fn read(&self) -> Result<Network, std::io::Error> {
        match self.get_lines().as_ref() {
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error when reading the file : {}", self.file_path),
            )),
            Some(lines) => {
                // println!("lines: {}", lines.len());

                let title = self.get_title(&lines);
                //  println!("title : {:?}", title);
                let mut junctions = self.get_junctions(&lines);
                let mut tanks = self.get_tanks(&lines);
                let mut reservoirs = self.get_reservoirs(&lines);
                let mut pipes = self.get_pipes(&lines);
                let mut pumps = self.get_pumps(&lines);
                let options = self.get_options(&lines).unwrap_or_default();

                let node_positions = self.get_coordinates(&lines);
                let link_vertices = self.get_vertices(lines);

                match node_positions {
                    None => (),
                    Some(positions) => {
                        // set junctions and their positions:
                        match junctions.as_mut() {
                            None => (),
                            Some(nodes) => {
                                for jn in nodes.iter_mut() {
                                    match positions.iter().find(|p| p.0 == jn.id) {
                                        None => (),
                                        Some(pos) => jn.position = pos.1.clone(),
                                    }
                                }
                            }
                        };
                        // ------------------------------
                        // set tanks and their positions

                        match tanks.as_mut() {
                            None => (),
                            Some(tanks) => {
                                for tnk in tanks.iter_mut() {
                                    match positions.iter().find(|p| p.0 == tnk.id) {
                                        None => (),
                                        Some(pos) => tnk.position = pos.1.clone(),
                                    }
                                }
                            }
                        };
                        //---------------------------
                        // set reservoirs and  positions

                        match reservoirs.as_mut() {
                            None => (),
                            Some(reservoirs) => {
                                for rsrvr in reservoirs.iter_mut() {
                                    match positions.iter().find(|p| p.0 == rsrvr.id) {
                                        None => (),
                                        Some(pos) => rsrvr.position = pos.1.clone(),
                                    };
                                }
                            }
                        };
                    }
                };

                match &link_vertices {
                    None => (),
                    Some(vertices) => {
                        match pipes.as_mut() {
                            None => (),
                            Some(pipes) => {
                                for pipe in pipes.iter_mut() {
                                    // pipe.vertices =
                                    let vrtxs: Vec<Position> = vertices
                                        .iter()
                                        .filter(|v| v.0 == pipe.id)
                                        .map(|(_i, pos)| pos.clone())
                                        .collect();
                                    if vrtxs.len() > 0 {
                                        pipe.vertices = Some(vrtxs);
                                    }
                                }
                            }
                        };
                    }
                }
                /*
                        match pumps {
                            None => (),
                            Some(pmps) => {}
                        };

                        match optns {
                            None => (),
                            Some(optins) => {}
                        };
                */
                // update node and pipe flow_unit:
                if let Some(nodes) = &mut junctions {
                    nodes
                        .iter_mut()
                        .for_each(|nd| nd.set_flow_unit(options.flow_unit));
                };

                if let Some(nodes) = &mut tanks {
                    nodes
                        .iter_mut()
                        .for_each(|nd| nd.set_flow_unit(options.flow_unit));
                };

                if let Some(nodes) = &mut reservoirs {
                    nodes
                        .iter_mut()
                        .for_each(|nd| nd.set_flow_unit(options.flow_unit));
                };

                if let Some(edges) = &mut pipes {
                    edges
                        .iter_mut()
                        .for_each(|lnk| lnk.set_flow_unit(options.flow_unit));
                };

                if let Some(edges) = &mut pumps {
                    edges
                        .iter_mut()
                        .for_each(|lnk| lnk.set_flow_unit(options.flow_unit));
                };

                let wdn = NetworkBuilder::new()
                    .set_title(title)
                    .set_junctions(junctions)
                    .set_reservoirs(reservoirs)
                    .set_tanks(tanks)
                    .set_pipes(pipes)
                    .set_pumps(pumps)
                    .set_valves(None)
                    .set_options(options)
                    .build();

                Ok(wdn)
            }
        }
    }

    fn get_content(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(self.file_path)?; //.expect("Cannot open this file!");
        let mut the_content = String::new();
        file.read_to_string(&mut the_content)?; //.expect("Cannot read the file !!!"); 

        Ok(the_content)
        //self.content=Some(the_content);
    }

    fn get_lines(&self) -> Option<Vec<String>> {
        //let blocks :Vec<String> = Vec::new();
        match self.get_content() {
            Err(_err) => None,
            Ok(x) => {
                let y: Vec<&str> = x.split('\n').collect();

                let lines = y.iter().map(|&s| s.to_string()).collect::<Vec<String>>();

                Some(lines)
            }
        }
    }

    fn get_title(&self, lines: &Vec<String>) -> Option<String> {
        let mut index = 0;
        let mut title: String = String::new();

        for lin in lines.iter() {
            if lin.trim().eq("[TITLE]") {
                index += 1;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    } else {
                        title = lin.clone();
                        break;
                    }
                }
            }
            index += 1;
        }
        if title.trim().eq("") {
            None
        } else {
            Some(title)
        }
    }

    fn get_junctions(&self, lines: &Vec<String>) -> Option<Vec<Junction>> {
        let mut index = 0;
        let mut junctions: Vec<Junction> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[JUNCTIONS]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();
                    // get junction id

                    if let Ok(id) = row[0].parse::<usize>() {
                        // get junction elevation
                        let elev: f64 = match row[1].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        // get junction elevation
                        let demand: f64 = match row[2].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        let jn: Junction = JunctionBuilder::new()
                            .set_id(id)
                            .set_elevation(elev)
                            .set_demand(demand)
                            .build();

                        junctions.push(jn);
                    };
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }
        //for jn in junctions.iter() {
        //   println!("* {:?}", jn.to_string());
        //}
        Some(junctions)
    }

    fn get_reservoirs(&self, lines: &Vec<String>) -> Option<Vec<Reservoir>> {
        let mut index = 0;
        let mut reservoirs: Vec<Reservoir> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[RESERVOIRS]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    // get junction id
                    if let Ok(id) = row[0].parse::<usize>() {
                        // get junction elevation
                        let elev: f64 = match row[1].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        // rbuilder.set_head(elev);
                        let resrvr: Reservoir =
                            ReservoirBuilder::new().set_id(id).set_head(elev).build();
                        reservoirs.push(resrvr);
                    }
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }
        //for jn in junctions.iter() {
        //   println!("* {:?}", jn.to_string());
        //}
        Some(reservoirs)
    }

    fn get_tanks(&self, lines: &Vec<String>) -> Option<Vec<Tank>> {
        let mut index = 0;
        let mut tanks: Vec<Tank> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[TANKS]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    // get junction id
                    if let Ok(id) = row[0].parse::<usize>() {
                        // get junction elevation
                        let elev: f64 = match row[1].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        let mut initial_level: f64 = 0.0;

                        if row.len() > 2 {
                            initial_level = match row[2].parse::<f64>() {
                                Err(_eror) => 0.0f64,
                                Ok(value) => value,
                            };
                        };

                        let tank: Tank = TankBuilder::new()
                            .set_id(id)
                            .set_elevation(elev)
                            .set_initial_level(initial_level)
                            .build();

                        tanks.push(tank);
                    };
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }
        //for jn in junctions.iter() {
        //   println!("* {:?}", jn.to_string());
        //}
        Some(tanks)
    }

    fn get_pipes(&self, lines: &Vec<String>) -> Option<Vec<Pipe>> {
        let mut index = 0;
        let mut pipes: Vec<Pipe> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[PIPES]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    // get junction id
                    if let Ok(id) = row[0].parse::<usize>() {
                        // get junction elevation
                        let start_node: usize = match row[1].parse::<usize>() {
                            Err(_eror) => 0,
                            Ok(value) => value,
                        };

                        let end_node: usize = match row[2].parse::<usize>() {
                            Err(_eror) => 0,
                            Ok(value) => value,
                        };

                        let length: f64 = match row[3].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        let diameter: f64 = match row[4].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        let roughness: f64 = match row[5].parse::<f64>() {
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        let mut min_loss: f64 = 0.0;
                        let mut status: LinkStatus = LinkStatus::Open;

                        if row.len() > 6 {
                            min_loss = match row[6].parse::<f64>() {
                                Err(_eror) => 0.0f64,
                                Ok(value) => value,
                            };
                            if row.len() > 7 {
                                status = match row[7].parse::<String>() {
                                    Err(_eror) => LinkStatus::Open,
                                    Ok(value) => {
                                        if value == "Open" {
                                            LinkStatus::Open
                                        } else {
                                            LinkStatus::Closed
                                        }
                                    }
                                };
                            };
                        };

                        if start_node != end_node {
                            let pip = PipeBuilder::new()
                                .set_id(id)
                                // .set_name(name)
                                // .set_vertices(vertices)
                                .set_start(start_node)
                                .set_end(end_node)
                                .set_length(length)
                                .set_diameter(diameter)
                                .set_roughness(roughness)
                                .set_minorloss(min_loss)
                                .set_status(status)
                                //.set_check_valve(check_valve)
                                .build();

                            pipes.push(pip);
                        }
                    }
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }
        Some(pipes)
    }

    fn get_pumps(&self, lines: &Vec<String>) -> Option<Vec<Pump>> {
        let mut index = 0;
        let mut pumps: Vec<Pump> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[PUMPS]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    if let Ok(id) = row[0].parse::<usize>() {
                        let start_node: usize = match row[1].parse::<usize>() {
                            Err(_eror) => 0,
                            Ok(value) => value,
                        };

                        let end_node: usize = match row[2].parse::<usize>() {
                            Err(_eror) => 0,
                            Ok(value) => value,
                        };

                        let parameters: Option<String> = match row[3].parse::<String>() {
                            Err(_eror) => None,
                            Ok(value) => Some(value),
                        };

                        if start_node != end_node {
                            let pmp = PumpBuilder::new()
                                .set_id(id)
                                .set_start(start_node)
                                .set_end(end_node)
                                .set_parameters(parameters)
                                .build();

                            pumps.push(pmp);
                        };
                    };
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }
        //for jn in junctions.iter() {
        //   println!("* {:?}", jn.to_string());
        //}
        Some(pumps)
    }

    fn get_options(&self, lines: &Vec<String>) -> Option<Options> {
        let mut index = 0;
        let mut flow_unit: FlowUnits = FlowUnits::Cms;
        let mut headlossformula = HeadlossFormula::Hw;

        for lin in lines.iter() {
            if lin.trim().eq("[OPTIONS]") {
                index += 1;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    //println!("***//////////-------- {:?}", row[0]);

                    if row[0].trim().eq("Units") {
                        if row[1].trim().eq("LPS") {
                            flow_unit = FlowUnits::Lps;
                        }

                        if row[1].trim().eq("CMH") {
                            flow_unit = FlowUnits::Cmh;
                        }
                    };

                    if row[0].eq("Headloss") {
                        if row[1].eq("H-W") {
                            headlossformula = HeadlossFormula::Hw;
                        } else if row[1].eq("D-W") {
                            headlossformula = HeadlossFormula::Dw;
                        } else if row[1].eq("C-M") {
                            headlossformula = HeadlossFormula::Cm;
                        } else {
                            headlossformula = HeadlossFormula::Hw;
                        }
                    };

                    //let mut rbuilder : OptionsBuilder = OptionsBuilder::new();

                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }
        //for jn in junctions.iter() {
        //   println!("* {:?}", jn.to_string());
        //}
        let optns: Options = OptionsBuilder::new()
            .set_flow_unit(flow_unit)
            .set_headlossformula(headlossformula)
            .build();
        Some(optns)
    }

    fn get_coordinates(&self, lines: &Vec<String>) -> Option<Vec<(usize, Position)>> {
        let mut index = 0;
        let mut positions: Vec<(usize, Position)> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[COORDINATES]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    // get junction id

                    if let Ok(id) = row[0].parse::<usize>() {
                        let x: f32 = match row[1].parse::<f32>() {
                            Err(_eror) => 0.0f32,
                            Ok(value) => value,
                        };

                        let y: f32 = match row[2].parse::<f32>() {
                            Err(_eror) => 0.0f32,
                            Ok(value) => value,
                        };
                        positions.push((id, Position::new(x, y)));
                    };
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }

        Some(positions)
    }

    fn get_vertices(&self, lines: &Vec<String>) -> Option<Vec<(usize, Position)>> {
        let mut index = 0;
        let mut positions: Vec<(usize, Position)> = Vec::new();

        for lin in lines.iter() {
            if lin.trim().eq("[VERTICES]") {
                index += 2;
                let mut _continueloop: bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {
                        _continueloop = false;
                        break;
                    }

                    let row: Vec<&str> = lines[index].split_whitespace().collect();

                    // get junction id

                    if let Ok(id) = row[0].parse::<usize>() {
                        let x: f32 = match row[1].parse::<f32>() {
                            Err(_eror) => 0.0f32,
                            Ok(value) => value,
                        };

                        let y: f32 = match row[2].parse::<f32>() {
                            Err(_eror) => 0.0f32,
                            Ok(value) => value,
                        };
                        positions.push((id, Position::new(x, y)));
                    };
                    //---------------------------------------------
                    index += 1;
                }
            }
            index += 1;
        }

        Some(positions)
    }
}
