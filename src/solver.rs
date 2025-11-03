// Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks.
// Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017,
// publisher: American Society of Civil Engineers.
//***********************************************************************************************
// Developped by : Saad Dahmani <sd.dahmani2000@gmail.com; s.dahmani@univ-bouira.dz>
//***********************************************************************************************

// no : number of fixed head nodes (tanks + reservoirs)
// self.junction_count : number of node - no (exclude fixed head)
// np : number of links = pipe + pumps + valves
// a21 : incidence matrix (self.junction_count x np)
// a12 = transpose(a21) : incidence matrix (np x self.junction_count)

use std::time::{Duration, Instant};

use crate::{AFD_FACTOR, CMD_FACTOR, CMH_FACTOR, LPM_FACTOR, LPS_FACTOR, network::FlowUnits};

// use super::network::node::*;
//use super::network::link::{pipe::Pipe, pump::Pump, valve::Valve};
//use super::network::node::{junction::Junction, reservoir::Reservoir, tank::Tank};
use super::network::Network;

pub struct Solver<'a> {
    pub network: &'a mut Network,
    /*
       junctions: &'a mut Vec<Junction>,
       tanks: &'a mut Vec<Tank>,
       reservoirs: &'a mut Vec<Reservoir>,
       pipes: &'a mut Vec<Pipe>,
       pumps: &'a mut Vec<Pump>,
       valves: &'a mut Vec<Valve>,
    */
    junction_count: usize,
    tank_count: usize,
    reservoir_count: usize,
    pipe_count: usize,
    pump_count: usize,
    valve_count: usize,
    flow_unit_multiplayer: f64,
    ///
    /// non-zero & strict positive m-value. Default value : m = 100.
    ///
    m: f64,
    n: f64,
    iterations: Option<usize>,
    final_error: Option<(f64, f64)>,
    time_analysis: Option<Duration>,
    objective_error: f64,
}

impl<'a> Solver<'a> {
    ///
    /// use this function to build new Solver
    ///
    /// objective_error : minimal error flow and head computation (stopping criterion). If None, the default value (objective_error = 0.001) will be used.
    ///
    pub fn new(wdn: &'a mut Network, objective_error: Option<f64>) -> Self {
        let njunction: usize = match &wdn.junctions {
            None => 0,
            Some(items) => items.len(),
        };

        let ntank: usize = match &wdn.tanks {
            None => 0,
            Some(items) => items.len(),
        };

        let nreservoir: usize = match &wdn.reservoirs {
            None => 0,
            Some(items) => items.len(),
        };

        let npipe: usize = match &wdn.pipes {
            None => 0,
            Some(items) => items.len(),
        };

        let npump: usize = match &wdn.pumps {
            None => 0,
            Some(items) => items.len(),
        };
        let nvalve: usize = match &wdn.valves {
            None => 0,
            Some(items) => items.len(),
        };

        let obj_err: f64 = match objective_error {
            None => 0.001,
            Some(objerr) => objerr,
        };

        let flow_unit_multiplayer = Solver::conversion_2is_multiplayer(&wdn);

        let solver = Solver {
            network: wdn,
            junction_count: njunction,
            tank_count: ntank,
            reservoir_count: nreservoir,
            pipe_count: npipe,
            pump_count: npump,
            valve_count: nvalve,
            m: 100.0f64,
            n: 1.852f64,
            iterations: None,
            final_error: None,
            objective_error: obj_err,
            time_analysis: None,
            flow_unit_multiplayer,
        };
        //solver.convert_2is();
        //
        solver
    }

    ///
    /// Set non-zero & strict positive m-value. Default value : m = 100.
    ///
    pub fn set_m_parameter(&mut self, m_value: f64) {
        self.m = f64::max(m_value, 1.0);
    }

    ///
    /// Set non-zero & strict positive. Default value :  objective_error = 0.001.
    ///
    pub fn set_objective_error(&mut self, err_value: f64) {
        self.objective_error = f64::max(err_value, 0.0000000000001);
    }

    pub fn get_final_iterations(&self) -> Option<usize> {
        self.iterations
    }

    pub fn get_final_errors(&self) -> Option<(f64, f64)> {
        self.final_error
    }

    pub fn get_version(&self) -> &'static str {
        "0.1.2"
    }

    pub fn get_time_analysis(&self) -> Option<Duration> {
        self.time_analysis
    }
    pub fn compute(&mut self) -> Option<&Network> {
        let chronos = Instant::now();

        let (a21, a10, h0, q) = self.get_network();

        let nn = a21.len();
        let np = a21[0].len();

        // let npip : usize = self.pipes.len();
        // let npump : usize = self.pumps.len();
        // let nvlv : usize = self.valves.len();

        if nn < 2 {
            panic!("No nodes !!!");
        } // return Option::None;}
        if np < 1 {
            panic!("No pipes !!!");
        } //return Option::None;}

        let mut iter: usize = 0;
        let itermax: usize = 20;
        let objective_err: f64 = self.objective_error;
        let mut final_err_q: f64 = f64::MAX;
        let mut final_err_h: f64 = f64::MAX;

        let mut _a: Vec<Vec<f64>> = self.initilize_a_matrix(); // = vec![vec![0.0f64; np]; np]; //A
        let mut _b = vec![0.0f64; np]; // B
        let mut _c = vec![0.0f64; np];
        let mut _flowsq = vec![0.0f64; np];
        let mut _previous_q = vec![0.0f64; np];
        let mut _headsh = vec![0.0f64; nn];
        let mut _previous_h = vec![0.0f64; nn];

        let mut _coef_a = vec![0.0f64; np]; // ai
        let mut _coef_b = vec![0.0f64; np]; //bi

        //let m : f64 = 100.0;
        //let n : f64 = 1.852; //2.0;

        let _a12 = Self::transpose(&a21);

        #[cfg(feature = "deep_report")]
        {
            Self::print(&a21, &"A21");
            Self::print(&_a12, &"A12");
        }

        // step 0 : compute Qmax
        let qmax: f64 = q.iter().sum();
        /*
        for i in 0..q.len() {
           qmax+=q[i];
        } */

        // compute delta Q
        let deltaq = qmax / self.m;
        for i in 0..np {
            _flowsq[i] = qmax;
        }

        let mut stoploop: bool = false;

        while stoploop == false {
            #[cfg(feature = "report")]
            {
                println!("-----------------------------> iter : {}", iter);
            }

            #[cfg(feature = "deep_report")]
            {
                Solver::print(&_a, &"[A]0");
                Solver::print_vector(&_b, &"[B]0");
            }

            //Updating A (eq13) & B (eq14):
            self.update_matrices_a_b(&mut _a, &mut _b, &_flowsq, deltaq, self.n);

            #[cfg(feature = "deep_report")]
            {
                Solver::print(&_a, &String::from("[A]"));
                Solver::print_vector(&_b, &"[B]");
            }

            // Step 2 : Compute V (eq) and C
            // Compute V:
            let inva = Solver::invers_diagonal(&_a);
            let inva = match inva {
                Ok(matrx) => matrx,
                Err(error) => panic!("Problem with inverse diagonal matrix : {:?}", error),
            };

            #[cfg(feature = "deep_report")]
            {
                Solver::print(&inva, "[A-]");
            }

            let _v1 = Solver::product(&a21, &inva);
            let _v1 = match _v1 {
                Ok(matrx) => matrx,
                Err(error) => panic!("Problem with product matrices : {:?}", error),
            };

            let _v = Solver::product(&_v1, &_a12);
            let _v = match _v {
                Ok(matrx) => matrx,
                Err(error) => panic!("Problem with product matrices : {:?}", error),
            };

            #[cfg(feature = "deep_report")]
            {
                Solver::print(&_v, "[V]");
            }

            //Compute C:
            let _tmpc = Solver::product2(&a10, &h0);
            let tmpc = match _tmpc {
                Ok(vectr) => vectr,
                Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
            };

            for i in 0..np {
                _c[i] = (-1.0 * _b[i]) - tmpc[i];
            }

            //print_vector(&_c, "C : ");

            // Step 3 : Compute H (eq.29)
            let invv = Solver::invers(&_v);

            let invv = match invv {
                Ok(matrix) => matrix,
                Err(error) => panic!("Problem with inverse matrix : {:?}", error),
            };

            let tmp = Solver::product2(&_v1, &_c);

            let mut tmp = match tmp {
                Ok(vectr) => vectr,
                Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
            };

            for i in 0..nn {
                tmp[i] -= q[i];
            }

            let _h = Solver::product2(&invv, &tmp);
            _headsh = match _h {
                Ok(vect) => vect,
                Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
            };

            #[cfg(feature = "deep_report")]
            {
                Solver::print_vector(&_headsh, "[H] :");
            }
            // Step 4 : Compute flowws Q (eq30)
            let tmpql = Solver::product2(&inva, &_c);
            let tmpql = match tmpql {
                Ok(vect) => vect,
                Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
            };

            let tmpqm = Solver::product(&inva, &_a12);
            let tmpqm = match tmpqm {
                Ok(matrx) => matrx,
                Err(error) => panic!("Problem with matrix multiplication : {:?}", error),
            };

            let tmpqr = Solver::product2(&tmpqm, &_headsh);
            let tmpqr = match tmpqr {
                Ok(vect) => vect,
                Err(error) => panic!("Problem with product matrix by vector : {:?}", error),
            };

            for i in 0..np {
                _flowsq[i] = tmpql[i] - tmpqr[i];
            }

            #[cfg(feature = "deep_report")]
            {
                Solver::print_vector(&_flowsq, "[Q]");
            }

            #[cfg(feature = "deep_report")]
            {
                Solver::print(&tmpqm, &String::from("At-1 x A12"));
            }

            //Check convergence :
            let check_q_err = Solver::check_convergence(&_flowsq, &_previous_q, objective_err);
            match check_q_err.0 {
                false => stoploop = false,
                true => {
                    let check_h_err =
                        Solver::check_convergence(&_headsh, &_previous_h, objective_err);
                    final_err_h = check_h_err.1;
                    // match check_h_err.0 {
                    //     false => stoploop = false,
                    //     true => stoploop = true,
                    //  };
                    stoploop = check_h_err.0;
                }
            };

            final_err_q = check_q_err.1;

            //Copy data
            for i in 0..np {
                _previous_q[i] = _flowsq[i];
            }

            for j in 0..nn {
                _previous_h[j] = _headsh[j];
            }

            iter += 1;

            if iter >= itermax {
                stoploop = true;
            }

            #[cfg(feature = "report")]
            {
                Solver::print_vector(&_flowsq, "[Qs]");
                Solver::print_vector(&_headsh, "[Hs]");
            }
        }

        self.copy_results(&_headsh, &_flowsq);
        self.iterations = Some(iter);
        self.final_error = Some((final_err_q, final_err_h));

        self.time_analysis = Some(chronos.elapsed());
        /*  let wdn = NetworkBuilder::new()
        .set_junctions(Some(self.junctions.clone()))
        .set_pipes(Some(self.pipes.clone()))
        .set_pumps(Some(self.pumps.clone()))
        .set_valves(Some(self.valves.clone()))
        .build();  */
        Some(&self.network)
    }

    fn copy_results(&mut self, heads_h: &[f64], flows_q: &[f64]) {
        if let Some(junctions) = &mut self.network.junctions {
            for i in 0..self.junction_count {
                junctions[i].head = Some(heads_h[i]);
            }
        };

        if let Some(pipes) = &mut self.network.pipes {
            for i in 0..self.pipe_count {
                pipes[i].flow = Some(flows_q[i] / self.flow_unit_multiplayer);
            }
        };

        let mut k: usize = self.pipe_count;

        if let Some(pumps) = &mut self.network.pumps {
            for i in 0..self.pump_count {
                pumps[i].flow = Some(flows_q[k] / self.flow_unit_multiplayer);
                k += 1;
            }
        };

        if let Some(valves) = &mut self.network.valves {
            for i in 0..self.valve_count {
                valves[i].flow = Some(flows_q[k] / self.flow_unit_multiplayer);
                k += 1;
            }
        };
    }

    ///
    /// Convert the network to the IS (International System)
    ///
    fn conversion_2is_multiplayer(wdn: &Network) -> f64 {
        match &wdn.junctions {
            None => 1.0,
            Some(_items) => match wdn.options.flow_unit {
                FlowUnits::Lps => LPS_FACTOR,
                FlowUnits::Afd => AFD_FACTOR,
                FlowUnits::Cfs => 1.0,
                FlowUnits::Cmd => CMD_FACTOR,
                FlowUnits::Cmh => CMH_FACTOR,
                FlowUnits::Gpm => 1.0,
                FlowUnits::Imgd => 1.0,
                FlowUnits::Lpm => LPM_FACTOR,
                FlowUnits::Mgd => 1.0,
                FlowUnits::Mld => 1.0,
                FlowUnits::Cms => 1.0,
            },
        }
    }

    fn link_sizes(&self) -> (usize, usize, usize) {
        (self.pipe_count, self.pump_count, self.valve_count)
    }

    ///
    /// Get network matrices
    ///
    fn get_network(&self) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
        let nt = self.tank_count;
        let nr = self.reservoir_count;
        let (npip, npmp, nvlv) = self.link_sizes();
        let no = self.tank_count + self.reservoir_count;
        let np = self.pipe_count + self.pump_count + self.valve_count;
        // =========================================================
        println!(
            "tanks: {}, reservoirs: {}, pipes: {}, pumps: {}, valves: {}",
            self.tank_count,
            self.reservoir_count,
            self.pipe_count,
            self.pump_count,
            self.valve_count
        );
        // =========================================================

        // nodal demand
        let mut q = vec![0.0f64; self.junction_count];
        //H0 : reservoirs + tanks
        let mut _h0 = vec![0.0f64; no];

        //Matrix A21
        let mut _a21 = vec![vec![0.0f64; np]; self.junction_count];

        match &self.network.junctions {
            None => {}
            Some(junctions) => {
                match &self.network.pipes {
                    None => {}
                    Some(pipes) => {
                        for i in 0..self.junction_count {
                            // Junction - Pipes :
                            for j in 0..self.pipe_count {
                                if pipes[j].start == junctions[i].id {
                                    _a21[i][j] = -1.0;
                                } else if pipes[j].end == junctions[i].id {
                                    _a21[i][j] = 1.0;
                                }
                            }
                        }
                    }
                };

                match &self.network.pumps {
                    None => {}
                    Some(pumps) => {
                        for i in 0..self.junction_count {
                            // Pumps :
                            for k in 0..self.pump_count {
                                if pumps[k].start == junctions[i].id {
                                    _a21[i][k + npip] = -1.0;
                                } else if pumps[k].end == junctions[i].id {
                                    _a21[i][k + npip] = 1.0;
                                }
                            }
                        }
                    }
                };

                match &self.network.valves {
                    None => {}
                    Some(valves) => {
                        for i in 0..self.junction_count {
                            // Valves :
                            for k in 0..self.valve_count {
                                if valves[k].start == junctions[i].id {
                                    _a21[i][k + npip + npmp] = -1.0;
                                } else if valves[k].end == junctions[i].id {
                                    _a21[i][k + npip + npmp] = 1.0;
                                }
                            }
                        }
                    }
                };
                //nodal demand
                for i in 0..self.junction_count {
                    q[i] = junctions[i].demand * self.flow_unit_multiplayer;
                }
            }
        };

        //Matrix A10
        let mut _a10 = vec![vec![0.0f64; no]; np];

        // Tanks
        match &self.network.tanks {
            None => {}
            Some(tanks) => {
                //pipes
                match &self.network.pipes {
                    None => {}
                    Some(pipes) => {
                        //tanks - pipes
                        for j in 0..self.tank_count {
                            // Tanks -Pipes
                            for i in 0..self.pipe_count {
                                if pipes[i].start == tanks[j].id {
                                    _a10[i][j] = -1.0;
                                } else if pipes[i].end == tanks[j].id {
                                    _a10[i][j] = 1.0;
                                }
                            }
                        }
                    }
                };

                match &self.network.pumps {
                    None => {}
                    Some(pumps) => {
                        for j in 0..self.tank_count {
                            // Tanks - Pumps
                            for i in 0..self.pump_count {
                                if pumps[i].start == tanks[j].id {
                                    _a10[i + npip][j] = -1.0;
                                } else if pumps[i].end == tanks[j].id {
                                    _a10[i + npip][j] = 1.0;
                                }
                            }
                        }
                    }
                };

                match &self.network.valves {
                    None => {}
                    Some(valves) => {
                        for j in 0..self.tank_count {
                            // Tanks - Valves
                            for i in 0..self.valve_count {
                                if valves[i].start == tanks[j].id {
                                    _a10[i + npip + npmp][j] = -1.0;
                                } else if valves[i].end == tanks[j].id {
                                    _a10[i + npip + npmp][j] = 1.0;
                                }
                            }
                        }
                    }
                };
                //Fixed head
                for k in 0..nt {
                    _h0[k] = tanks[k].head();
                }
            }
        };

        // Reservoirs
        match &self.network.reservoirs {
            None => {}
            Some(reservoirs) => {
                // Pipes
                match &self.network.pipes {
                    None => {}
                    Some(pipes) => {
                        for j in 0..self.reservoir_count {
                            // Reservoirs - Pipes
                            for i in 0..npip {
                                if pipes[i].start == reservoirs[j].id {
                                    _a10[i][j + nt] = -1.0;
                                } else if pipes[i].end == reservoirs[j].id {
                                    _a10[i][j + nt] = 1.0;
                                }
                            }
                        }
                    }
                };

                // Pumps
                match &self.network.pumps {
                    None => {}
                    Some(pumps) => {
                        for j in 0..self.reservoir_count {
                            // Reservoirs - Pumps
                            for i in 0..npmp {
                                if pumps[i].start == reservoirs[j].id {
                                    _a10[i + npip][j + nt] = -1.0;
                                } else if pumps[i].end == reservoirs[j].id {
                                    _a10[i + npip][j + nt] = 1.0;
                                }
                            }
                        }
                    }
                };

                // Valves
                match &self.network.valves {
                    None => {}
                    Some(valves) => {
                        for j in 0..self.reservoir_count {
                            // Reservoirs - Valves
                            for i in 0..nvlv {
                                if valves[i].start == reservoirs[j].id {
                                    _a10[i + npip + npmp][j + nt] = -1.0;
                                } else if valves[i].end == reservoirs[j].id {
                                    _a10[i + npip + npmp][j + nt] = 1.0;
                                }
                            }
                        }
                    }
                };
                //Fixed head
                for k in 0..nr {
                    _h0[k + nt] = reservoirs[k].head;
                }
            }
        };

        println!("A21: {:?}", _a21);
        println!("A10 : {:?}", _a10);
        println!("H0: {:?}", _h0);
        println!("q = {:?}", q);

        (_a21, _a10, _h0, q)
    }

    fn check_convergence(actual: &[f64], previous: &[f64], objective: f64) -> (bool, f64) {
        let sum_err = actual
            .iter()
            .zip(previous.iter())
            .fold(0.0f64, |acc, (a, b)| acc + f64::abs(a - b));

        let sumq = actual.iter().fold(0.0f64, |acc, q| acc + q.abs());

        let computed_err = sum_err / sumq;

        #[cfg(feature = "report")]
        {
            println!("Actual convergence err : {}", computed_err);
        }

        if computed_err <= objective {
            (true, computed_err)
        } else {
            (false, computed_err)
        }
    }

    fn invers(matrix: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, String> {
        // if matrix.len() != matrix[0].len() {
        //     Err(String::from("Matrix is not square!"))
        // }
        // else {
        //    let n = matrix.len();
        //     ////let mut inv = vec![vec![0.0f64; n]; n];
        //    ////Using peroxide crate :

        //     let mut pmatrix = zeros(n,n);
        //    //copy matrix
        //    for i in 0..n {
        //        for j in 0..n {
        //            pmatrix[(i,j)]=matrix[i][j];
        //        }
        //    }
        //    let inversed =pmatrix.inv().to_vec();
        //    Ok(inversed)
        // }

        Solver::inverse_matrix_jordan(&matrix)
    }

    fn inverse_matrix_jordan(matrix: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, String> {
        let n = matrix.len();

        if matrix.len() != matrix[0].len() {
            Err(String::from("Matrix is not square!"))
        } else {
            let mut a = vec![vec![0.0f64; 2 * n]; n];

            //copy th matrix
            for i in 0..n {
                for j in 0..n {
                    a[i][j] = matrix[i][j];
                }
            }

            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        a[i][j + n] = 1.0;
                    }
                }
            }

            //Apply Gauss Jordan Elimination on Augmented Matrix (A):

            for i in 0..n {
                if a[i][i] == 0.0 {
                    panic!("diagonal is nul")
                    //Err(String::from("Diagonal is null !"))
                } else {
                    for j in 0..n {
                        if i != j {
                            let ratio = a[j][i] / a[i][i];

                            for k in 0..2 * n {
                                a[j][k] = a[j][k] - ratio * a[i][k]
                            }
                        }
                    }
                }
            }
            // Row Operation to Convert Principal Diagonal to 1.
            for i in 0..n {
                for j in n..2 * n {
                    a[i][j] = a[i][j] / a[i][i];
                }
            }

            //copy result to b :
            let mut b = vec![vec![0.0f64; n]; n];
            for i in 0..n {
                for j in 0..n {
                    b[i][j] = a[i][j + n];
                }
            }
            return Ok(b);
        }
    }

    fn product(left: &Vec<Vec<f64>>, right: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, String> {
        let m = left.len();
        let pl = left[0].len();

        let n = right[0].len();
        let pr = right.len();

        let mut result = vec![vec![0.0f64; n]; m];
        let mut _sum = 0.0f64;
        if pl == pr {
            for i in 0..m {
                for j in 0..n {
                    _sum = 0.0f64;

                    for k in 0..pl {
                        _sum += left[i][k] * right[k][j];
                    }

                    result[i][j] = _sum;
                }
            }
            Ok(result)
        } else {
            Err(String::from(
                "Colomns's count of left matrix not equals rows's count of right matrix!",
            ))
        }
    }

    fn product2(left: &Vec<Vec<f64>>, right: &Vec<f64>) -> Result<Vec<f64>, String> {
        let m = left.len();
        let pl = left[0].len();

        let pr = right.len();

        let mut result = vec![0.0f64; m];
        let mut _sum = 0.0f64;
        if pl == pr {
            for i in 0..m {
                _sum = 0.0f64;

                for j in 0..pl {
                    _sum += left[i][j] * right[j];
                }

                result[i] = _sum;
            }
            Ok(result)
        } else {
            Err(String::from(
                "Colomns's count of left matrix not equals rows's count of right vector!",
            ))
        }
    }

    fn invers_diagonal(matrix: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, String> {
        if matrix.len() == 0 {
            Err(String::from("The matrix size must be >0!"))
        } else {
            if matrix.len() == matrix[0].len() {
                let mut invers = vec![vec![0.0f64; matrix.len()]; matrix.len()];

                for i in 0..matrix.len() {
                    invers[i][i] = 1.0 / matrix[i][i];
                }

                Ok(invers)
            } else {
                Err(String::from("The matrix is not square!"))
            }
        }
    }

    fn initilize_a_matrix(&self) -> Vec<Vec<f64>> {
        // let self.junction_count : usize = match self.junctions {
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
        let (npip, npmp, nvlv) = self.link_sizes();

        let np = npip + npmp + nvlv;

        let mut result_a = vec![vec![0.0f64; np]; np];

        //let np = npip+npmp;
        let rspipes = self.network.get_pipes_resistances().unwrap();

        let qmax = match &self.network.junctions {
            None => 0.0f64,
            Some(junctions) => junctions.iter().fold(0.0f64, |acc, j| acc + j.demand),
        };

        // Pipes resistances
        for i in 0..npip {
            result_a[i][i] = rspipes[i] * qmax;
        }

        // Pumps resistances
        match &self.network.pumps {
            None => {}
            Some(pumps) => {
                for i in 0..npmp {
                    // result_a[i+npip][i+npip]= network.pumps[i].alpha*qmax + network.pumps[i].beta + network.pumps[i].gamma/qmax;
                    result_a[i + npip][i + npip] = 1.0; //  pumps[i].get_r_of_q(qmax, self.flow_unit_multiplayer);
                }
            }
        };

        // Valves resistances
        match &self.network.valves {
            None => {}
            Some(valves) => {
                for i in 0..nvlv {
                    result_a[i + npip + npmp][i + npip + npmp] = valves[i].get_rq(qmax);
                }
            }
        };

        result_a
    }

    fn update_matrices_a_b(
        &self,
        a: &mut Vec<Vec<f64>>,
        b: &mut Vec<f64>,
        flowsq: &Vec<f64>,
        deltaq: f64,
        n: f64,
    ) {
        let mut _intpart: f64 = 0.0;
        let mut _coef_a: f64 = 0.0;
        let mut _coef_b: f64 = 0.0;

        let (npip, npmp, nvlv) = self.link_sizes();

        if let Some(pipes) = &self.network.pipes {
            //update A & B matrices for pipes :
            for i in 0..npip {
                _intpart = flowsq[i].abs() / deltaq;

                #[cfg(feature = "deep_report")]
                {
                    println!("_intpart = {}", _intpart);
                }

                _coef_a = f64::trunc(_intpart) * deltaq;
                _coef_b = _coef_a + deltaq;

                //Updating A (eq13):
                // A(i,i) = R(i)*(b(i)^n-a(i)^n)/(b(i)-a(i));

                _intpart = (f64::powf(_coef_b, n) - f64::powf(_coef_a, n)) / (_coef_b - _coef_a);
                a[i][i] = pipes[i].get_r_of_q(flowsq[i]) * _intpart;

                //Updating B (eq14):

                //B(i) = sign(Q(i))*R(i)*((b(i)^n-a(i)^n)/(b(i)-a(i))*a(i)-a(i)^n);
                b[i] = -1.0
                    * f64::signum(flowsq[i])
                    * pipes[i].get_r_of_q(flowsq[i])
                    * ((_intpart * _coef_a) - f64::powf(_coef_a, n));

                // println!("P: {}, _intpart = {}, a = {}, b = {}, A = {}, B = {} ", i, _intpart, _coef_a, _coef_b, a[i][i], b[i]);
            }
        };

        //update A & B matrices for pumps :
        if let Some(pumps) = &self.network.pumps {
            //update A & B matrices for pipes :
            for i in 0..npmp {
                let x = pumps[i].alpha;
                let y = pumps[i].beta;
                let z = pumps[i].gamma;

                let k = i + npip;
                _intpart = flowsq[k].abs() / deltaq;

                #[cfg(feature = "deep_report")]
                {
                    println!("flows Q : {:?}", flowsq);
                    println!("pumps: _intpart = {}", _intpart);
                }

                _coef_a = f64::trunc(_intpart) * deltaq;
                _coef_b = _coef_a + deltaq;

                //Updating A (eq36):
                // A(i,i) = R(i)*(b(i)^n-a(i)^n)/(b(i)-a(i));

                _intpart = (f64::powf(_coef_b, n) - f64::powf(_coef_a, n)) / (_coef_b - _coef_a);
                a[k][k] = -1.0 * (x * _intpart + y);

                //Updating B (eq37):

                //B(i) = sign(Q(i))*R(i)*((b(i)^n-a(i)^n)/(b(i)-a(i))*a(i)-a(i)^n);
                b[k] = (x * (_intpart * _coef_a - _coef_a.powi(2))) - z;
            }
        }
        /*
                //println!("pump state {:?}, R = {}", network.pumps[0].state, network.pumps[0].get_rq(0.01));
                match &self.network.pumps {
                    None => {}
                    Some(pumps) => {
                        for i in 0..npmp {
                            _intpart = flowsq[i + npip] / deltaq;
                            _coef_a = f64::trunc(_intpart) * deltaq;
                            _coef_b = _coef_a + deltaq; // f64::trunc(_intpart + f64::signum(flowsq[i + npip])) * deltaq;

                            //Updating A (eq13):
                            _intpart =
                                (f64::powf(_coef_b, n) - f64::powf(_coef_a, n)) / (_coef_b - _coef_a);
                            a[i + npip][i + npip] = f64::signum(flowsq[i + npip])
                                * _intpart
                                * pumps[i].get_rq(flowsq[i + npip]);

                            //Updating B (eq14):
                            b[i + npip] = -1.0
                                * f64::signum(flowsq[i + npip])
                                * pumps[i].get_rq(flowsq[i + npip])
                                * ((_intpart * _coef_a) - f64::powf(_coef_a, n));
                        }
                    }
                };
        */
        //update A & B matrices for valves :

        let _k: usize = npip + npmp;

        match &self.network.valves {
            None => {}
            Some(valves) => {
                for i in 0..nvlv {
                    _intpart = flowsq[i + _k] / deltaq;
                    _coef_a = f64::trunc(_intpart) * deltaq;
                    _coef_b = f64::trunc(_intpart + f64::signum(flowsq[i + _k])) * deltaq;

                    //Updating A (eq13):
                    _intpart =
                        (f64::powf(_coef_b, n) - f64::powf(_coef_a, n)) / (_coef_b - _coef_a);
                    a[i + _k][i + _k] =
                        f64::signum(flowsq[i + _k]) * _intpart * valves[i].get_rq(flowsq[i + _k]);

                    //Updating B (eq14):
                    b[i + _k] = -1.0
                        * f64::signum(flowsq[i + _k])
                        * valves[i].get_rq(flowsq[i + _k])
                        * ((_intpart * _coef_a) - f64::powf(_coef_a, n));
                }
            }
        }
    }

    fn transpose(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let nr = matrix.len();
        let nc = matrix[0].len();
        let mut transposed = vec![vec![0.0f64; nr]; nc];

        for i in 0..nr {
            for j in 0..nc {
                transposed[j][i] = matrix[i][j];
            }
        }

        transposed
    }

    #[allow(dead_code)]
    fn print(matrix: &Vec<Vec<f64>>, msg: &str) {
        let nr = matrix.len();
        let nc = matrix[0].len();

        println!("---- {}", msg);
        for i in 0..nr {
            print!("[{},:]", i);
            for j in 0..nc {
                print!(" {}", matrix[i][j]);
            }
            println!(" ");
        }
    }
    #[allow(dead_code)]
    fn print_vector(vector: &[f64], msg: &str) {
        let nr = vector.len();
        println!("---- {}", msg);
        for i in 0..nr {
            print!("[{},:]", i);
            println!("  {}", vector[i]);
        }
    }
}
