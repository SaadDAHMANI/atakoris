use crate::network::Network;
use serde::{Deserialize, Serialize};
use serde_json;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

///
/// FfiDto (FFI DTO) is a Foriegn function interface with Data Transfer Objects (using Json).
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FfiDto {}
impl FfiDto {
    pub fn convert_from_json(json_net_ptr: *const c_char) -> Network {
        let c_str = unsafe { CStr::from_ptr(json_net_ptr) };
        let network: Network = match c_str.to_str() {
            Err(eror) => {
                let mut err_net = Network::default();
                err_net.title = Some(eror.to_string());
                err_net
            }
            Ok(json_net) => {
                let net: Network = match serde_json::from_str(json_net) {
                    Err(_eror) => Network::default(),
                    Ok(net) => net,
                };
                net
            }
        };
        network
    }

    pub fn convert_to_json(network: &Network) -> *mut c_char {
        let json_str = serde_json::to_string(network).unwrap();
        CString::new(json_str).unwrap().into_raw()
    }

    /*
        pub fn solve(&self) -> &Self {

        }
    */
}

/// Junction definition
///

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JunctionDto {
    pub id: usize,
    pub elevation: f64,
    pub demand: f64,
    //  pub pattern: Option<usize>,
    //  pub name: Option<String>,
    // pub head: Option<f64>,
    // #[cfg(feature = "optimization")]
    // target_head: Option<f64>,
    // pub pressure: Option<f64>,
}

#[unsafe(no_mangle)]
pub extern "C" fn free_json_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

/// Analyse (solve) the given network using Data Transfer Object (DTO with Json) mode.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn analyse_network(json_net_ptr: *const c_char) -> *mut c_char {
    let mut netw: Network = FfiDto::convert_from_json(json_net_ptr);
    //--------------------------------------------------------------------
    println!("Rust: solve() fn, recived network : {:?}", netw.title);
    netw.title = Some(String::from("New title for the network"));

    if let Some(ref mut jnctns) = netw.junctions {
        if jnctns.len() > 0 {
            jnctns[0].elevation = 2123.65;
        }
    }
    //--------------------------------------------------------------------
    FfiDto::convert_to_json(&netw)
}
