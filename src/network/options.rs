///
/// Network and Analysis options
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Options {
    pub flow_unit: FlowUnits,
    pub headloss_formula: HeadlossFormula,
    pub viscosity: f64,
    pub trials: usize,
    pub accuracy: f64,
    pub unbalanced: Unbalanced,
    pub pattern: usize,
    pub demand_multiplier: f64,
    pub emitter_exponent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeadlossFormula {
    ///
    /// Hazen-Williams (H-W)
    ///
    Hw,

    ///
    /// Darcy-Weisbach (D-W)
    ///
    Dw,

    ///
    ///  Chezy-Manning (C-M)
    ///
    Cm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Unbalanced {
    StopIter,
    ContinueIter(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowUnits {
    ///
    /// cubic feet per second
    ///
    Cfs,

    ///
    /// gallons per minute
    ///
    Gpm,

    ///
    /// million gallons per day
    ///  
    Mgd,

    ///
    /// Imperial MGD
    ///  
    Imgd,

    ///
    /// acre-feet per day
    ///  
    Afd,

    ///
    /// liters per second
    Lps,

    ///
    /// liters per minute
    Lpm,

    ///
    /// million liters per day
    ///  
    Mld,

    ///
    /// cubic meters per second
    ///  
    Cms,

    ///
    /// cubic meters per hour
    ///  
    Cmh,

    ///
    /// cubic meters per day
    Cmd,
}

#[derive(Debug, Clone)]
pub struct OptionsBuilder {
    pub flow_unit: FlowUnits,
    pub headloss_formula: HeadlossFormula,
    pub viscosity: f64,
    pub trials: usize,
    pub accuracy: f64,
    pub unbalanced: Unbalanced,
    pub pattern: usize,
    pub demand_multiplier: f64,
    pub emitter_exponent: f64,
}

impl OptionsBuilder {
    pub fn new() -> Self {
        OptionsBuilder {
            flow_unit: FlowUnits::Lps,
            headloss_formula: HeadlossFormula::Hw,
            viscosity: 0.0000001f64,
            trials: 40,
            accuracy: 0.0001,
            unbalanced: Unbalanced::StopIter,
            pattern: 0,
            demand_multiplier: 1.0,
            emitter_exponent: 0.5,
        }
    }

    pub fn set_flow_unit(&mut self, flow_unit: FlowUnits) -> &mut Self {
        self.flow_unit = flow_unit;
        self
    }

    pub fn set_headlossformula(&mut self, headlossformula: HeadlossFormula) -> &mut Self {
        self.headloss_formula = headlossformula;
        self
    }

    pub fn build(&self) -> Options {
        Options {
            flow_unit: self.flow_unit.clone(),
            headloss_formula: self.headloss_formula.clone(),
            viscosity: 0.0000001f64,
            trials: 40,
            accuracy: 0.0001,
            unbalanced: Unbalanced::StopIter,
            pattern: 0,
            demand_multiplier: 1.0,
            emitter_exponent: 0.5,
        }
    }
}
