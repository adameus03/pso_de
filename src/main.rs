#![feature(generic_arg_infer)]
#![allow(clippy::needless_return)]

const FN_SIZE: usize = 30;

use std::ops::AddAssign;

use clap::{Parser, Subcommand};
use particle_swarm::pso_de::WorldState;
use particle_swarm::de;
use std::f64::consts::TAU;

#[derive(Parser, Clone, Debug)]
struct Config {
	#[arg(long, value_delimiter = ',', num_args = 1.., required = true)]
	functions: Vec<String>,
	#[arg(long = "crossover")]
	crossover_possibility: f64,
	#[arg(long = "amplifier")]
	diff_weight: f64,
	#[arg(long = "diff-pop")]
	diff_population: usize,
	#[arg(long = "diff-iters")]
	differential_iterations: usize,
	#[arg(long = "try-count")]
	try_count: Option<usize>,
	#[command(subcommand)]
	command: Option<ComputationMode>,
}

#[derive(Subcommand, Clone, Debug)]
enum ComputationMode {
	DiffPart {
		#[arg(long)]
		particles: usize,
		#[arg(long = "part-iters")]
		particle_iterations: usize,
	},
}

struct BatchRunData {
	pub min_result: f64,
	pub max_result: f64,
	pub average: f64,
	pub run_count: u32,
}

impl BatchRunData {
	fn new() -> Self {
		return Self {
			min_result: f64::MAX,
			max_result: f64::MIN,
			average: 0.0,
			run_count: 0,
		};
	}
}

impl AddAssign for BatchRunData {
	fn add_assign(&mut self, other: Self) {
		if other.max_result > self.max_result {
			self.max_result = other.max_result;
		}
		if other.min_result < self.min_result {
			self.min_result = other.min_result;
		}
		let self_sum = self.average * self.run_count as f64;
		let other_sum = other.average * other.run_count as f64;
		self.run_count += other.run_count;
		self.average = (self_sum + other_sum) / self.run_count as f64;
	}
}

impl AddAssign<f64> for BatchRunData {
	fn add_assign(&mut self, rhs: f64) {
		if rhs > self.max_result {
			self.max_result = rhs;
		}
		if rhs < self.min_result {
			self.min_result = rhs;
		}
		let previous_sum = self.average * self.run_count as f64;
		self.run_count += 1;
		self.average = (previous_sum + rhs) / self.run_count as f64;
		
	}
}

///<test functions for DE>
#[no_mangle]
pub unsafe extern "C" fn sphere(input: de::Vector) -> f64 {
	let mut result = 0.0;
	for i in 0..input.num_dimensions as isize {
		unsafe {
			result += *input.coordinates.offset(i) * *input.coordinates.offset(i);
		}
	}
	return result;
}

#[no_mangle]
pub unsafe extern "C" fn shifted_sphere(input: de::Vector) -> f64 {
	let mut result = 0.0;
	for i in 0..input.num_dimensions as isize {
		let mut a;
		unsafe {
			a = *input.coordinates.offset(i);
		}
		// Substract i+1
		a -= (i + 1) as f64;
		result += a * a;
	}
	return result;
}

pub fn exp(x: f64) -> f64 {
	return x.exp();
}

pub fn cos(x: f64) -> f64 {
	return x.cos();
}

pub fn sqrt(x: f64) -> f64 {
	return x.sqrt();
}

#[no_mangle]
pub unsafe extern "C" fn ackley(input: de::Vector) -> f64 {
	let mut sum1 = 0.0;
	let mut sum2 = 0.0;
	for i in 0..input.num_dimensions as isize {
		unsafe {
			sum1 += *input.coordinates.offset(i) * *input.coordinates.offset(i);
			sum2 += (TAU * *input.coordinates.offset(i)).cos();
		}
	}
	let n = input.num_dimensions as f64;
	return -20.0 * exp(-0.2 * sqrt(sum1 / n)) - exp(sum2 / n) + 20.0 + exp(1.0);

}

#[no_mangle]
pub unsafe extern "C" fn rastrigin(input: de::Vector) -> f64 {
	let mut result = 0.0;
	for i in 0..input.num_dimensions as isize {
		unsafe {
			let  x = *input.coordinates.offset(i);
			result += x*x - 10.0 * cos(TAU*x) + 10.0;
		}
	}
	return result;
}

#[no_mangle]
pub unsafe extern "C" fn weierstrass(input: de::Vector) -> f64 {
	let a = 0.5 as f64;
	let b = 3.0 as f64;
	let k_max = 20;
	let mut double_sum = 0.0;
	let mut single_sum = 0.0;
	for i in 0..input.num_dimensions as isize {
		let  x = *input.coordinates.offset(i);
		let mut embedded_sum = 0.0;
		for k in 0..k_max as isize {
			embedded_sum += a.powi(k as i32) * cos(TAU * b.powi(k as i32) * (x+0.5));
		}
		double_sum += embedded_sum;
	}
	for k in 0..k_max as isize {
		single_sum += a.powi(k as i32) * cos(TAU * b.powi(k as i32) * 0.5);
	}
	return double_sum - (input.num_dimensions as f64) * single_sum;
}


// Quick&dirty test for de_minimum_stub and de_minimum
fn de_test() {
	println!("Hello from rust!");
	
	// [Test the DE library]
	let stop_condition = de::DeStopCondition {
		stype: de::DeStopType::StopAfterIters,
		union: de::DeLimitation { iters: 1000 }
	};
	let mut config = de::DeConfig {
		population_size: 1000,
		crossover_probability: 0.5,
		amplification_factor: 0.5,
		lambda: 0.5,
		stop_condition: stop_condition
	};
	// Use the holder function and its bounds (declared in src/functions.rs)
	let mut target = de::DeOptimizationTarget {
		//f: particle_swarm::functions::Holder::get_function(&self),
		f: /*shifted_sphere*//*ackley*/rastrigin/*weierstrass*/,
		num_dimensions: 30,
		/*left_bound::get_bounds().0[0],
		right_bound::get_bounds().1[0]*/
		left_bound: -10.0,
		right_bound: 10.0
	};
	// Call the DE library
	println!("Calling de_minimum_stub");
	let mut result = unsafe { de::de_minimum_stub(&mut target, &mut config) };
	println!("de_minimum_stub call returned.");
	// Print the de_minimum_stub result coordinates
	for i in 0..result.num_dimensions as isize {
		unsafe {
			println!("Result coordinate {}: {}", i, *result.coordinates.offset(i));
		}
	}
	
	// Free the result
	unsafe { de::de_vector_free_coordinates(&mut result) };

	println!("Calling de_minimum");
	result = unsafe { de::de_minimum(&mut target, &mut config) };
	println!("de_minimum call returned.");
	// Print the de_minimum result coordinates
	for i in 0..result.num_dimensions as isize {
		unsafe {
			println!("Result coordinate {}: {}", i, *result.coordinates.offset(i));
		}
	}

	unsafe {
		println!("Extreme function value: {}", rastrigin(result));
	}
	
	// Free the result
	unsafe { de::de_vector_free_coordinates(&mut result) };
	
	// [/Test the DE library]
}
///</test functions for DE>

fn main() {
	//de_test();
	//return;

	let builtin_fns = particle_swarm::functions::create_function_list::<FN_SIZE>();
	let config = Config::parse();
	if config.functions.is_empty() {
		panic!("No functions given");
	}
	let test_functions = config.functions.into_iter().map(|s| {
		return (s.clone(), builtin_fns.get(&s).unwrap());
	}).collect::<Vec<_>>();


	if let Some(tries) = config.try_count {
		for (function_name, function) in test_functions {
			let bounds = function.get_bounds();
			let tries_per_thread = tries.div_ceil(num_cpus::get());
			let mut threads = Vec::with_capacity(num_cpus::get());
			
			match config.command {
				Some(ComputationMode::DiffPart { particles, particle_iterations }) => {
					/*// hardcoded bounds because higher values are invalid
					let world = EvolvingParticles::new(config.diff_population, config.crossover_possibility, config.diff_weight, (0.0, 1.0), particles, function.get_function(), bounds, particle_iterations);
					for _ in 0..num_cpus::get() {
						let mut thread_world = world.clone();
						threads.push(std::thread::spawn(move || {
							let mut run_stats = BatchRunData::new();
							for _ in 0..tries_per_thread {
								thread_world.do_all_iterations(config.differential_iterations);
								run_stats += (world.particle_function)(thread_world.get_best_solution());
								thread_world.reset();
							}
							return run_stats;
						}));
					}*/
					for _ in 0..num_cpus::get() { //STUB
						threads.push(std::thread::spawn(move || {
							return BatchRunData::new();
						}));
					}
				}
				None => {
					/*let world = EvolvingFunction::new(config.diff_population, config.crossover_possibility, config.diff_weight, bounds, function.get_function());
					for _ in 0..num_cpus::get() {
						let mut thread_world = world.clone();
						threads.push(std::thread::spawn(move || {
							let mut run_stats = BatchRunData::new();
							for _ in 0..tries_per_thread {
								thread_world.do_all_iterations(config.differential_iterations);
								run_stats += thread_world.best_solution_value;
								thread_world.reset();
							}
							return run_stats;
						}));
					}*/
					threads.push(std::thread::spawn(move || { //STUB
						return BatchRunData::new();
					}));
				}
			}
			
			let result = threads.into_iter().map(|handle| handle.join().unwrap()).reduce(|mut a, b| {
				a += b;
				return a;
			}).unwrap();
			println!("{}: Finished {} runs. Max solution is {}. Average solution is {}. Min solution is {}.", function_name, result.run_count, result.max_result, result.average, result.min_result);
		}
	} else {
		let mut threads = Vec::new();
		for (function_name, function) in test_functions {
			let c_func = function.get_c_function();
			let bounds = function.get_bounds();
			match config.command {
				Some(ComputationMode::DiffPart { particles, particle_iterations }) => {
					/*threads.push(std::thread::spawn(move || {
						let mut world = EvolvingParticles::new(config.diff_population, config.crossover_possibility, config.diff_weight, (0.0, 1.0), particles, func, bounds, particle_iterations);
						world.do_all_iterations(config.differential_iterations);
						println!("{}: Found optimum at {:?} = {}", function_name, world.get_best_solution().coordinates, func(world.get_best_solution()));
					}));*/

					threads.push(std::thread::spawn(move || { //STUB
						
					}));
				}
				None => {
					/*threads.push(std::thread::spawn(move || {
						let mut world = EvolvingFunction::new(config.diff_population, config.crossover_possibility, config.diff_weight, bounds, func);
						world.do_all_iterations(config.differential_iterations);
						println!("{}: Found optimum at {:?} = {}", function_name, world.best_solution.coordinates, world.best_solution_value);
					}));*/

					threads.push(std::thread::spawn(move || {
						let stop_condition = de::DeStopCondition {
							stype: de::DeStopType::StopAfterIters,
							union: de::DeLimitation { iters: config.differential_iterations as u64 }
						};
						let mut config = de::DeConfig {
							population_size: config.diff_population as u32,
							crossover_probability: config.crossover_possibility,
							amplification_factor: config.diff_weight,
							lambda: 0.5,
							stop_condition: stop_condition
						};
						// Use the holder function and its bounds (declared in src/functions.rs)
						let mut target = de::DeOptimizationTarget {
							//f: particle_swarm::functions::Holder::get_function(&self),
							f: /*shifted_sphere*//*ackley*//*rastrigin*//*weierstrass*/c_func,
							num_dimensions: 30,
							/*left_bound::get_bounds().0[0],
							right_bound::get_bounds().1[0]*/
							left_bound: -10.0,
							right_bound: 10.0
						};
						println!("Calling de_minimum");
						let mut result = unsafe { de::de_minimum(&mut target, &mut config) };
						println!("de_minimum call returned.");
						// Print the de_minimum result coordinates
						for i in 0..result.num_dimensions as isize {
							unsafe {
								println!("Result coordinate {}: {}", i, *result.coordinates.offset(i));
							}
						}

						unsafe {
							println!("Extreme function value: {}", c_func(result));
						}
						
						// Free the result
						unsafe { de::de_vector_free_coordinates(&mut result) };
					}));
				}
			}
		}
		for thread in threads {
			thread.join().unwrap();
		}
	}//////////

	// OLD VectorN PSO CALLS
	/*if let Some(tries) = config.try_count {
		for (function_name, function) in test_functions {
			let bounds = function.get_bounds();
			let world = WorldState::new(config.particles, function.get_function(), bounds, config.social_coeff, config.cognitive_coeff, config.inertia_coeff);
			let tries_per_thread = tries.div_ceil(num_cpus::get());
			let mut threads = Vec::with_capacity(num_cpus::get());
			for _ in 0..num_cpus::get() {
				let mut thread_world = world.clone();
				threads.push(std::thread::spawn(move || {
					let mut run_stats = BatchRunData::new();
					for _ in 0..tries_per_thread {
						thread_world.do_all_iterations(config.particle_iterations);
						run_stats += (world.function)(thread_world.best_solution);
						thread_world.reset();
					}
					return run_stats;
				}));
			}
			let result = threads.into_iter().map(|handle| handle.join().unwrap()).reduce(|mut a, b| {
				a += b;
				return a;
			}).unwrap();
			println!("{}: Finished {} runs. Max solution is {}. Average solution is {}. Min solution is {}.", function_name, result.run_count, result.max_result, result.average, result.min_result);
		}
	} else {
		let mut threads = Vec::new();
		for (function_name, function) in test_functions {
			let func = function.get_function();
			let bounds = function.get_bounds();
			threads.push(std::thread::spawn(move || {
				let mut world = WorldState::new(config.particles, func, bounds, config.social_coeff, config.cognitive_coeff, config.inertia_coeff);
				world.do_all_iterations(config.particle_iterations);
				println!("{}: Found optimum at {:?} = {}", function_name, world.best_solution.coordinates, func(world.best_solution));
			}));
		}
		for thread in threads {
			thread.join().unwrap();
		}
	}*/
}
