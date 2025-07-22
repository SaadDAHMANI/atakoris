//include!("benchmark.rs");

use atakoris::ffi_dto::*;

use atakoris::network::Network;
/* use std::fs::File;
use std::io::prelude::*;

use std::str::FromStr;
 */
use atakoris::parsers::inpfileparser::InpFileParser;
//use atakor::network::*;
//use atakor::network::node::junction::*;
use atakoris::network::link::Link;
use atakoris::network::node::Node;

use atakoris::solver::Solver;

pub mod benchmark;
use benchmark::benchmark::*;

//--------------------------------------------
extern crate once_cell;
use once_cell::sync::Lazy;

//use crate::sequential_optimization::optimization;
// make a static network
static WDN: Lazy<Network> = Lazy::new(|| {
    match Network::read_from_file(
        "/home/sd/Documents/Rust_apps/atakoris/src/bin/data/Hanoi_optimal.inp",
    ) {
        Ok(wdn) => wdn,
        Err(eror) => panic!("Cannot read the file because of : {}", eror),
    }
});
//--------------------------------------------

const RUN: usize = 342;

fn main() {
    println!("Atakor : a Water Distribution Networks Analyser in Rust programming language.");
    println!("_________________________________________________________________________________");

    println!("--------- RUN : {} -----------", RUN);
    show_static_wdn();

    // test_network3();
    // test_network1_todini();
    // test_network2_todini();
    // test_network4();
    // test_modena_net();
    // test_2loop_network();
}

#[allow(dead_code)]
fn show_static_wdn() {
    println!("The static network : \n");

    println!("The WDN from file : {:?}", WDN.title);

    match &WDN.pipes {
        None => println!("no pipes !!!"),
        Some(pipes) => {
            println!("Pipes = {:?}", pipes.len());

            for p in pipes.iter() {
                println!(
                    "{}, C: {}, Vertices: {:?}",
                    p.to_string(),
                    p.roughness,
                    p.vertices
                );
            }
        }
    };

    match &WDN.junctions {
        None => println!("no junction !!!"),
        Some(nodes) => {
            println!("Junctions = {:?}", nodes.len());
            nodes
                .iter()
                .for_each(|jn| println!("id: {}, pos: {:?}", jn.id, jn.position));
        }
    };

    println!("Flow unit = {:?}", WDN.options.as_ref().unwrap().flow_unit);
}

#[allow(dead_code)]
fn read_wdn_file() {
    //let inp_file = "/home/sd/Documents/Rust_apps/atakor/src/bin/data/network1.inp".to_owned();
    //let inp_file = "/home/sd/Documents/Rust_apps/atakor/src/bin/data/Net1.inp".to_owned();
    //let inp_file = "/home/sd/Documents/Rust_apps/atakor/src/bin/data/Net3.inp".to_owned();
    let inp_file = "/home/sd/Documents/Rust_apps/atakor/src/bin/data/Modena.inp".to_owned();

    //let parser : InpFileParser = InpFileParser::new(&inp_file);

    let wdn = InpFileParser::new(&inp_file).read();

    //println!("The file path is : {}", parser.file_path);

    //println!("Content = \n {:?}", parser.content);

    match wdn {
        Err(eror) => panic!("An error is occured : {}.", eror),

        Ok(netw) => {
            print!("Junctions :\n");
            match netw.junctions {
                None => println!("No junctions !!"),
                Some(juncts) => {
                    for j in juncts.iter() {
                        println!("{}", j.to_string());
                    }
                }
            }

            print!("Reservoirs :\n");
            match netw.reservoirs {
                None => println!("No reservoirs !!"),
                Some(nodes) => {
                    for j in nodes.iter() {
                        println!("{}", j.to_string());
                    }
                }
            }

            print!("Tanks :\n");
            match netw.tanks {
                None => println!("No tanks !!"),
                Some(nodes) => {
                    for j in nodes.iter() {
                        println!("{}", j.to_string());
                    }
                }
            }

            print!("Pipes :\n");
            match netw.pipes {
                None => println!("No pipes !!"),
                Some(pipes) => {
                    for p in pipes.iter() {
                        println!("{}", p.to_string());
                    }
                }
            }

            print!("Pumps :\n");
            match netw.pumps {
                None => println!("No pumps !!"),
                Some(pumps) => {
                    for p in pumps.iter() {
                        println!("{}, {:?}", p.to_string(), p.parameters);
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn test_network3() {
    let mut net3: Network = network3();
    let mut solver: Solver = Solver::new(&mut net3, None);

    let _result = solver.compute();

    let (erq, erh) = solver.get_final_errors().unwrap();
    println!(
        "Q_error : {:?}; \n H_error : {:?}; \n Iterations : {:?}",
        erq,
        erh,
        solver.get_final_iterations()
    );

    /*  match result {
        None => println!("No simulation results !!!"),
        Some(wdn) => {

            match wdn.junctions {
                None => println!("no junctions !!!"),
                Some(nodes) => {
                    for jn in nodes.iter() {
                        println!("{}", jn.to_string())
                    }
                },
            };
        },
    };     */

    match &net3.junctions {
        None => println!("no junctions !!!"),
        Some(nodes) => {
            for nd in nodes.iter() {
                println!("{:?}", nd.to_string());
            }
        }
    }
}

#[allow(dead_code)]
fn test_network1_todini() {
    let mut net: Network = network1_todini();
    let mut solver: Solver = Solver::new(&mut net, None);

    let _result = solver.compute();
    let (erq, erh) = solver.get_final_errors().unwrap();
    println!(
        "Q_error : {:?}; \n H_error : {:?}; \n Iterations : {:?}",
        erq,
        erh,
        solver.get_final_iterations()
    );

    /* match result {
        None => println!("No simulation results !!!"),
        Some(wdn) => {

            match wdn.junctions {
                None => println!("no junctions !!!"),
                Some(nodes) => {
                    for jn in nodes.iter() {
                        println!("{}", jn.to_string())
                    }
                },
            };

            println!("___________[PIPES]____________");

            match wdn.pipes {
                None => println!("no pipes !!!"),
                Some(pipes) => {
                    for elm in pipes.iter() {
                        println!("{}", elm.to_string())
                    }
                },
            };

        },
    }; */

    match &net.junctions {
        None => println!("no junctions !!!"),
        Some(nodes) => {
            for jn in nodes.iter() {
                println!("{}", jn.to_string())
            }
        }
    };

    match &net.pipes {
        None => println!("no pipes !!!"),
        Some(pipes) => {
            for elm in pipes.iter() {
                println!("{}", elm.to_string())
            }
        }
    };
}

#[allow(dead_code)]
fn test_network2_todini() {
    let mut net: Network = network2_todini();
    let mut solver: Solver = Solver::new(&mut net, None);

    // change the m value.
    solver.set_m_parameter(100.0f64);

    use std::time::Instant;
    let now = Instant::now();

    let _result = solver.compute();

    let elapsed = now.elapsed();
    println!("\n Time duration (Elapsed) T : {:.2?} \n", elapsed);

    let (erq, erh) = solver.get_final_errors().unwrap();
    println!(
        "Q_error : {:?}; \n H_error : {:?}; \n Iterations : {:?}",
        erq,
        erh,
        solver.get_final_iterations()
    );

    /* match result {
        None => println!("No simulation results !!!"),
        Some(wdn) => {

            println!("___________[JUNCTIONS]____________");

            match wdn.junctions {
                None => println!("no junctions !!!"),
                Some(nodes) => {
                    for jn in nodes.iter() {
                        println!("{}", jn.to_string())
                    }
                },
            };


            println!("___________[PIPES]____________");

            match wdn.pipes {
                None => println!("no pipes !!!"),
                Some(pipes) => {
                    for elm in pipes.iter() {
                        println!("{}", elm.to_string())
                    }
                },
            };

        },
    }; */

    match &net.junctions {
        None => println!("no junctions !!!"),
        Some(nodes) => {
            for jn in nodes.iter() {
                println!("{}", jn.to_string())
            }
        }
    };

    match &net.pipes {
        None => println!("no pipes !!!"),
        Some(pipes) => {
            for elm in pipes.iter() {
                println!("{}", elm.to_string())
            }
        }
    };
}

#[allow(dead_code)]
fn test_network4() {
    let mut net: Network = network4();
    let mut solver: Solver = Solver::new(&mut net, None);

    // change the m value.
    solver.set_m_parameter(100.0f64);

    let _result = solver.compute();
    let (erq, erh) = solver.get_final_errors().unwrap();
    println!(
        "Q_error: \n {:?} \n; H_error : \n {:?}; Iterations : {:?}",
        erq,
        erh,
        solver.get_final_iterations()
    );

    /* match result {
        None => println!("No simulation results !!!"),
        Some(wdn) => {

            for qi in wdn.pipes.unwrap().iter() {
                println!("q = {} m3/h", qi.flow.unwrap()*3600.00f64);
            }

            println!("___________[JUNCTIONS]____________");

            match wdn.junctions {
                None => println!("no junctions !!!"),
                Some(nodes) => {
                    for jn in nodes.iter() {
                        println!("{}", jn.to_string())
                    }
                },
            };


            println!("___________[PIPES]____________");

            // match wdn.pipes {
            //     None => println!("no pipes !!!"),
            //     Some(pips) => {
            //         for elm in pips.iter() {
            //             println!("{}", elm.to_string())
            //         }
            //     },
            // };

            println!("___________[PUMPS]____________");

            match wdn.pumps {
                None => println!("no pumps !!!"),
                Some(pumps) => {
                    for elm in pumps.iter() {
                        println!("{}", elm.to_string())
                    }
                },
            };

        },
    }; */

    println!("___________[JUNCTIONS]____________");
    match &net.junctions {
        None => println!("no junctions !!!"),
        Some(nodes) => {
            for jn in nodes.iter() {
                println!("{}", jn.to_string())
            }
        }
    };

    println!("___________[PUMPS]____________");
    match &net.pipes {
        None => println!("no pipes !!!"),
        Some(pipes) => {
            for elm in pipes.iter() {
                println!("{}", elm.to_string())
            }
        }
    };

    println!("___________[PUMPS]____________");

    match &net.pumps {
        None => println!("no pumps !!!"),
        Some(pumps) => {
            for elm in pumps.iter() {
                println!("{}", elm.to_string())
            }
        }
    };
}

#[allow(dead_code)]
fn test_modena_net() {
    let inp_file = "/home/sd/Documents/Rust_apps/atakor/src/bin/data/Modena.inp".to_owned();

    //let parser : InpFileParser = InpFileParser::new(&inp_file);

    let wdn = InpFileParser::new(&inp_file).read();

    match wdn {
        Err(eror) => println!("Cannot read the file because of : {:?}.", eror),
        Ok(mut net) => {
            use std::time::Instant;
            let now = Instant::now();

            let mut solver: Solver = Solver::new(&mut net, None);
            solver.set_m_parameter(100.00);

            let _result = solver.compute();

            let elapsed = now.elapsed();
            println!("\n Time duration (Elapsed) T : {:.2?} \n", elapsed);

            let (erq, erh) = solver.get_final_errors().unwrap();
            println!(
                "Q_error: \n {:?} \n; H_error : \n {:?}; Iterations : {:?}",
                erq,
                erh,
                solver.get_final_iterations()
            );

            /* match result {
                None => println!("No results !!!"),
                Some(wdn)=>{

                    println!("___________[JUNCTIONS]____________");

                    // match wdn.junctions {
                    //     None => println!("no junctions !!!"),
                    //     Some(nodes) => {
                    //         for jn in nodes.iter() {
                    //             println!("{}", jn.to_string())
                    //         }
                    //     },
                    // };


                    println!("___________[PIPES]____________");

                    match wdn.pipes {
                        None => println!("no pipes !!!"),
                        Some(pipes) => {
                            for elm in pipes.iter() {
                                println!("{}", elm.to_string())
                            }
                        },
                    };

                }
            } */

            println!("___________[JUNCTIONS]____________");
            match &net.junctions {
                None => println!("no junctions !!!"),
                Some(nodes) => {
                    for jn in nodes.iter() {
                        println!("{}", jn.to_string())
                    }
                }
            };

            println!("___________[PUMPS]____________");
            match &net.pipes {
                None => println!("no pipes !!!"),
                Some(pipes) => {
                    for elm in pipes.iter() {
                        println!("{}", elm.to_string())
                    }
                }
            };

            println!("___________[PUMPS]____________");

            match &net.pumps {
                None => println!("no pumps !!!"),
                Some(pumps) => {
                    for elm in pumps.iter() {
                        println!("{}", elm.to_string())
                    }
                }
            };
        }
    };
}

#[allow(dead_code)]
fn test_2loop_network() {
    let mut net: Network = network_2loop();

    let mut solver: Solver = Solver::new(&mut net, Some(0.00001));
    solver.set_m_parameter(1000.0);

    let _result = solver.compute();

    let (erq, erh) = solver.get_final_errors().unwrap();
    println!(
        "Q_error : {:?}; \n H_error : {:?}; \n Iterations : {:?}",
        erq,
        erh,
        solver.get_final_iterations()
    );

    let _net2 = solver.compute();

    println!("Flow unit (After) = {:?}", net.options.unwrap().flow_unit);

    if let Some(nodes) = &net.junctions {
        println!("___________[JUNCTIONS]____________");
        for jn in nodes.iter() {
            println!("id: {}, P={:?} m.", jn.id, jn.pressure());
        }
    }

    if let Some(pipes) = &net.pipes {
        println!("___________[PIPES]____________");
        for pip in pipes.iter() {
            println!("id: {}, Q={:?} m3/s.", pip.id, pip.flow);
        }
    }

    if let Some(pumps) = &net.pumps {
        println!("___________[PUMPS]____________");
        for pmp in pumps.iter() {
            println!("{}", pmp.to_string());
        }
    }
}

#[allow(dead_code)]
fn test_dto() {
    let _j1: JunctionDto = JunctionDto {
        id: 1,
        elevation: 100.0,
        demand: 10.0,
    };

    let _j2: JunctionDto = JunctionDto {
        id: 2,
        elevation: 100.0,
        demand: 10.0,
    };

    let _j3: JunctionDto = JunctionDto {
        id: 3,
        elevation: 100.0,
        demand: 10.0,
    };
    /*
    let _netw: NetworkDto = NetworkDto {
        title: Some(String::from("Net1")),
        junctions: Some(vec![_j1, _j2, _j3]),
    };
    */
}
