pub mod graph;
pub mod network;
pub mod parsers;
pub mod solver;
//-------------- FFI using data transfer objects DTO (using Json)-------
pub mod ffi_dto;
//use data_transfer_objects::JunctionDto;
//use data_transfer_objects::NetworkDto;
// -------------------------
//-----------FFI (using C interop)-------

// pub mod ffi; //This FFI mode use potential unsafe code!
// --------------------------------------

pub use network::*;
pub use parsers::inpfileparser::InpFileParser;
pub use solver::Solver;

#[cfg(test)]
mod tests {
    //use super::*;
    //use crate::network::node::junction::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
