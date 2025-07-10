use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct NetworkC {
    title: *const c_char,
    junctions: *const JunctionC,
    junction_count: usize,
}

#[repr(C)]
pub struct JunctionC {
    id: usize,
    elevation: f64,
    demand: f64,
}

// Create a new network
#[unsafe(no_mangle)]
pub extern "C" fn analyse_network(
    title: *const c_char,
    junctions: *const JunctionC,
    junction_count: usize,
) -> *mut NetworkC {
    // create a networkk from sent data:

    let c_title = unsafe { CStr::from_ptr(title) }.to_owned().into_raw();

    let network = Box::new(NetworkC {
        title: c_title,
        junctions,
        junction_count,
    });

    //----------------------
    // network.title = String::from("Roua-Lilyane");

    // --------------------
    Box::into_raw(network)
}

// solve network using atakor engine
// fn solve(net: &mut NetworkC) {}

// Free network (drop)
#[unsafe(no_mangle)]
pub extern "C" fn free_network(network: *mut NetworkC) {
    if network.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(network));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_cstring(s: *const c_char) -> *mut c_char {
    if s.is_null() {
        return ptr::null_mut();
    }
    let c_string = unsafe { CString::from_raw(s as *mut c_char) };
    CString::into_raw(c_string)
}

// Free a CString
#[unsafe(no_mangle)]
pub extern "C" fn free_cstring(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe { drop(CString::from_raw(s)) }
}
