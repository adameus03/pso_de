use libc::c_double;

#[repr(C)]
#[derive(Debug)]
pub struct Vector {
    pub coordinates: *mut c_double,
    pub num_dimensions: u32
}

#[repr(C)]
pub struct DeConfig {
    pub population_size: u32
}

#[repr(C)]
pub struct DeOptimizationTarget {
    pub f: unsafe extern "C" fn(Vector) -> c_double,
    pub num_dimensions: u32,
    pub left_bound: c_double,
    pub right_bound: c_double
}

#[link(name = "differential_evolution")]
extern "C" {
   pub fn de_minimum(pOptimizationTarget: *mut DeOptimizationTarget, pConfig: *mut DeConfig) -> Vector;
   pub fn vector_free_coordinates (pVector: *mut Vector);
}