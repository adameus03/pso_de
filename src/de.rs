use libc::c_double;

#[repr(C)]
#[derive(Debug)]
pub struct Vector {
    pub coordinates: *mut c_double,
    pub num_dimensions: u32
}

#[repr(C)]
pub enum DeStopType {
    StopAfterIters,
    StopWhenSatisfied
}

#[repr(C)]
pub union DeLimitation {
    pub iters: u64,
    pub accuracy: c_double
}

#[repr(C)]
pub struct DeStopCondition {
    pub stype: DeStopType,
    pub union: DeLimitation
}

#[repr(C)]
pub struct DeConfig {
    pub population_size: u32,
    pub crossover_probability: c_double,
    pub amplification_factor: c_double,
    pub lambda: c_double,
    pub stop_condition: DeStopCondition
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
   pub fn de_minimum_stub(pOptimizationTarget: *mut DeOptimizationTarget, pConfig: *mut DeConfig) -> Vector;
   pub fn de_vector_free_coordinates (pVector: *mut Vector);
}