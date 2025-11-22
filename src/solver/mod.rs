pub mod solver;
pub mod solver2;
//----------------------
pub use solver::Solver;
pub use solver2::Solver2;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SolverError {
    #[error("The network is empty!")]
    EmptyNetwork,

    #[error("The node [ID='{0}'] is not linked to network! ")]
    IsolatedNodeErr(usize),

    #[error("No links (pipes / pumps / valves) in the network!")]
    EmptyLinksError,

    #[error("No water source in the network!")]
    NoWaterSourceErr,

    #[error("Dividing by zero!")]
    DivideByZero,

    #[error("Matrix production error: Columns.left = '{0}', while, Columns.right = '{1}'.")]
    MatrixProductionError(usize, usize),

    #[error("The matrix is not squate! : '{0}'")]
    MatrixNotSquare(String),

    #[error("Undefined error!")]
    Other,
}
