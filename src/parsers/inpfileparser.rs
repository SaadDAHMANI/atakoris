
use std::fs::File;
use std::io::prelude::*;

use crate::network::Network;
use crate::network::NetworkBuilder;
use crate::network::link::LinkStatus;
use crate::network::link::pipe::*;
use crate::network::node::junction::*;
use crate::network::node::reservoir::*;
use crate::network::node::tank::*;
use crate::network::link::pump::*;
use crate::network::{Options, OptionsBuilder, FlowUnits, HeadlossFormula};



///
/// This is a parser for "*.inp" files (Epanet file format)
/// Look at : http://wateranalytics.org/EPANET/_inp_file.html
/// 
pub struct InpFileParser<'a> {
    pub file_path : &'a str,
    pub content : Option<String>,
    lines : Option<Vec<String>>,  
}

impl<'a> InpFileParser<'a>{
    pub fn new(file_path : &'a str)->Self{
        InpFileParser { 
            file_path, 
            content : None,
            lines : None,
        }
    }   
    
    pub fn read(&mut self)-> Result<Network, std::io::Error> {
       
        let content = self.get_content()?;

        self.content=Some(content);

        self.get_lines();

        let junctions = self.get_junctions();
        let tanks = self.get_tanks();
        let reservoirs = self.get_reservoirs();
        let pipes = self.get_pipes(); 
        let pumps = self.get_pumps();
        let optns = self.get_options();

        let mut wdnb = NetworkBuilder::new();

        match junctions {
            None => (),
            Some(nodes)=> {
                wdnb.set_junctions(nodes);
                ()
            },
        };

        match tanks {
            None => (),
            Some(tanks) => {
                wdnb.set_tanks(tanks);
            },
        };

        match reservoirs {
            None => (),
            Some(reservoirs) => {
                wdnb.set_reservoir(reservoirs);
            },
        };

        match pipes {
            None => (),
            Some(pps) => {
                wdnb.set_pipes(pps);
            },
        };

        match pumps {
            None => (),
            Some(pmps) => {
                wdnb.set_pumps(pmps);
            },
        };

        match optns {
            None => (),
            Some(optins) => {
                wdnb.set_options(optins);
            },
        };

        Ok(wdnb.build())
    }

    fn get_content(&self)->Result<String, std::io::Error>{
        let mut file = File::open(self.file_path)?;//.expect("Cannot open this file!");
        let mut the_content = String::new();
        file.read_to_string(&mut the_content)?; //.expect("Cannot read the file !!!"); 

        Ok(the_content)        
        //self.content=Some(the_content);
    }
    
    fn get_lines(&mut self) {
        //let blocks :Vec<String> = Vec::new();
        let content = self.content.clone();
        let x = match content { 
                      None => String::from(""),
                      Some(text) => text,                
            };
        
         let y:Vec<&str> = x.split('\n').collect();
         
         let lines = y.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
         
         self.lines =Some(lines);         
    }
    
    fn get_junctions(&self) -> Option<Vec<Junction>>{
        let lines = self.lines.clone();
        match lines{
            None => None,
            Some(lines)=> {
                let mut index = 0;
                let mut junctions : Vec<Junction> = Vec::new();
                
                for lin in lines.iter() {
                    if lin.trim().eq("[JUNCTIONS]") {
                        index +=2;  
                        let mut _continueloop : bool = true;
                        while _continueloop {
                            if lines[index].trim().eq("") {_continueloop = false; break;}
                            
                            let row : Vec<&str> = lines[index].split_whitespace().collect();
                            let mut jbuilder : JunctionBuilder = JunctionBuilder::new();
                            
                            // get junction id
                            let val_id = row[0].parse::<usize>();
                            let id : usize =match val_id{
                                Err(_eror) => 0,
                                Ok(value) => value,
                            };
                            jbuilder.set_id(id);
                            
                            // get junction elevation
                            let val_elev = row[1].parse::<f64>();
                            let elev : f64 =match val_elev{
                                Err(_eror) => 0.0f64,
                                Ok(value) => value,
                            };
                            jbuilder.set_elevation(elev);
                            
                            // get junction elevation
                            let val_demand = row[2].parse::<f64>();
                            let demand : f64 =match val_demand {
                                Err(_eror) => 0.0f64,
                                Ok(value) => value,
                            };
                            jbuilder.set_demand(demand);
                            junctions.push(jbuilder.build());
                            
                            //---------------------------------------------                    
                            index+=1;
                        }                          
                    }             
                    index +=1; 
                }        
                //for jn in junctions.iter() {
                //   println!("* {:?}", jn.to_string());
                //}
                 Some(junctions)
            }
        }   
    }        
     
    fn get_reservoirs(&self) -> Option<Vec<Reservoir>>{
        let lines = self.lines.clone();

        match lines{
            None => None,
            Some(lines)=> {
       
            let mut index = 0;
            let mut reservoirs : Vec<Reservoir> = Vec::new();
        
            for lin in lines.iter() {
                if lin.trim().eq("[RESERVOIRS]") {
                    index +=2;  
                    let mut _continueloop : bool = true;
                    while _continueloop {
                        if lines[index].trim().eq("") {_continueloop = false; break;}
                    
                        let row : Vec<&str> = lines[index].split_whitespace().collect();
                        let mut rbuilder : ReservoirBuilder = ReservoirBuilder::new();
                    
                    // get junction id
                            let val_id = row[0].parse::<usize>();
                            let id : usize =match val_id{
                                Err(_eror) => 0,
                                Ok(value) => value,
                            };
                            rbuilder.set_id(id);
                            
                            // get junction elevation
                            let val_elev = row[1].parse::<f64>();
                            let elev : f64 =match val_elev{
                                Err(_eror) => 0.0f64,
                                Ok(value) => value,
                            };
                            
                            rbuilder.set_head(elev);
                            
                            reservoirs.push(rbuilder.build());
                            
                            //---------------------------------------------                    
                            index+=1;
                        }                          
                    }             
                    index +=1; 
                }        
                //for jn in junctions.iter() {
                //   println!("* {:?}", jn.to_string());
                //}
            Some(reservoirs)
        }
    }
}        
 
    fn get_tanks(&self) -> Option<Vec<Tank>>{
        let lines = self.lines.clone();

        match lines{
            None => None,
            Some(lines)=> {
   
                let mut index = 0;
                let mut tanks : Vec<Tank> = Vec::new();
    
                for lin in lines.iter() {
                if lin.trim().eq("[TANKS]") {
                    index +=2;  
                    let mut _continueloop : bool = true;
                    while _continueloop {
                        if lines[index].trim().eq("") {_continueloop = false; break;}
                
                        let row : Vec<&str> = lines[index].split_whitespace().collect();
                        let mut tbuilder : TankBuilder = TankBuilder::new();
                    
                        // get junction id
                        let val_id = row[0].parse::<usize>();
                        let id : usize =match val_id{
                            Err(_eror) => 0,
                            Ok(value) => value,
                        };
                        tbuilder.set_id(id);
                        
                        // get junction elevation
                        let val_elev = row[1].parse::<f64>();
                        let elev : f64 =match val_elev{
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        tbuilder.set_elevation(elev);

                        // get junction elevation
                        let val_init_level = row[2].parse::<f64>();
                        let init_level : f64 =match val_init_level{
                            Err(_eror) => 0.0f64,
                            Ok(value) => value,
                        };

                        tbuilder.set_initial_level(init_level);                        
                                               
                        tanks.push(tbuilder.build());
                        
                        //---------------------------------------------                    
                        index+=1;
                    }                          
                }             
                index +=1; 
            }        
            //for jn in junctions.iter() {
            //   println!("* {:?}", jn.to_string());
            //}
            Some(tanks)
        }
    }
}        

    fn get_pipes(&self) -> Option<Vec<Pipe>>{
    let lines = self.lines.clone();

    match lines{
        None => None,
        Some(lines)=> {

            let mut index = 0;
            let mut pipes : Vec<Pipe> = Vec::new();

            for lin in lines.iter() {
            if lin.trim().eq("[PIPES]") {
                index +=2;  
                let mut _continueloop : bool = true;
                while _continueloop {
                    if lines[index].trim().eq("") {_continueloop = false; break;}
            
                    let row : Vec<&str> = lines[index].split_whitespace().collect();
                    let mut builder : PipeBuilder = PipeBuilder::new();
                
                    // get junction id
                    let val_id = row[0].parse::<usize>();
                    let id : usize =match val_id{
                        Err(_eror) => 0,
                        Ok(value) => value,
                    };
                    builder.set_id(id);
                    
                    // get junction elevation
                    let val_start= row[1].parse::<usize>();
                    let start : usize =match val_start{
                        Err(_eror) => 0,
                        Ok(value) => value,
                    };

                    builder.set_start(start);

                              
                    let val_end= row[2].parse::<usize>();
                    let end : usize =match val_end{
                        Err(_eror) => 0,
                        Ok(value) => value,
                    };
                    builder.set_end(end);
        
                    let val_length= row[3].parse::<f64>();
                    let length : f64 =match val_length{
                        Err(_eror) => 0.0f64,
                        Ok(value) => value,
                    };
                    builder.set_length(length);
                    
                    let val_diameter= row[4].parse::<f64>();
                    let diameter : f64 =match val_diameter{
                        Err(_eror) => 0.0f64,
                        Ok(value) => value,
                    };
                    builder.set_diameter(diameter);

                    let val_roughness= row[5].parse::<f64>();
                    let roughness : f64 =match val_roughness{
                        Err(_eror) => 0.0f64,
                        Ok(value) => value,
                    };
                    builder.set_roughness(roughness);

                    let val_minloss= row[6].parse::<f64>();
                    let minloss : f64 =match val_minloss{
                        Err(_eror) => 0.0f64,
                        Ok(value) => value,
                    };
                    builder.set_minorloss(minloss);

                    let val_status = row[7].parse::<String>();
                    let status : LinkStatus = match val_status{
                        Err(_eror) => LinkStatus::Open,
                        Ok(value) =>  { 
                            if value=="Open" { LinkStatus::Open}
                            else { LinkStatus::Closed} 
                        },
                    };
                    builder.set_status(status);

                    pipes.push(builder.build());
                    
                        //---------------------------------------------                    
                        index+=1;
                    }                          
                }             
                index +=1; 
            }        
            Some(pipes)
        }
        }
    }    
    
    fn get_pumps(&self) -> Option<Vec<Pump>>{
        let lines = self.lines.clone();

        match lines{
            None => None,
            Some(lines)=> {
       
            let mut index = 0;
            let mut pumps : Vec<Pump> = Vec::new();
        
            for lin in lines.iter() {
                if lin.trim().eq("[PUMPS]") {
                    index +=2;  
                    let mut _continueloop : bool = true;
                    while _continueloop {
                        if lines[index].trim().eq("") {_continueloop = false; break;}
                    
                        let row : Vec<&str> = lines[index].split_whitespace().collect();
                        let mut builder : PumpBuilder = PumpBuilder::new();
                    
                            let val_id = row[0].parse::<usize>();
                            let id : usize =match val_id{
                                Err(_eror) => 0,
                                Ok(value) => value,
                            };
                            builder.set_id(id); 

                            let val_startnode = row[1].parse::<usize>();
                            let startnode : usize =match val_startnode{
                                Err(_eror) => 0,
                                Ok(value) => value,
                            };
                            builder.set_start(startnode); 
                            
                            let val_endnode = row[2].parse::<usize>();
                            let endnode : usize =match val_endnode{
                                Err(_eror) => 0,
                                Ok(value) => value,
                            };
                            builder.set_end(endnode); 
                                                   
                            let val_parameters = row[3].parse::<String>();
                            let parameters : Option<String> = match val_parameters {
                                Err(_eror) => None,
                                Ok(value) => Some(value),
                            };
                            builder.set_parameters(parameters); 
                            
                            
                            pumps.push(builder.build());
                            
                            //---------------------------------------------                    
                            index+=1;
                        }                          
                    }             
                    index +=1; 
                }        
                //for jn in junctions.iter() {
                //   println!("* {:?}", jn.to_string());
                //}
            Some(pumps)
        }
    }
}        
    
    fn get_options(&self) -> Option<Options> {
        let lines = self.lines.clone();

        match lines{
            None => None,
            Some(lines)=> {
    
            let mut index = 0;
            let mut optionsbldr : OptionsBuilder = OptionsBuilder::new();
        
            for lin in lines.iter() {
                if lin.trim().eq("[OPTIONS]") {
                    index +=1;

                    let mut _continueloop : bool = true;
                    while _continueloop {
                        if lines[index].trim().eq("") {_continueloop = false; break;}
                    
                        let row : Vec<&str> = lines[index].split_whitespace().collect();

                        //println!("***//////////-------- {:?}", row[0]);

                        if row[0].trim().eq("Units") {
                                if row[1].trim().eq("LPS") {
                                    optionsbldr.set_flow_unit(FlowUnits::Lps);
                                }

                                if row[1].trim().eq("CMH") {
                                    optionsbldr.set_flow_unit(FlowUnits::Cmh);
                                }

                        };

                        if row[0].eq("Headloss") {
                            if row[1].eq("H-W") {
                                optionsbldr.set_headlossformula(HeadlossFormula::Hw);
                            }
                            else if row[1].eq("D-W") {
                                optionsbldr.set_headlossformula(HeadlossFormula::Dw);

                            }
                            else if row[1].eq("C-M") {
                                optionsbldr.set_headlossformula(HeadlossFormula::Cm);

                            }
                            else {
                                optionsbldr.set_headlossformula(HeadlossFormula::Hw);
                            }
                        };                       

                        
                        //let mut rbuilder : OptionsBuilder = OptionsBuilder::new();
                            

                            //---------------------------------------------                    
                            index+=1;
                        }                          
                    }             
                    index +=1; 
                }        
                //for jn in junctions.iter() {
                //   println!("* {:?}", jn.to_string());
                //}
            Some(optionsbldr.build())
        }
    }
    }        

    
                          
                                        
}
//
//fn parse_test_args(argv: Vec<&str>) -> Vec<String> {
//    argv.iter().map(|&s| s.to_string()).collect::<Vec<String>>()
//}
//
