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

use crate::{
    AFD_FACTOR, CMD_FACTOR, CMH_FACTOR, LPM_FACTOR, LPS_FACTOR, Network, network::FlowUnits,
    solver::SolverError,
};

#[derive(Debug)]
pub struct Solver2 {
    ///
    /// non-zero & strict positive m-value. Default value : m = 100, m includes in [10.0, 10.0^6].
    ///
    m: f64,
    n: f64,
    target_error: f64,
    flow_unit_multiplayer: f64,
    //---------------------------------------
    junction_count: usize,
    tank_count: usize,
    reservoir_count: usize,

    pipe_count: usize,
    pump_count: usize,
    valve_count: usize,
}

impl Solver2 {
    ///
    /// use this function to build new Solver
    ///
    /// objective_error : minimal error flow and head computation (stopping criterion). If None, the default value (objective_error = 0.001) will be used.
    ///
    pub fn new(m_parameter: Option<f64>, target_error: Option<f64>) -> Self {
        let obj_err: f64 = match target_error {
            None => 0.0001,
            Some(objerr) => f64::max(objerr, 0.00000000001),
        };

        let m_value = match m_parameter {
            None => 100.0,
            Some(m) => f64::min(m, 1000000.0).max(10.0),
        };

        Solver2 {
            m: m_value,
            n: 1.852f64,
            target_error: obj_err,
            junction_count: 0,
            tank_count: 0,
            reservoir_count: 0,
            pipe_count: 0,
            pump_count: 0,
            valve_count: 0,
            flow_unit_multiplayer: 1.0,
        }
    }

    ///
    /// Set non-zero & strict positive m-value. Default value : m = 100, m inculdes in [10.0, 10.0^6].
    ///
    pub fn set_m_parameter(&mut self, m_value: f64) {
        self.m = f64::max(m_value, 10.0).min(1000000.0);
    }

    ///
    /// Set non-zero & strict positive. Default value :  objective_error = 0.001.
    ///
    pub fn set_target_error(&mut self, err_value: f64) {
        self.target_error = f64::max(err_value, 0.0000000000001);
    }
    pub fn get_version(&self) -> &'static str {
        "0.1.3"
    }

    fn init_solver(&mut self, network: &Network) -> Result<(), SolverError> {
        self.junction_count = network.junctions.as_ref().map_or(0, |nodes| nodes.len());
        self.tank_count = network.tanks.as_ref().map_or(0, |nodes| nodes.len());
        self.reservoir_count = network.reservoirs.as_ref().map_or(0, |nodes| nodes.len());

        self.pipe_count = network.pipes.as_ref().map_or(0, |links| links.len());
        self.pump_count = network.pumps.as_ref().map_or(0, |links| links.len());
        self.valve_count = network.valves.as_ref().map_or(0, |links| links.len());
        let no = self.tank_count + self.reservoir_count;
        // number of pipes + pumps + valves
        let np = self.pipe_count + self.pump_count + self.valve_count;

        self.flow_unit_multiplayer = Solver2::conversion_2is_multiplayer(network);

        if no == 0 {
            return Err(SolverError::NoWaterSourceErr);
        };

        if np == 0 {
            return Err(SolverError::EmptyLinksError);
        };

        Ok(())
    }

    pub fn compute(&mut self, network: &mut Network) -> Result<AnalysisResult, SolverError> {
        let chronos = Instant::now();

        if let Err(net_err) = self.init_solver(network) {
            return Err(net_err);
        };

        let (a21, a10, h0, q) = self.get_network(&network);
        let nn = a21.len();
        let np = a21[0].len();

        // let npip : usize = self.pipes.len();
        // let npump : usize = self.pumps.len();
        // let nvlv : usize = self.valves.len();

        if nn < 2 {
            return Err(SolverError::EmptyNetwork);
        } // return Option::None;}
        if np < 1 {
            return Err(SolverError::EmptyLinksError);
        } //return Option::None;}

        let mut iter: usize = 0;
        let itermax: usize = 20;
        let mut final_err_q: f64 = f64::MAX;
        let mut final_err_h: f64 = f64::MAX;

        let mut _a: Vec<Vec<f64>> = self.initilize_a_matrix(&network); // = vec![vec![0.0f64; np]; np]; //A
        let mut _b = vec![0.0f64; np]; // B
        let mut _c = vec![0.0f64; np];
        let mut _flowsq = vec![0.0f64; np];
        let mut _previous_q = vec![0.0f64; np];
        let mut _headsh = vec![0.0f64; nn];
        let mut _previous_h = vec![0.0f64; nn];

        let mut _coef_a = vec![0.0f64; np]; // ai
        let mut _coef_b = vec![0.0f64; np]; //bi

        let _a12 = Self::transpose(&a21);

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
            //Updating A (eq13) & B (eq14):
            self.update_matrices_a_b(&network, &mut _a, &mut _b, &_flowsq, deltaq, self.n);

            // Step 2 : Compute V (eq) and C
            // Compute V:
            let inva = Solver2::invers_diagonal(&_a)?;

            let _v1 = Solver2::product(&a21, &inva)?;

            let _v = Solver2::product(&_v1, &_a12)?;

            //Compute C:
            let tmpc = Solver2::product2(&a10, &h0)?;

            for i in 0..np {
                _c[i] = (-1.0 * _b[i]) - tmpc[i];
            }

            // Step 3 : Compute H (eq.29)
            let invv = Solver2::inverse_matrix_jordan(&_v)?;

            let mut tmp = Solver2::product2(&_v1, &_c)?;

            for i in 0..nn {
                tmp[i] -= q[i];
            }

            _headsh = Solver2::product2(&invv, &tmp)?;

            // Step 4 : Compute flowws Q (eq30)
            let tmpql = Solver2::product2(&inva, &_c)?;

            let tmpqm = Solver2::product(&inva, &_a12)?;

            let tmpqr = Solver2::product2(&tmpqm, &_headsh)?;

            for i in 0..np {
                _flowsq[i] = tmpql[i] - tmpqr[i];
            }

            //Check convergence :
            let check_q_err = self.check_convergence(&_flowsq, &_previous_q);

            if check_q_err.0 {
                let check_h_err = self.check_convergence(&_headsh, &_previous_h);
                final_err_h = check_h_err.1;
                stoploop = true;
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
        }

        self.update_network(network, &_flowsq, &_headsh);
        let time_analysis = chronos.elapsed();
        let analysis_result = AnalysisResult::new(iter, final_err_q, final_err_h, time_analysis);
        Ok(analysis_result)
    }

    fn update_network(&self, network: &mut Network, flows_q: &[f64], heads_h: &[f64]) {
        // ========================   Solver2::copy_results(&mut network, &_headsh, &_flowsq); ===============

        if let Some(junctions) = &mut network.junctions {
            for i in 0..self.junction_count {
                junctions[i].head = Some(heads_h[i]);
            }
        };

        if let Some(pipes) = &mut network.pipes {
            for i in 0..self.pipe_count {
                pipes[i].flow = Some(flows_q[i] / self.flow_unit_multiplayer);
            }
        };

        let mut k: usize = self.pipe_count;

        if let Some(pumps) = &mut network.pumps {
            for i in 0..self.pump_count {
                pumps[i].flow = Some(flows_q[k] / self.flow_unit_multiplayer);
                k += 1;
            }
        };

        if let Some(valves) = &mut network.valves {
            for i in 0..self.valve_count {
                valves[i].flow = Some(flows_q[k] / self.flow_unit_multiplayer);
                k += 1;
            }
        };
        // ====================================================================
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

    ///
    /// Get network matrices
    ///
    fn get_network(&self, network: &Network) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
        let junction_count = self.junction_count;
        let tank_count = self.tank_count;
        let reservoir_count = self.reservoir_count;

        let pipe_count = self.pipe_count;
        let pump_count = self.pump_count;
        let valve_count = self.valve_count;
        let no = tank_count + reservoir_count;

        let pipes_pumps = pipe_count + pump_count;

        // number of pipes + pumps + valves
        let np = pipe_count + pump_count + valve_count;

        // =========================================================
        /*
        println!(
            "tanks: {}, reservoirs: {}, pipes: {}, pumps: {}, valves: {}",
            self.tank_count,
            self.reservoir_count,
            self.pipe_count,
            self.pump_count,
            self.valve_count
        );*/
        // =========================================================

        // nodal demand
        let mut q = vec![0.0f64; junction_count];
        //H0 : reservoirs + tanks
        let mut _h0 = vec![0.0f64; no];

        //Matrix A21
        let mut _a21 = vec![vec![0.0f64; np]; junction_count];

        if let Some(junctions) = network.junctions.as_ref() {
            // ----------------- Pipes ----------------------------
            if let Some(pipes) = network.pipes.as_ref() {
                for i in 0..junction_count {
                    // Junction - Pipes :
                    for j in 0..pipe_count {
                        if pipes[j].start == junctions[i].id {
                            _a21[i][j] = -1.0;
                        } else if pipes[j].end == junctions[i].id {
                            _a21[i][j] = 1.0;
                        }
                    }
                }
            };
            //------------------ Pumps -----------------------------
            if let Some(pumps) = network.pumps.as_ref() {
                for i in 0..junction_count {
                    // Pumps :
                    let mut j = 0;
                    for k in pipe_count..pipes_pumps {
                        if pumps[j].start == junctions[i].id {
                            _a21[i][k] = -1.0;
                        } else if pumps[j].end == junctions[i].id {
                            _a21[i][k] = 1.0;
                        }
                        j += 1;
                    }
                }
            };
            //---------------------Valves --------------------------
            if let Some(valves) = network.valves.as_ref() {
                for i in 0..junction_count {
                    // Valves :
                    let mut j: usize = 0;
                    for k in pipes_pumps..np {
                        if valves[j].start == junctions[i].id {
                            _a21[i][k] = -1.0;
                        } else if valves[j].end == junctions[i].id {
                            _a21[i][k] = 1.0;
                        }
                        j += 1;
                    }
                }
            }

            // -------------- Nodal demand --------------------------

            //nodal demand
            for i in 0..junction_count {
                q[i] = junctions[i].demand * self.flow_unit_multiplayer;
            }
            //
        };

        //Matrix A10
        let mut _a10 = vec![vec![0.0f64; no]; np];

        if let Some(tanks) = network.tanks.as_ref() {
            //Fixed head
            for k in 0..tank_count {
                _h0[k] = tanks[k].head();
            }

            let mut ki: usize;
            //-------------------tanks - pipes ----------------
            if let Some(pipes) = network.pipes.as_ref() {
                for j in 0..tank_count {
                    // Tanks -Pipes
                    for i in 0..pipe_count {
                        if pipes[i].start == tanks[j].id {
                            _a10[i][j] = -1.0;
                        } else if pipes[i].end == tanks[j].id {
                            _a10[i][j] = 1.0;
                        }
                    }
                }
            };

            // ---------------- tanks - pumps -----------------
            if let Some(pumps) = network.pumps.as_ref() {
                for j in 0..tank_count {
                    // Tanks - Pumps
                    for i in pipe_count..pipes_pumps {
                        ki = i - pipe_count;

                        if pumps[ki].start == tanks[j].id {
                            _a10[i][j] = -1.0;
                        } else if pumps[ki].end == tanks[j].id {
                            _a10[i][j] = 1.0;
                        };
                    }
                }
            };

            if let Some(valves) = network.valves.as_ref() {
                for j in 0..tank_count {
                    // Tanks - Valves

                    for i in pipes_pumps..np {
                        ki = i - pipes_pumps;

                        if valves[ki].start == tanks[j].id {
                            _a10[i][j] = -1.0;
                        } else if valves[ki].end == tanks[j].id {
                            _a10[i][j] = 1.0;
                        };
                    }
                }
            }
        }

        if let Some(reservoirs) = network.reservoirs.as_ref() {
            //
            //Fixed head
            for k in 0..reservoir_count {
                _h0[k + tank_count] = reservoirs[k].head;
            }

            let mut ki: usize;
            let mut kj: usize;

            //-----------------Pipes ------------------------
            if let Some(pipes) = network.pipes.as_ref() {
                for j in tank_count..no {
                    // Reservoirs - Pipes
                    kj = j - tank_count;
                    for i in 0..pipe_count {
                        if pipes[i].start == reservoirs[kj].id {
                            _a10[i][j] = -1.0;
                        } else if pipes[i].end == reservoirs[kj].id {
                            _a10[i][j] = 1.0;
                        };
                    }
                }
            };
            // ---------------- pumps ----------------------
            if let Some(pumps) = network.pumps.as_ref() {
                for j in tank_count..no {
                    kj = j - tank_count;
                    // Reservoirs - Pumps
                    for i in pipe_count..pipes_pumps {
                        ki = i - pipe_count;

                        if pumps[ki].start == reservoirs[kj].id {
                            _a10[i][j] = -1.0;
                        } else if pumps[ki].end == reservoirs[kj].id {
                            _a10[i][j] = 1.0;
                        };
                    }
                }
            };

            //------------------ valves ----------------------
            if let Some(valves) = network.valves.as_ref() {
                for j in tank_count..no {
                    kj = j - tank_count;
                    // Reservoirs - Valves

                    for i in pipes_pumps..np {
                        ki = i - pipes_pumps;

                        if valves[ki].start == reservoirs[kj].id {
                            _a10[i][j] = -1.0;
                        } else if valves[ki].end == reservoirs[kj].id {
                            _a10[i][j] = 1.0;
                        };
                    }
                }
            }
        }
        /*
                println!("A21: {:?}", _a21);
                println!("A10 : {:?}", _a10);
                println!("H0: {:?}", _h0);
                println!("q = {:?}", q);
        */
        (_a21, _a10, _h0, q)
    }

    fn check_convergence(&self, actual: &[f64], previous: &[f64]) -> (bool, f64) {
        let sum_err = actual
            .iter()
            .zip(previous.iter())
            .fold(0.0f64, |acc, (a, b)| acc + f64::abs(a - b));

        let sumq = actual.iter().fold(0.0f64, |acc, q| acc + q.abs());

        let computed_err = sum_err / sumq;

        if computed_err <= self.target_error {
            (true, computed_err)
        } else {
            (false, computed_err)
        }
    }

    fn inverse_matrix_jordan(matrix: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, SolverError> {
        let n = matrix.len();

        if matrix.len() == matrix[0].len() {
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
                    return Err(SolverError::DivideByZero);
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
        } else {
            Err(SolverError::MatrixNotSquare(format!(
                "Jordan inverse [V] matrix."
            )))
        }
    }

    fn product(left: &Vec<Vec<f64>>, right: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, SolverError> {
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
            Err(SolverError::MatrixProductionError(pl, pr))
        }
    }

    fn product2(left: &Vec<Vec<f64>>, right: &Vec<f64>) -> Result<Vec<f64>, SolverError> {
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
            Err(SolverError::MatrixProductionError(pl, pr))
        }
    }

    fn invers_diagonal(matrix: &Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, SolverError> {
        let mlen = matrix.len();

        let mut invers = vec![vec![0.0f64; mlen]; mlen];

        for i in 0..matrix.len() {
            if matrix[i][i] == 0.0 {
                return Err(SolverError::DivideByZero);
            }
            invers[i][i] = 1.0 / matrix[i][i];
        }

        Ok(invers)
    }

    fn initilize_a_matrix(&self, network: &Network) -> Vec<Vec<f64>> {
        let npip = self.pipe_count;
        let npmp = self.pump_count;
        let nvlv = self.valve_count;

        let np = npip + npmp + nvlv;

        let mut result_a = vec![vec![0.0f64; np]; np];

        //let np = npip+npmp;
        let rspipes = network.get_pipes_resistances().unwrap();

        let qmax = match &network.junctions {
            None => 0.0f64,
            Some(junctions) => junctions.iter().fold(0.0f64, |acc, j| acc + j.demand),
        };

        // Pipes resistances
        for i in 0..npip {
            result_a[i][i] = rspipes[i] * qmax;
        }

        // Pumps resistances
        match &network.pumps {
            None => {}
            Some(_pumps) => {
                for i in 0..npmp {
                    // result_a[i+npip][i+npip]= network.pumps[i].alpha*qmax + network.pumps[i].beta + network.pumps[i].gamma/qmax;
                    result_a[i + npip][i + npip] = 1.0; //  pumps[i].get_r_of_q(qmax, self.flow_unit_multiplayer);
                }
            }
        };

        // Valves resistances
        match &network.valves {
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
        network: &Network,
        a: &mut Vec<Vec<f64>>,
        b: &mut Vec<f64>,
        flowsq: &Vec<f64>,
        deltaq: f64,
        n: f64,
    ) {
        let mut _intpart: f64 = 0.0;
        let mut _coef_a: f64 = 0.0;
        let mut _coef_b: f64 = 0.0;

        //  let (npip, npmp, nvlv) = Solver2::links_cont(network);
        let npip = self.pipe_count;
        let npmp = self.pump_count;
        let nvlv = self.valve_count;

        if let Some(pipes) = &network.pipes {
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
        if let Some(pumps) = &network.pumps {
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
        let _k: usize = npip + npmp;

        match &network.valves {
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

impl Default for Solver2 {
    fn default() -> Self {
        Solver2 {
            m: 100.0,
            n: 1.852f64,
            target_error: 0.0001,
            junction_count: 0,
            tank_count: 0,
            reservoir_count: 0,
            pipe_count: 0,
            pump_count: 0,
            valve_count: 0,
            flow_unit_multiplayer: 1.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct AnalysisResult {
    pub iterations: usize,
    pub final_flow_error: f64,
    pub final_head_error: f64,
    pub time_analysis: Duration,
}

impl AnalysisResult {
    pub fn new(
        iterations: usize,
        final_flow_error: f64,
        final_head_error: f64,
        time_analysis: Duration,
    ) -> Self {
        Self {
            iterations,
            final_flow_error,
            final_head_error,
            time_analysis,
        }
    }
}

//#[allow(dead_code)]
#[derive(Debug)]
pub struct AsyncSolver2 {
    pub net: Network,
    pub solver_rx: std::sync::mpsc::Receiver<(Network, Result<AnalysisResult, SolverError>)>,
}
impl AsyncSolver2 {
    pub fn start_solver(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.solver_rx = rx;

        let mut net_copy = self.net.clone();

        std::thread::spawn(move || {
            let mut solver = Solver2::default();
            let result = solver.compute(&mut net_copy);
            tx.send((net_copy, result)).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn solver2_test1() {}
}
