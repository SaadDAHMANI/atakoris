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

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::network::node::junction::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn junction_builder_test() {
        let j = Junction::new(1, 18.36f64, 18.49f64).set_name(&"j1").build();

        assert_eq!(j.id, 1);
        assert_eq!(j.demand, 18.49f64);
        assert_eq!(j.pressure, None);
        assert_eq!(j.elevation, 18.36f64);
    }
}
